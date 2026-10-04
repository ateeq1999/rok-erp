//! The module installer: read every `module.toml`, order the modules by their
//! dependencies and apply their migrations, once, safely and repeatably.
//!
//! This is our own installer rather than `Db::migrate`. sqlx's migrator keeps
//! one version list per database, and module migrations reuse numbers such as
//! `0001`, so the versions have to live per module. They do, in
//! `core.applied_migrations`, keyed by `(module_key, migration_name)` with a
//! SHA-256 checksum of the file that was applied.
//!
//! The install goes in two phases. First every shipped migration is checked
//! against what the database recorded, so a tampered or edited migration is
//! refused before any DDL runs. Then the modules are applied in dependency
//! order, each migration in its own transaction, writing progress to
//! `core.module_jobs.steps` as it goes.
//!
//! One bootstrapping wrinkle: the registry tables are created by core's own
//! migration 0003, so on an empty database core's first migrations run before
//! there is anywhere to record them. They are recorded as soon as the
//! registry appears.
//!
//! ```no_run
//! # async fn run() -> Result<(), rok_pos_database::InstallError> {
//! use rok_db::Db;
//! use rok_pos_database::{ModuleInstaller, default_modules_directory};
//!
//! let database = Db::connect("postgres://postgres@localhost/app").await?;
//! let directory = default_modules_directory();
//! let installer = ModuleInstaller::load(&directory)?;
//! let report = installer.install(&database).await?;
//! assert_eq!(report.modules.len(), 8);
//! # Ok(()) }
//! ```

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};
use std::time::Instant;

use rok_db::{Db, raw};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use uuid::Uuid;

/// The risk levels `core.permissions` accepts.
const RISK_LEVELS: [&str; 3] = ["normal", "sensitive", "money"];

/// The `[module]` section of a `module.toml` manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModuleSection {
    /// The key other modules depend on, and the schema this module owns.
    pub key: String,
    /// The display name shown on install screens.
    pub name: String,
    /// The manifest version, for example `1.0.0`.
    pub version: String,
    /// The PostgreSQL schema the module's tables live in.
    pub schema: String,
    /// Modules that must be installed before this one.
    #[serde(default)]
    pub depends_on: Vec<String>,
    /// Modules used when they are present, never required.
    #[serde(default)]
    pub optional_depends_on: Vec<String>,
}

/// One `[[permissions]]` entry, upserted into `core.permissions` on install.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PermissionEntry {
    /// The permission key, for example `pharmacy.clinical_check.approve`.
    pub key: String,
    /// What the permission allows, shown when a role is edited.
    pub description: String,
    /// `normal`, `sensitive` or `money`.
    #[serde(default = "normal_risk_level")]
    pub risk_level: String,
}

/// The default risk level when a manifest leaves it out.
#[must_use]
pub fn normal_risk_level() -> String {
    "normal".to_string()
}

/// The parts of a `module.toml` the installer reads. The other sections
/// (`pages`, `settings`, `number_sequences`, `assistant_rules`,
/// `default_roles`) are kept as parsed TOML in [`LoadedModule::document`]
/// and are activated for a business in a later phase.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    /// The `[module]` identity and dependencies.
    pub module: ModuleSection,
    /// The `[[permissions]]` this module registers.
    #[serde(default)]
    pub permissions: Vec<PermissionEntry>,
}

/// One `NNNN_name.up.sql` file, its matching `.down.sql` and the SHA-256
/// checksum of the up file as it ships.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MigrationFile {
    /// The module the migration belongs to.
    pub module_key: String,
    /// The file name without `.up.sql`, for example `0001_create_core_schema_and_helpers`.
    pub name: String,
    /// The number parsed from the start of [`MigrationFile::name`].
    pub number: u32,
    /// Where the up migration was read from.
    pub up_path: PathBuf,
    /// The matching down migration, when the module ships one.
    pub down_path: Option<PathBuf>,
    /// Lowercase hex of the SHA-256 of [`MigrationFile::up_path`].
    pub checksum: String,
}

/// A module read from disk: its manifest, the whole parsed TOML document and
/// its migrations in file order.
#[derive(Debug, Clone)]
pub struct LoadedModule {
    /// The typed `[module]` and `[[permissions]]` sections.
    pub manifest: ModuleManifest,
    /// Every section of `module.toml`, for the `core.installed_modules`
    /// manifest column and for activation in a later phase.
    pub document: toml::Value,
    /// The module's migrations, ordered by number.
    pub migrations: Vec<MigrationFile>,
}

/// Everything that can go wrong while reading manifests or talking to the
/// database during an install.
#[derive(Debug, thiserror::Error)]
pub enum InstallError {
    /// A manifest or migration file could not be read.
    #[error("cannot read {path}: {source}")]
    Read {
        /// The file that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// A `module.toml` is not valid TOML or does not match the manifest shape.
    #[error("cannot parse {path}: {source}")]
    Parse {
        /// The manifest that could not be parsed.
        path: PathBuf,
        /// The underlying TOML error, boxed so the whole error stays small
        /// enough to pass freely through `Result`.
        #[source]
        source: Box<toml::de::Error>,
    },
    /// A module depends on a module that is not in the modules directory.
    #[error("module `{module}` depends on `{dependency}`, which is not in {directory}")]
    MissingDependency {
        /// The module whose dependency is missing.
        module: String,
        /// The dependency that is not installed and not on disk.
        dependency: String,
        /// The directory that was searched.
        directory: PathBuf,
    },
    /// The dependencies do not form a directed acyclic graph.
    #[error("modules form a dependency cycle: {chain}")]
    DependencyCycle {
        /// The cycle, as `a -> b -> a`.
        chain: String,
    },
    /// A permission declares a risk level the database would reject.
    #[error(
        "module `{module}` permission `{permission}` has risk level `{risk_level}`; expected one of normal, sensitive, money"
    )]
    InvalidRiskLevel {
        /// The module that declares the permission.
        module: String,
        /// The permission key.
        permission: String,
        /// The risk level that was found.
        risk_level: String,
    },
    /// A file inside `migrations/` is not named `NNNN_name.up.sql`.
    #[error("migration file {path} is named `{file}`; expected NNNN_name.up.sql")]
    InvalidMigrationName {
        /// The file that does not match.
        path: PathBuf,
        /// Its file name.
        file: String,
    },
    /// Two migration files in one module share a number and name.
    #[error("module `{module}` ships migration `{name}` twice")]
    DuplicateMigration {
        /// The module with the duplicate.
        module: String,
        /// The duplicated migration name.
        name: String,
    },
    /// A migration that was applied has a different checksum now. A shipped
    /// migration may never change after it has run.
    #[error(
        "module `{module}` migration `{migration}` was recorded with checksum {recorded} but ships with {shipped}; a shipped migration may not change"
    )]
    ChecksumChanged {
        /// The module that owns the migration.
        module: String,
        /// The migration whose file changed.
        migration: String,
        /// The checksum in `core.applied_migrations`.
        recorded: String,
        /// The checksum of the file on disk.
        shipped: String,
    },
    /// A migration failed; the transaction rolled back, so nothing of it ran.
    #[error("migration `{migration}` of module `{module}` failed: {source}")]
    Migration {
        /// The module being installed.
        module: String,
        /// The migration that failed.
        migration: String,
        /// The underlying failure.
        #[source]
        source: Box<InstallError>,
    },
    /// The seed file could not be read.
    #[error("cannot read the seed {path}: {source}")]
    SeedRead {
        /// The seed file that could not be read.
        path: PathBuf,
        /// The underlying I/O error.
        #[source]
        source: std::io::Error,
    },
    /// Anything rok-db reported: a connection, a query or a transaction.
    #[error(transparent)]
    Database(#[from] rok_db::Error),
    /// Anything sqlx reported while running a migration file.
    #[error(transparent)]
    Sql(#[from] sqlx::Error),
}

/// What one module did during an install.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleReport {
    /// The module key.
    pub module_key: String,
    /// The `core.module_jobs` row this install wrote, so a screen can show
    /// the steps that were recorded. Only absent while the registry itself
    /// is still being created: core installing into an empty database.
    pub job_id: Option<Uuid>,
    /// The migrations that were applied, in order.
    pub applied: Vec<String>,
    /// How many migrations were already recorded and left alone.
    pub already_applied: usize,
    /// How many permissions were upserted into `core.permissions`.
    pub permissions: usize,
}

/// What a whole install did, module by module.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstallReport {
    /// One entry per module, in install order.
    pub modules: Vec<ModuleReport>,
}

impl InstallReport {
    /// The migrations applied across every module.
    #[must_use]
    pub fn total_applied(&self) -> usize {
        self.modules.iter().map(|report| report.applied.len()).sum()
    }
}

/// What one module did when its migrations were rolled back. Rolling back is
/// a development and verification tool; customer data is never rolled back.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RollbackReport {
    /// The module key.
    pub module_key: String,
    /// The down migrations that ran, in reverse order.
    pub rolled_back: Vec<String>,
}

/// The installer: every module in `database/modules`, read once, validated
/// and ordered by dependency.
#[derive(Debug, Clone)]
pub struct ModuleInstaller {
    modules: Vec<LoadedModule>,
}

/// The modules directory this binary was built next to, overridable with
/// `ROK_MODULES_DIR`.
#[must_use]
pub fn default_modules_directory() -> PathBuf {
    std::env::var_os("ROK_MODULES_DIR").map_or_else(
        || Path::new(env!("CARGO_MANIFEST_DIR")).join("../../database/modules"),
        PathBuf::from,
    )
}

/// The Afya story seed, overridable with `ROK_STORY_SEED`.
#[must_use]
pub fn default_story_seed_path() -> PathBuf {
    std::env::var_os("ROK_STORY_SEED").map_or_else(
        || {
            Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("../../database/seeds/afya_pharmacy_story.sql")
        },
        PathBuf::from,
    )
}

/// Open the module's `core.module_jobs` row and write its registry rows, as
/// soon as the registry exists. On a fresh database only core installs before
/// it does, so the job may start later in the module.
async fn begin_module(
    database: &Db,
    module: &LoadedModule,
) -> Result<(Option<Uuid>, bool), (Option<Uuid>, InstallError)> {
    let mut job_id = None;
    let registry_ready = registry_exists(database)
        .await
        .map_err(|error| (None, error))?;
    if registry_ready {
        job_id = Some(
            create_job(database, &module.manifest.module.key)
                .await
                .map_err(|error| (None, error))?,
        );
        write_registry_rows(database, module)
            .await
            .map_err(|error| (job_id, error))?;
    }
    Ok((job_id, registry_ready))
}

/// Run every pending migration, one transaction each, recording progress on
/// the job. Migrations that ran before the registry existed are recorded as
/// soon as it does; core's own 0001 and 0002 are the only such files.
async fn run_pending(
    database: &Db,
    module: &LoadedModule,
    pending: &[&MigrationFile],
    mut job_id: Option<Uuid>,
    mut registry_ready: bool,
) -> Result<PendingOutcome, (Option<Uuid>, InstallError)> {
    let section = &module.manifest.module;
    let mut unrecorded: Vec<&MigrationFile> = Vec::new();
    let mut applied = Vec::with_capacity(pending.len());
    for migration in pending {
        let started = Instant::now();
        let sql = read_utf8(&migration.up_path).map_err(|error| (job_id, error))?;
        let records_itself = registry_ready;
        let elapsed = elapsed_milliseconds(started);
        if let Err(error) =
            apply_migration(database, migration, &sql, elapsed, records_itself).await
        {
            return Err((
                job_id,
                InstallError::Migration {
                    module: section.key.clone(),
                    migration: migration.name.clone(),
                    source: Box::new(error),
                },
            ));
        }
        applied.push(migration.name.clone());

        if !records_itself {
            unrecorded.push(migration);
            // Core's 0003 may have just created the registry.
            registry_ready = registry_exists(database)
                .await
                .map_err(|error| (job_id, error))?;
            if registry_ready {
                job_id = Some(
                    create_job(database, &section.key)
                        .await
                        .map_err(|error| (None, error))?,
                );
                write_registry_rows(database, module)
                    .await
                    .map_err(|error| (job_id, error))?;
                let elapsed = elapsed_milliseconds(started);
                for late in unrecorded.drain(..) {
                    record_migration(database, late, elapsed)
                        .await
                        .map_err(|error| (job_id, error))?;
                }
            }
        }
        if let Some(job) = job_id {
            append_step(database, job, migration, started)
                .await
                .map_err(|error| (job_id, error))?;
        }
    }
    Ok(PendingOutcome {
        applied,
        job_id,
        registry_ready,
    })
}

/// What the migration loop left behind for the rest of the module install.
struct PendingOutcome {
    /// The migration names applied, in order.
    applied: Vec<String>,
    /// The job to close, if the registry existed in time to open one.
    job_id: Option<Uuid>,
    /// Whether the registry exists now, after the loop.
    registry_ready: bool,
}

/// Upsert the module's permissions, mark the module installed and close its
/// job, once the registry exists. Returns how many permissions were written.
async fn finish_module(
    database: &Db,
    module: &LoadedModule,
    job_id: Option<Uuid>,
    registry_ready: bool,
) -> Result<usize, (Option<Uuid>, InstallError)> {
    if !registry_ready {
        return Ok(0);
    }
    let section = &module.manifest.module;
    let mut permissions = 0;
    for permission in &module.manifest.permissions {
        raw(
            "insert into core.permissions (key, module_key, description, risk_level) \
             values (?, ?, ?, ?) \
             on conflict (key) do update set \
               module_key = excluded.module_key, \
               description = excluded.description, \
               risk_level = excluded.risk_level",
        )
        .bind(&permission.key)
        .bind(&section.key)
        .bind(&permission.description)
        .bind(&permission.risk_level)
        .execute(database)
        .await
        .map_err(|error| (job_id, InstallError::from(error)))?;
        permissions += 1;
    }
    raw("update core.installed_modules \
         set status = 'installed', upgraded_at = now() where module_key = ?")
    .bind(&section.key)
    .execute(database)
    .await
    .map_err(|error| (job_id, InstallError::from(error)))?;
    if let Some(job) = job_id {
        raw("update core.module_jobs \
             set status = 'succeeded', finished_at = now() where id = ?")
        .bind(job)
        .execute(database)
        .await
        .map_err(|error| (job_id, InstallError::from(error)))?;
    }
    Ok(permissions)
}

/// One step written to `core.module_jobs.steps` as the install runs.
#[derive(Debug, Serialize)]
struct JobStep {
    key: String,
    label: String,
    status: &'static str,
    duration_milliseconds: u64,
}

/// Read a file as UTF-8, naming it in the error.
fn read_utf8(path: &Path) -> Result<String, InstallError> {
    std::fs::read_to_string(path).map_err(|source| InstallError::Read {
        path: path.to_path_buf(),
        source,
    })
}

/// Lowercase hex of the SHA-256 of a migration file.
fn checksum_of(contents: &str) -> String {
    let digest = Sha256::digest(contents.as_bytes());
    let mut hex = String::with_capacity(digest.len() * 2);
    for byte in digest {
        let _ = write!(hex, "{byte:02x}");
    }
    hex
}

/// `0001_create_core_schema_and_helpers` becomes
/// `Apply 0001 create core schema and helpers`.
fn migration_label(name: &str) -> String {
    let without_number = name.split_once('_').map_or(name, |(_, rest)| rest);
    let words = without_number.replace('_', " ");
    let mut label = String::with_capacity(words.len() + 6);
    label.push_str("Apply ");
    label.push_str(&words);
    label
}

/// Milliseconds elapsed, capped at what the integer column holds.
fn elapsed_milliseconds(since: Instant) -> i64 {
    i64::try_from(since.elapsed().as_millis())
        .unwrap_or(i64::MAX)
        .min(i64::from(i32::MAX))
}

/// Whether the registry tables exist yet. Core creates them with its own
/// migration 0003, so on an empty database they appear in the middle of the
/// install; before that, progress and records cannot be written.
async fn registry_exists(database: &Db) -> Result<bool, InstallError> {
    Ok(
        raw("select to_regclass('core.applied_migrations') is not null")
            .scalar(database)
            .await?,
    )
}

/// Record one applied migration in `core.applied_migrations`.
async fn record_migration<'e, E>(
    executor: E,
    migration: &MigrationFile,
    elapsed_milliseconds: i64,
) -> Result<(), InstallError>
where
    E: rok_db::Executor<'e>,
{
    raw("insert into core.applied_migrations \
           (module_key, migration_name, checksum, duration_milliseconds) \
         values (?, ?, ?, ?) \
         on conflict do nothing")
    .bind(&migration.module_key)
    .bind(&migration.name)
    .bind(&migration.checksum)
    .bind(elapsed_milliseconds)
    .execute(executor)
    .await?;
    Ok(())
}

/// Start a `core.module_jobs` row for this module's install.
async fn create_job(database: &Db, module_key: &str) -> Result<Uuid, InstallError> {
    let (job_id,): (Uuid,) = raw("insert into core.module_jobs \
           (module_key, job_type, status, steps, started_at) \
         values (?, 'install', 'running', '[]'::jsonb, now()) \
         returning id")
    .bind(module_key)
    .fetch_one(database)
    .await?;
    Ok(job_id)
}

/// Append one succeeded step to `core.module_jobs.steps`.
async fn append_step(
    database: &Db,
    job_id: Uuid,
    migration: &MigrationFile,
    started: Instant,
) -> Result<(), InstallError> {
    let step = JobStep {
        key: migration.name.clone(),
        label: migration_label(&migration.name),
        status: "succeeded",
        duration_milliseconds: u64::try_from(started.elapsed().as_millis()).unwrap_or(u64::MAX),
    };
    let step_json = serde_json::to_string(&step)
        .unwrap_or_else(|_| r#"{"key":"step","status":"failed"}"#.to_string());
    raw("update core.module_jobs set steps = steps || ?::jsonb where id = ?")
        .bind(step_json)
        .bind(job_id)
        .execute(database)
        .await?;
    Ok(())
}

/// Upsert `core.installed_modules` and the `core.module_dependencies` rows
/// for one module.
async fn write_registry_rows(database: &Db, module: &LoadedModule) -> Result<(), InstallError> {
    let section = &module.manifest.module;
    let manifest_json = serde_json::to_value(&module.document)
        .unwrap_or_else(|_| serde_json::json!({ "module": { "key": section.key } }));
    raw("insert into core.installed_modules \
           (module_key, display_name, installed_version, schema_name, status, manifest) \
         values (?, ?, ?, ?, 'installing', ?) \
         on conflict (module_key) do update set \
           display_name = excluded.display_name, \
           installed_version = excluded.installed_version, \
           status = 'installing'")
    .bind(&section.key)
    .bind(&section.name)
    .bind(&section.version)
    .bind(&section.schema)
    .bind(manifest_json)
    .execute(database)
    .await?;

    for dependency in section
        .depends_on
        .iter()
        .chain(&section.optional_depends_on)
    {
        let optional = section
            .optional_depends_on
            .iter()
            .any(|candidate| candidate == dependency);
        raw("insert into core.module_dependencies \
               (module_key, depends_on_module_key, is_optional) \
             values (?, ?, ?) \
             on conflict do nothing")
        .bind(&section.key)
        .bind(dependency)
        .bind(optional)
        .execute(database)
        .await?;
    }
    Ok(())
}

/// Run a migration file's statements and its recording in one transaction,
/// committing on success and rolling back on failure.
///
/// This is `Db::transaction` written out by hand: `sqlx::raw_sql` borrows the
/// SQL it runs, and that borrow does not survive the higher-ranked closure
/// `Db::transaction` expects, so the transaction is opened and finished here.
///
/// # Errors
///
/// Fails when the transaction cannot be opened, when the SQL is rejected
/// (rolled back), or when commit fails.
async fn apply_migration(
    database: &Db,
    migration: &MigrationFile,
    sql: &str,
    elapsed_milliseconds: i64,
    records_itself: bool,
) -> Result<(), InstallError> {
    let mut transaction = database.begin().await?;
    let outcome = async {
        sqlx::raw_sql(sql).execute(&mut transaction).await?;
        if records_itself {
            record_migration(&mut transaction, migration, elapsed_milliseconds).await?;
        }
        Ok::<_, InstallError>(())
    }
    .await;
    match outcome {
        Ok(()) => {
            transaction.commit().await?;
            Ok(())
        }
        Err(error) => {
            transaction.rollback().await?;
            Err(error)
        }
    }
}

/// Run one down migration in a transaction and clear its record, committing
/// on success and rolling back on failure.
///
/// # Errors
///
/// Fails when the transaction cannot be opened, when the SQL is rejected
/// (rolled back), or when commit fails.
async fn revert_migration(
    database: &Db,
    migration: &MigrationFile,
    sql: &str,
    registry_ready: bool,
) -> Result<(), InstallError> {
    let mut transaction = database.begin().await?;
    let outcome = async {
        sqlx::raw_sql(sql).execute(&mut transaction).await?;
        if registry_ready {
            let still_there: bool =
                raw("select to_regclass('core.applied_migrations') is not null")
                    .scalar(&mut transaction)
                    .await?;
            if still_there {
                raw("delete from core.applied_migrations \
                     where module_key = ? and migration_name = ?")
                .bind(&migration.module_key)
                .bind(&migration.name)
                .execute(&mut transaction)
                .await?;
            }
        }
        Ok::<_, InstallError>(())
    }
    .await;
    match outcome {
        Ok(()) => {
            transaction.commit().await?;
            Ok(())
        }
        Err(error) => {
            transaction.rollback().await?;
            Err(error)
        }
    }
}

/// Read every migration of one module, ordered by number, with checksums.
fn read_migrations(module_key: &str, directory: &Path) -> Result<Vec<MigrationFile>, InstallError> {
    if !directory.is_dir() {
        return Ok(Vec::new());
    }
    let entries = std::fs::read_dir(directory).map_err(|source| InstallError::Read {
        path: directory.to_path_buf(),
        source,
    })?;

    let mut migrations = Vec::new();
    for entry in entries {
        let entry = entry.map_err(|source| InstallError::Read {
            path: directory.to_path_buf(),
            source,
        })?;
        let path = entry.path();
        let Some(file_name) = path.file_name().and_then(|name| name.to_str()) else {
            continue;
        };
        let Some(stem) = file_name.strip_suffix(".up.sql") else {
            continue;
        };
        let number = stem
            .split_once('_')
            .and_then(|(number, _)| number.parse::<u32>().ok())
            .ok_or_else(|| InstallError::InvalidMigrationName {
                path: path.clone(),
                file: file_name.to_string(),
            })?;
        let sql = read_utf8(&path)?;
        let down_path = directory.join(format!("{stem}.down.sql"));
        migrations.push(MigrationFile {
            module_key: module_key.to_string(),
            name: stem.to_string(),
            number,
            up_path: path,
            down_path: down_path.is_file().then_some(down_path),
            checksum: checksum_of(&sql),
        });
    }

    migrations.sort_by(|left, right| (left.number, &left.name).cmp(&(right.number, &right.name)));
    for pair in migrations.windows(2) {
        if pair[0].name == pair[1].name {
            return Err(InstallError::DuplicateMigration {
                module: module_key.to_string(),
                name: pair[0].name.clone(),
            });
        }
    }
    Ok(migrations)
}

/// Order the loaded modules so every module comes after the modules it
/// depends on, refusing a cycle among the dependencies it cannot do without.
///
/// [`ModuleSection::optional_depends_on`] is a preference about order, not a
/// contract, so it orders the install when it can and is broken when it would
/// otherwise close a circle: `accounting` and `hr` each name the other as
/// optional, and one of them has to install first.
fn dependency_order(
    modules: &[LoadedModule],
    directory: &Path,
) -> Result<Vec<usize>, InstallError> {
    let index_of = |key: &str| {
        modules
            .iter()
            .position(|module| module.manifest.module.key == key)
    };

    for module in modules {
        let section = &module.manifest.module;
        // Required dependencies must be present; optional ones are used when
        // they are and skipped when they are not.
        for dependency in &section.depends_on {
            if index_of(dependency).is_none() {
                return Err(InstallError::MissingDependency {
                    module: section.key.clone(),
                    dependency: dependency.clone(),
                    directory: directory.to_path_buf(),
                });
            }
        }
    }

    // Depth first: a module's dependencies are pushed before the module
    // itself. `visiting` marks the current path so a back edge is a cycle.
    let mut done = vec![false; modules.len()];
    let mut visiting = vec![false; modules.len()];
    let mut path: Vec<usize> = Vec::new();
    let mut ordered = Vec::with_capacity(modules.len());

    for start in 0..modules.len() {
        if done[start] {
            continue;
        }
        visit(
            start,
            modules,
            &index_of,
            &mut done,
            &mut visiting,
            &mut path,
            &mut ordered,
        )?;
    }
    Ok(ordered)
}

/// One depth-first step of [`dependency_order`].
fn visit(
    index: usize,
    modules: &[LoadedModule],
    index_of: &impl Fn(&str) -> Option<usize>,
    done: &mut [bool],
    visiting: &mut [bool],
    path: &mut Vec<usize>,
    ordered: &mut Vec<usize>,
) -> Result<(), InstallError> {
    if done[index] {
        return Ok(());
    }
    if visiting[index] {
        let start = path.iter().position(|&seen| seen == index).unwrap_or(0);
        let mut chain = String::new();
        for &step in &path[start..] {
            let _ = write!(chain, "{} -> ", modules[step].manifest.module.key);
        }
        chain.push_str(&modules[index].manifest.module.key);
        return Err(InstallError::DependencyCycle { chain });
    }

    visiting[index] = true;
    path.push(index);
    let section = &modules[index].manifest.module;

    let mut required: Vec<usize> = section
        .depends_on
        .iter()
        .filter_map(|key| index_of(key))
        .collect();
    required.sort_unstable();
    required.dedup();
    for dependency in required {
        visit(dependency, modules, index_of, done, visiting, path, ordered)?;
    }

    // An optional dependency still has to be applied before the module that
    // uses it when it is present, so it is followed like a required one -
    // except when it would close a circle. `visiting[dependency]` means this
    // module is already on the path, so the two want the opposite order and
    // neither requires it: skip the edge, which is what "optional" allows.
    let mut optional: Vec<usize> = section
        .optional_depends_on
        .iter()
        .filter_map(|key| index_of(key))
        .collect();
    optional.sort_unstable();
    optional.dedup();
    for dependency in optional {
        if visiting[dependency] {
            continue;
        }
        visit(dependency, modules, index_of, done, visiting, path, ordered)?;
    }
    path.pop();
    visiting[index] = false;
    done[index] = true;
    ordered.push(index);
    Ok(())
}

/// What the database has already recorded, per module: migration name to
/// checksum. An empty database has nothing recorded yet, and the tables do
/// not exist yet either, so the absence of the registry is not an error.
async fn recorded_migrations(
    database: &Db,
    installer: &ModuleInstaller,
) -> Result<Vec<BTreeMap<String, String>>, InstallError> {
    let mut recorded = vec![BTreeMap::new(); installer.modules.len()];
    if !registry_exists(database).await? {
        return Ok(recorded);
    }
    for (position, module) in installer.modules.iter().enumerate() {
        let rows: Vec<(String, String)> = raw(
            "select migration_name, checksum from core.applied_migrations where module_key = ?",
        )
        .bind(&module.manifest.module.key)
        .fetch_all(database)
        .await?;
        recorded[position] = rows.into_iter().collect();
    }
    Ok(recorded)
}

impl ModuleInstaller {
    /// Read every `module.toml` under `directory`, validate the dependency
    /// graph and collect each module's migrations with their checksums.
    ///
    /// # Errors
    ///
    /// Fails when a manifest cannot be read or parsed, when a required
    /// dependency is missing from `directory`, when the dependencies form a
    /// cycle, when a permission has a risk level the database would reject,
    /// or when a migration file is misnamed.
    pub fn load(directory: &Path) -> Result<Self, InstallError> {
        let entries = std::fs::read_dir(directory).map_err(|source| InstallError::Read {
            path: directory.to_path_buf(),
            source,
        })?;

        let mut modules = Vec::new();
        for entry in entries {
            let entry = entry.map_err(|source| InstallError::Read {
                path: directory.to_path_buf(),
                source,
            })?;
            let path = entry.path();
            if !path.is_dir() {
                continue;
            }
            let manifest_path = path.join("module.toml");
            if !manifest_path.is_file() {
                continue;
            }
            let source = read_utf8(&manifest_path)?;
            let document: toml::Value =
                toml::from_str(&source).map_err(|source| InstallError::Parse {
                    path: manifest_path.clone(),
                    source: Box::new(source),
                })?;
            let manifest: ModuleManifest =
                toml::from_str(&source).map_err(|source| InstallError::Parse {
                    path: manifest_path.clone(),
                    source: Box::new(source),
                })?;

            let section = &manifest.module;
            for permission in &manifest.permissions {
                if !RISK_LEVELS.contains(&permission.risk_level.as_str()) {
                    return Err(InstallError::InvalidRiskLevel {
                        module: section.key.clone(),
                        permission: permission.key.clone(),
                        risk_level: permission.risk_level.clone(),
                    });
                }
            }

            let migrations = read_migrations(&section.key, &path.join("migrations"))?;
            modules.push(LoadedModule {
                manifest,
                document,
                migrations,
            });
        }

        modules.sort_by(|left, right| left.manifest.module.key.cmp(&right.manifest.module.key));
        let order = dependency_order(&modules, directory)?;
        let modules = order
            .into_iter()
            .map(|index| modules[index].clone())
            .collect();
        Ok(Self { modules })
    }

    /// The loaded modules, in the order they install.
    #[must_use]
    pub fn modules(&self) -> &[LoadedModule] {
        &self.modules
    }

    /// Install every module, in dependency order.
    ///
    /// First the recorded checksums are checked against the files that ship,
    /// so a changed migration is refused before any DDL runs. Then each
    /// pending `NNNN_*.up.sql` runs in its own transaction together with the
    /// row that records it in `core.applied_migrations`, the module's
    /// permissions are upserted into `core.permissions`, and the progress of
    /// each step is appended to `core.module_jobs.steps`.
    ///
    /// Running it again is a no-op: everything is already recorded.
    ///
    /// # Errors
    ///
    /// Fails when a shipped migration's checksum differs from the recorded
    /// one, when a migration fails (its transaction rolls back), or on any
    /// database error.
    pub async fn install(&self, database: &Db) -> Result<InstallReport, InstallError> {
        let recorded = recorded_migrations(database, self).await?;

        // Phase 1: refuse to start if anything that shipped has changed.
        for (position, module) in self.modules.iter().enumerate() {
            for migration in &module.migrations {
                if let Some(checksum) = recorded[position].get(&migration.name)
                    && checksum != &migration.checksum
                {
                    return Err(InstallError::ChecksumChanged {
                        module: module.manifest.module.key.clone(),
                        migration: migration.name.clone(),
                        recorded: checksum.clone(),
                        shipped: migration.checksum.clone(),
                    });
                }
            }
        }

        // Phase 2: apply what is still pending, module by module.
        let mut reports = Vec::with_capacity(self.modules.len());
        for (position, module) in self.modules.iter().enumerate() {
            let pending: Vec<&MigrationFile> = module
                .migrations
                .iter()
                .filter(|migration| !recorded[position].contains_key(&migration.name))
                .collect();
            let already_applied = module.migrations.len() - pending.len();
            reports.push(
                self.install_module(database, module, pending, already_applied)
                    .await?,
            );
        }
        Ok(InstallReport { modules: reports })
    }

    /// Install one module, marking its job failed if anything goes wrong.
    async fn install_module(
        &self,
        database: &Db,
        module: &LoadedModule,
        pending: Vec<&MigrationFile>,
        already_applied: usize,
    ) -> Result<ModuleReport, InstallError> {
        match self.apply_module(database, module, &pending).await {
            Ok(report) => Ok(ModuleReport {
                already_applied,
                ..report
            }),
            Err((job_id, error)) => {
                if let Some(job_id) = job_id {
                    let message = error.to_string();
                    let _ = raw("update core.module_jobs \
                         set status = 'failed', error_message = ?, finished_at = now() \
                         where id = ?")
                    .bind(message)
                    .bind(job_id)
                    .execute(database)
                    .await;
                    let _ = raw("update core.installed_modules \
                         set status = 'failed' where module_key = ?")
                    .bind(&module.manifest.module.key)
                    .execute(database)
                    .await;
                }
                Err(error)
            }
        }
    }

    /// The work an install does. On failure it returns the job to mark, when
    /// one exists, alongside the error.
    async fn apply_module(
        &self,
        database: &Db,
        module: &LoadedModule,
        pending: &[&MigrationFile],
    ) -> Result<ModuleReport, (Option<Uuid>, InstallError)> {
        let (job_id, registry_ready) = begin_module(database, module).await?;
        let outcome = run_pending(database, module, pending, job_id, registry_ready).await?;
        let permissions =
            finish_module(database, module, outcome.job_id, outcome.registry_ready).await?;

        Ok(ModuleReport {
            module_key: module.manifest.module.key.clone(),
            job_id: outcome.job_id,
            applied: outcome.applied,
            already_applied: 0,
            permissions,
        })
    }

    /// Roll every module back, in reverse dependency order, running each
    /// module's down migrations in reverse and clearing the records. This is
    /// for the migration verifier and for development; customer data is never
    /// rolled back.
    ///
    /// # Errors
    ///
    /// Fails when a down migration fails (its transaction rolls back) or on
    /// any database error.
    pub async fn rollback(&self, database: &Db) -> Result<Vec<RollbackReport>, InstallError> {
        let registry_ready = registry_exists(database).await?;
        let mut reports = Vec::with_capacity(self.modules.len());
        for module in self.modules.iter().rev() {
            let mut rolled_back = Vec::new();
            for migration in module.migrations.iter().rev() {
                let Some(down_path) = &migration.down_path else {
                    continue;
                };
                let sql = read_utf8(down_path)?;
                revert_migration(database, migration, &sql, registry_ready).await?;
                rolled_back.push(migration.name.clone());
            }

            if registry_ready {
                let still_there: bool =
                    raw("select to_regclass('core.installed_modules') is not null")
                        .scalar(database)
                        .await?;
                if still_there {
                    raw("delete from core.installed_modules where module_key = ?")
                        .bind(&module.manifest.module.key)
                        .execute(database)
                        .await?;
                }
            }
            reports.push(RollbackReport {
                module_key: module.manifest.module.key.clone(),
                rolled_back,
            });
        }
        Ok(reports)
    }
}

/// Run one SQL file (the seed) inside a single transaction.
///
/// # Errors
///
/// Fails when the file cannot be read or when PostgreSQL rejects it; the
/// transaction rolls back, so a failed seed leaves nothing behind.
pub async fn load_sql_file(database: &Db, path: &Path) -> Result<(), InstallError> {
    let sql = std::fs::read_to_string(path).map_err(|source| InstallError::SeedRead {
        path: path.to_path_buf(),
        source,
    })?;
    let mut transaction = database.begin().await?;
    let outcome = async {
        sqlx::raw_sql(&sql).execute(&mut transaction).await?;
        Ok::<_, InstallError>(())
    }
    .await;
    match outcome {
        Ok(()) => {
            transaction.commit().await?;
            Ok(())
        }
        Err(error) => {
            transaction.rollback().await?;
            Err(error)
        }
    }
}

/// Load the Afya Pharmacy story seed from [`default_story_seed_path`].
///
/// # Errors
///
/// Fails when the seed file is missing or PostgreSQL rejects it.
pub async fn load_afya_story(database: &Db) -> Result<(), InstallError> {
    load_sql_file(database, &default_story_seed_path()).await
}
