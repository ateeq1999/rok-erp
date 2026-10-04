//! One command installs every module a database needs, and can take them back
//! off again.
//!
//! The installer is ours rather than `Db::migrate` because sqlx keeps one
//! version list per database, while modules reuse numbers like `0001`: core,
//! catalog and pharmacy all have a `0001`. So progress is kept per module, in
//! `core.applied_migrations`, with each migration's checksum beside it.
//!
//! For every module, in dependency order, it:
//!
//! 1. opens a transaction,
//! 2. runs each `NNNN_*.up.sql` that has not run yet, in file order,
//! 3. records the migration, its SHA-256 checksum and how long it took,
//! 4. records the module in `core.installed_modules` and `core.module_dependencies`,
//! 5. upserts the module's `[[permissions]]` into `core.permissions`,
//! 6. writes what it is doing to `core.module_jobs.steps`.
//!
//! A migration that shipped is never edited: if the file's checksum no longer
//! matches the one recorded, [`install`] stops with
//! [`ModuleError::ChecksumChanged`] and says to add a new migration instead.
//! Running it twice changes nothing the second time.
//!
//! ```
//! use rok_pos_database::module_installer::checksum;
//!
//! // A migration is checksummed over its text with line endings normalised, so a
//! // Windows checkout and a Linux one agree on what the file says.
//! assert_eq!(checksum("select 1;\r\n"), checksum("select 1;\n"));
//! ```

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rok_db::{Db, Tx, raw};
use sha2::{Digest, Sha256};
use sqlx::Row;

use crate::module_manifest::{
    ModuleError, ModuleSource, Permission, discover, in_dependency_order,
};

/// One migration file, and the pair it belongs to.
#[derive(Debug, Clone)]
pub struct MigrationFile {
    /// The name without extension or direction, such as `0001_create_core`.
    pub name: String,
    /// The file that runs it.
    pub up: PathBuf,
    /// The file that takes it back off.
    pub down: PathBuf,
}

/// What to install.
#[derive(Debug, Clone)]
pub struct InstallRequest {
    /// The folder holding one subfolder per module.
    pub modules_root: PathBuf,
    /// Module keys to install, or empty for every module found.
    pub only: Vec<String>,
    /// Whether to write progress to `core.module_jobs`.
    pub record_jobs: bool,
}

impl InstallRequest {
    /// Install every module under `modules_root`.
    #[must_use]
    pub fn all(modules_root: impl Into<PathBuf>) -> Self {
        Self {
            modules_root: modules_root.into(),
            only: Vec::new(),
            record_jobs: true,
        }
    }

    /// Install only these modules, and whatever they depend on.
    #[must_use]
    pub fn only(mut self, keys: &[&str]) -> Self {
        self.only = keys.iter().map(|key| (*key).to_string()).collect();
        self
    }

    /// Install without writing job rows, for the verifier and for tests that
    /// only care about the schema.
    #[must_use]
    pub fn without_jobs(mut self) -> Self {
        self.record_jobs = false;
        self
    }
}

/// What an install did.
#[derive(Debug, Clone, Default)]
pub struct InstallReport {
    /// The modules, in the order they were installed.
    pub modules: Vec<String>,
    /// The migrations that ran, as `module/migration`.
    pub migrations_applied: Vec<String>,
    /// The permissions written, as keys.
    pub permissions_registered: Vec<String>,
    /// One entry per `core.module_jobs` row, in order.
    pub jobs: Vec<ModuleJob>,
}

impl InstallReport {
    /// `true` when the database already had everything this install would write.
    #[must_use]
    pub fn is_noop(&self) -> bool {
        self.migrations_applied.is_empty() && self.permissions_registered.is_empty()
    }
}

/// One `core.module_jobs` row: what happened to one module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleJob {
    /// The module it was for.
    pub module_key: String,
    /// The steps, in the order they ran.
    pub steps: Vec<JobStep>,
}

/// One line of progress in `core.module_jobs.steps`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JobStep {
    /// What the step is, such as `migrations` or `permissions`.
    pub key: String,
    /// What it says on screen.
    pub label: String,
    /// `running`, `succeeded` or `failed`.
    pub status: String,
    /// The migrations or permission keys it covered.
    pub details: Vec<String>,
}

impl JobStep {
    fn new(key: &str, label: &str) -> Self {
        Self {
            key: key.to_string(),
            label: label.to_string(),
            status: "running".to_string(),
            details: Vec::new(),
        }
    }

    fn finished(mut self, status: &str) -> Self {
        self.status = status.to_string();
        self
    }
}

/// The SHA-256 of a migration, over its text with line endings normalised so the
/// checksum does not depend on the checkout.
#[must_use]
pub fn checksum(sql: &str) -> String {
    let normalised = sql.replace("\r\n", "\n");
    format!("{:x}", Sha256::digest(normalised.as_bytes()))
}

/// Install every module the request asks for, in dependency order.
///
/// # Errors
///
/// Returns [`ModuleError`] when a manifest cannot be read, the dependency order
/// cannot be resolved, a recorded migration has changed or gone missing, or the
/// database refuses a migration. Nothing is left half installed: each migration
/// runs in a transaction of its own, and a failed one is rolled back.
pub async fn install(db: &Db, request: &InstallRequest) -> Result<InstallReport, ModuleError> {
    let discovered = discover(&request.modules_root)?;
    let ordered = select(&discovered, &request.only)?;
    let mut report = InstallReport::default();

    for source in ordered {
        report.modules.push(source.key().to_string());
        let job_id = start_job(db, source, request.record_jobs).await?;
        let steps = match install_module(db, source, &mut report).await {
            Ok(steps) => steps,
            Err(error) => {
                let _ = fail_job(db, job_id, &error.to_string()).await;
                return Err(error);
            }
        };
        if let Some(id) = job_id {
            finish_job(db, id, &steps).await?;
        }
        report.jobs.push(ModuleJob {
            module_key: source.key().to_string(),
            steps,
        });
    }

    Ok(report)
}

/// Install one module: its pending migrations, then the rows that describe it.
async fn install_module(
    db: &Db,
    source: &ModuleSource,
    report: &mut InstallReport,
) -> Result<Vec<JobStep>, ModuleError> {
    let migrations = migrations_of(source)?;
    let mut step = JobStep::new("migrations", "Apply migrations");

    // A migration and the row recording it commit together, so no crash can
    // leave one without the other. Core's first migrations run before the
    // registry exists, so they share one transaction until the migration that
    // creates it, and are recorded there: a failure before that point takes
    // them all back.
    let mut open: Option<Tx> = None;
    let mut unrecorded: Vec<AppliedMigration> = Vec::new();

    for migration in &migrations {
        let sql = read_sql(&migration.up)?;
        let found = checksum(&sql);
        match recorded_checksum(db, source.key(), &migration.name).await? {
            Some(recorded) if recorded == found => continue,
            Some(recorded) => {
                return Err(ModuleError::ChecksumChanged {
                    module: source.key().to_string(),
                    migration: migration.name.clone(),
                    recorded,
                    found,
                });
            }
            None => {}
        }

        let mut transaction = match open.take() {
            Some(transaction) => transaction,
            None => db.begin().await?,
        };
        let started = Instant::now();
        if let Err(error) = execute_script(&mut transaction, &sql).await {
            let _ = transaction.rollback().await;
            return Err(ModuleError::Database(rok_db::Error::Database(error)));
        }
        unrecorded.push(AppliedMigration {
            name: migration.name.clone(),
            checksum: found,
            duration_milliseconds: i64::try_from(started.elapsed().as_millis())
                .unwrap_or(i64::MAX),
        });

        if registry_exists(&mut *transaction).await? {
            record_migrations(&mut transaction, source.key(), &unrecorded).await?;
            unrecorded.clear();
            transaction.commit().await?;
        } else {
            open = Some(transaction);
        }

        step.details.push(migration.name.clone());
        report
            .migrations_applied
            .push(format!("{}/{}", source.key(), migration.name));
    }
    // A module folder with no registry anywhere, such as a test schema, still
    // keeps what it ran.
    if let Some(transaction) = open {
        transaction.commit().await?;
    }
    let mut steps = vec![step.clone().finished("succeeded")];

    // `core` creates the registry tables in its own third migration, so a
    // module's rows are only written once the tables they belong in exist.
    if !registry_exists(db).await? {
        return Ok(steps);
    }

    record_module(db, source).await?;

    let mut permissions = JobStep::new("permissions", "Register permissions");
    permissions.details = source
        .manifest
        .permissions
        .iter()
        .map(|permission| permission.key.clone())
        .collect();
    let changed = register_permissions(db, source.key(), &source.manifest.permissions).await?;
    // A second install rewrites the same rows, so only report what moved.
    report.permissions_registered.extend(
        permissions
            .details
            .iter()
            .filter(|key| changed.contains(key))
            .cloned(),
    );
    steps.push(permissions.finished("succeeded"));

    Ok(steps)
}

/// Run every module's down migration in reverse dependency order, leaving the
/// database as it was.
///
/// This is how the migration verifier proves a schema can be built and unbuilt.
/// It is not how a business loses a module: uninstalling keeps the data and
/// archives the schema.
///
/// # Errors
///
/// Returns [`ModuleError`] when a manifest cannot be read, or a down migration
/// fails.
pub async fn revert_all(db: &Db, modules_root: &Path) -> Result<Vec<String>, ModuleError> {
    let discovered = discover(modules_root)?;
    let mut ordered = in_dependency_order(&discovered)?;
    ordered.reverse();

    // Read what has run before anything comes down: core's own third migration
    // drops the registry, so it cannot be asked halfway through.
    let applied = applied_migrations(db).await?;
    let mut taken_off = Vec::new();

    for source in ordered {
        let mut migrations = migrations_of(source)?;
        migrations.reverse();
        for migration in &migrations {
            if !applied.contains(&(source.key().to_string(), migration.name.clone())) {
                continue;
            }
            let sql = read_sql(&migration.down)?;
            run_migration(db, &sql).await?;
            taken_off.push(format!("{}/{}", source.key(), migration.name));
        }
        forget_module(db, source.key()).await?;
    }

    Ok(taken_off)
}

/// Every migration the registry says has run, as `(module, migration)`.
async fn applied_migrations(db: &Db) -> Result<Vec<(String, String)>, ModuleError> {
    if !registry_exists(db).await? {
        return Ok(Vec::new());
    }
    let rows: Vec<(String, String)> =
        raw("select module_key, migration_name from core.applied_migrations")
            .fetch_all(db)
            .await?;
    Ok(rows.into_iter().map(|row| (row.0, row.1)).collect())
}

/// One migration that ran, as it is recorded in `core.applied_migrations`.
struct AppliedMigration {
    name: String,
    checksum: String,
    duration_milliseconds: i64,
}

/// Every migration in a module folder, in the order they must run.
fn migrations_of(source: &ModuleSource) -> Result<Vec<MigrationFile>, ModuleError> {
    let directory = source.migrations_directory();
    let entries = match std::fs::read_dir(&directory) {
        Ok(entries) => entries,
        // A module may be a manifest with nothing to run yet.
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(source) => {
            return Err(ModuleError::Io {
                path: directory,
                source,
            });
        }
    };

    let mut ups: BTreeMap<String, PathBuf> = BTreeMap::new();
    for entry in entries {
        let path = entry
            .map_err(|source| ModuleError::Io {
                path: directory.clone(),
                source,
            })?
            .path();
        let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(stem) = name.strip_suffix(".up.sql") else {
            continue;
        };
        ups.insert(stem.to_string(), path);
    }

    let mut migrations = Vec::with_capacity(ups.len());
    for (name, up) in ups {
        let down = up.with_file_name(format!("{name}.down.sql"));
        if !down.is_file() {
            return Err(ModuleError::MissingMigration {
                module: source.key().to_string(),
                migration: name,
            });
        }
        migrations.push(MigrationFile { name, up, down });
    }
    Ok(migrations)
}

fn read_sql(path: &Path) -> Result<String, ModuleError> {
    std::fs::read_to_string(path).map_err(|source| ModuleError::Io {
        path: path.to_path_buf(),
        source,
    })
}

/// The modules the request asks for, dependencies included, in order.
fn select<'a>(
    sources: &'a [ModuleSource],
    only: &[String],
) -> Result<Vec<&'a ModuleSource>, ModuleError> {
    let ordered = in_dependency_order(sources)?;
    if only.is_empty() {
        return Ok(ordered);
    }

    // Walk the dependencies all the way down: a module's dependency has
    // dependencies of its own, which the manifest does not repeat.
    let mut wanted: Vec<&str> = Vec::new();
    let mut to_visit: Vec<&str> = only.iter().map(String::as_str).collect();
    while let Some(key) = to_visit.pop() {
        if wanted.contains(&key) {
            continue;
        }
        let source = ordered
            .iter()
            .find(|source| source.key() == key)
            .ok_or_else(|| ModuleError::UnknownDependency {
                module: "<request>".to_string(),
                depends_on: key.to_string(),
            })?;
        wanted.push(source.key());
        to_visit.extend(source.manifest.module.depends_on.iter().map(String::as_str));
    }

    Ok(ordered
        .into_iter()
        .filter(|source| wanted.contains(&source.key()))
        .collect())
}

/// `true` once `core` has created the tables the installer records itself in.
async fn registry_exists<'e, E: rok_db::Executor<'e>>(executor: E) -> Result<bool, ModuleError> {
    let exists: bool = raw("select to_regclass('core.applied_migrations') is not null \
         and to_regclass('core.permissions') is not null")
    .scalar(executor)
    .await?;
    Ok(exists)
}

async fn recorded_checksum(
    db: &Db,
    module: &str,
    migration: &str,
) -> Result<Option<String>, ModuleError> {
    if !registry_exists(db).await? {
        return Ok(None);
    }
    let row = sqlx::query(
        "select checksum from core.applied_migrations where module_key = $1 and migration_name = $2",
    )
    .bind(module)
    .bind(migration)
    .fetch_optional(db)
    .await?;
    Ok(row.map(|row| row.get::<String, _>("checksum")))
}

/// Run one migration file in a transaction of its own, so a failure leaves the
/// schema as it was.
async fn run_migration(db: &Db, sql: &str) -> Result<(), ModuleError> {
    let mut transaction = db.begin().await?;
    if let Err(error) = execute_script(&mut transaction, sql).await {
        let _ = transaction.rollback().await;
        return Err(ModuleError::Database(rok_db::Error::Database(error)));
    }
    transaction.commit().await?;
    Ok(())
}

/// A migration is a script of many statements, which sqlx sends with the simple
/// query protocol.
async fn execute_script(transaction: &mut Tx, sql: &str) -> Result<(), sqlx::Error> {
    sqlx::raw_sql(sql)
        .execute(&mut **transaction.inner())
        .await?;
    Ok(())
}

/// Record migrations inside the transaction that ran them.
async fn record_migrations(
    transaction: &mut Tx,
    module: &str,
    applied: &[AppliedMigration],
) -> Result<(), ModuleError> {
    for migration in applied {
        raw("insert into core.applied_migrations \
             (module_key, migration_name, checksum, duration_milliseconds) values (?, ?, ?, ?)")
        .bind(module)
        .bind(&migration.name)
        .bind(&migration.checksum)
        .bind(migration.duration_milliseconds)
        .execute(&mut **transaction)
        .await?;
    }
    Ok(())
}

/// Record the module itself, with the manifest it was installed from.
async fn record_module(db: &Db, source: &ModuleSource) -> Result<(), ModuleError> {
    let manifest =
        serde_json::to_value(&source.manifest).map_err(|error| ModuleError::InvalidManifest {
            module: source.key().to_string(),
            message: format!("the manifest cannot be stored as json: {error}"),
        })?;

    raw(
        "insert into core.installed_modules (module_key, display_name, installed_version, schema_name, manifest) \
         values (?, ?, ?, ?, ?) \
         on conflict (module_key) do update set display_name = excluded.display_name, \
           installed_version = excluded.installed_version, schema_name = excluded.schema_name, \
           manifest = excluded.manifest",
    )
    .bind(source.key())
    .bind(&source.manifest.module.name)
    .bind(&source.manifest.module.version)
    .bind(&source.manifest.module.schema)
    .bind(manifest)
    .execute(db)
    .await?;

    raw("delete from core.module_dependencies where module_key = ?")
        .bind(source.key())
        .execute(db)
        .await?;
    for (depends_on, is_optional) in source
        .manifest
        .module
        .depends_on
        .iter()
        .map(|key| (key, false))
        .chain(
            source
                .manifest
                .module
                .optional_depends_on
                .iter()
                .map(|key| (key, true)),
        )
    {
        raw(
            "insert into core.module_dependencies (module_key, depends_on_module_key, is_optional) \
             values (?, ?, ?)",
        )
        .bind(source.key())
        .bind(depends_on)
        .bind(is_optional)
        .execute(db)
        .await?;
    }

    Ok(())
}

/// Write the module's permissions, and say which rows actually changed.
///
/// The upsert is skipped where nothing differs, so installing twice reports
/// nothing the second time instead of pretending it wrote 146 rows.
async fn register_permissions(
    db: &Db,
    module: &str,
    permissions: &[Permission],
) -> Result<Vec<String>, ModuleError> {
    let mut changed = Vec::new();
    for permission in permissions {
        let rows = raw(
            "insert into core.permissions (key, module_key, description, risk_level) \
             values (?, ?, ?, ?) \
             on conflict (key) do update set module_key = excluded.module_key, \
               description = excluded.description, risk_level = excluded.risk_level \
             where (core.permissions.module_key, core.permissions.description, core.permissions.risk_level) \
               is distinct from (excluded.module_key, excluded.description, excluded.risk_level)",
        )
        .bind(&permission.key)
        .bind(module)
        .bind(&permission.description)
        .bind(permission.risk_level.as_str())
        .execute(db)
        .await?;
        if rows > 0 {
            changed.push(permission.key.clone());
        }
    }
    Ok(changed)
}

/// Take a module's rows out of the registry, once its schema is gone.
///
/// Core's own down migrations drop these tables, so by the time core rolls back
/// there is nothing left to delete.
async fn forget_module(db: &Db, module: &str) -> Result<(), ModuleError> {
    if !registry_exists(db).await? {
        return Ok(());
    }
    raw("delete from core.applied_migrations where module_key = ?")
        .bind(module)
        .execute(db)
        .await?;
    raw("delete from core.permissions where module_key = ?")
        .bind(module)
        .execute(db)
        .await?;
    raw("delete from core.module_dependencies where module_key = ?")
        .bind(module)
        .execute(db)
        .await?;
    raw("delete from core.installed_modules where module_key = ?")
        .bind(module)
        .execute(db)
        .await?;
    Ok(())
}

async fn start_job(
    db: &Db,
    source: &ModuleSource,
    record_jobs: bool,
) -> Result<Option<uuid::Uuid>, ModuleError> {
    if !record_jobs || !registry_exists(db).await? {
        return Ok(None);
    }
    let id = uuid::Uuid::now_v7();
    raw(
        "insert into core.module_jobs (id, module_key, job_type, target_version, status, started_at) \
         values (?, ?, 'install', ?, 'running', now())",
    )
    .bind(id)
    .bind(source.key())
    .bind(&source.manifest.module.version)
    .execute(db)
    .await?;
    Ok(Some(id))
}

async fn finish_job(db: &Db, id: uuid::Uuid, steps: &[JobStep]) -> Result<(), ModuleError> {
    raw(
        "update core.module_jobs set steps = ?::jsonb, status = 'succeeded', finished_at = now() \
         where id = ?",
    )
    .bind(steps_json(steps))
    .bind(id)
    .execute(db)
    .await?;
    Ok(())
}

async fn fail_job(db: &Db, id: Option<uuid::Uuid>, message: &str) -> Result<(), ModuleError> {
    let Some(id) = id else {
        return Ok(());
    };
    raw(
        "update core.module_jobs set status = 'failed', error_message = ?, finished_at = now() \
         where id = ?",
    )
    .bind(message)
    .bind(id)
    .execute(db)
    .await?;
    Ok(())
}

/// `core.module_jobs.steps` holds one JSON array, as documented in core's third
/// migration.
fn steps_json(steps: &[JobStep]) -> String {
    let items: Vec<serde_json::Value> = steps
        .iter()
        .map(|step| {
            serde_json::json!({
                "key": step.key,
                "label": step.label,
                "status": step.status,
                "details": step.details,
            })
        })
        .collect();
    serde_json::to_string(&items).unwrap_or_else(|_| "[]".to_string())
}
