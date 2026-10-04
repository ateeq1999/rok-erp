//! Phase 1: the installer applies every module in dependency order,
//! records each migration with a checksum, writes progress to
//! `core.module_jobs.steps`, upserts the permissions, refuses a shipped
//! migration whose recorded checksum changed, and is a no-op the second time.
//!
//! Running the whole install also proves the pharmacy 0002 migration applies
//! on a fresh PostgreSQL.

use rok_db::{Db, raw};
use rok_pos_database::{InstallError, ModuleInstaller, default_modules_directory};

/// The installer over the modules this repository ships.
fn installer() -> ModuleInstaller {
    ModuleInstaller::load(&default_modules_directory()).expect("the manifests load")
}

/// How many modules the repository ships, counted from disk so the assertions
/// below track the tree rather than a number a reader has to keep in step.
fn shipped_module_count() -> usize {
    std::fs::read_dir(default_modules_directory())
        .expect("the module folder can be read")
        .filter_map(Result::ok)
        .filter(|entry| entry.path().join("module.toml").is_file())
        .count()
}

/// How many up migrations the repository ships in total.
fn shipped_migration_count(installer: &ModuleInstaller) -> usize {
    installer
        .modules()
        .iter()
        .map(|module| module.migrations.len())
        .sum()
}

/// Every module comes after the modules it depends on.
#[test]
fn the_modules_are_ordered_by_dependency() {
    let installer = installer();
    let order: Vec<&str> = installer
        .modules()
        .iter()
        .map(|module| module.manifest.module.key.as_str())
        .collect();

    assert_eq!(order.first(), Some(&"core"), "core installs first");
    assert_eq!(
        order.len(),
        shipped_module_count(),
        "every module folder is loaded, and loaded once"
    );
    for module in installer.modules() {
        for dependency in &module.manifest.module.depends_on {
            let module_at = order
                .iter()
                .position(|key| *key == module.manifest.module.key)
                .expect("the module is in the order");
            let dependency_at = order
                .iter()
                .position(|key| key == dependency)
                .expect("the dependency is in the order");
            assert!(
                dependency_at < module_at,
                "{dependency} must install before {}",
                module.manifest.module.key
            );
        }
    }
}

/// The four spikes' promise, now for the real thing: an empty database ends
/// up with every module, and installing again changes nothing.
#[rok_db::test]
async fn an_empty_database_installs_every_module_then_changes_nothing(db: Db) {
    let installer = installer();
    let shipped = shipped_migration_count(&installer);
    let modules = shipped_module_count();

    let report = installer.install(&db).await.expect("the install succeeds");
    assert_eq!(report.modules.len(), modules, "every module was installed");
    assert_eq!(report.total_applied(), shipped, "every shipped file ran");

    let recorded: i64 = raw("select count(*) from core.applied_migrations")
        .scalar(&db)
        .await
        .expect("the registry counts its rows");
    assert_eq!(recorded, i64::try_from(shipped).unwrap_or(i64::MAX));

    let installed: i64 =
        raw("select count(*) from core.installed_modules where status = 'installed'")
            .scalar(&db)
            .await
            .expect("the module registry counts its rows");
    assert_eq!(
        installed,
        i64::try_from(modules).unwrap_or(i64::MAX),
        "every module is marked installed"
    );

    // Progress: one succeeded job per module, every one carrying steps.
    let jobs: i64 = raw("select count(*) from core.module_jobs \
         where job_type = 'install' and status = 'succeeded' \
           and jsonb_array_length(steps) > 0")
    .scalar(&db)
    .await
    .expect("the jobs count their steps");
    assert_eq!(
        jobs,
        i64::try_from(modules).unwrap_or(i64::MAX),
        "every module recorded its steps"
    );

    // The pharmacy permissions from plan step 3, plus the ones 0001 shipped.
    for key in [
        "pharmacy.clinical_check.approve",
        "pharmacy.clinical_notes.view",
        "pharmacy.recalls.manage",
        "pharmacy.licences.manage",
        "pharmacy.temperature_logs.record",
        "pharmacy.interaction_rules.manage",
    ] {
        let found: Option<(String,)> = raw("select key from core.permissions where key = ?")
            .bind(key)
            .fetch_optional(&db)
            .await
            .expect("the permission lookup runs");
        assert_eq!(found.map(|(found,)| found).as_deref(), Some(key));
    }

    // The 0002 tables exist beside 0001's, and the claim batch column too.
    let pharmacy_tables: i64 =
        raw("select count(*) from information_schema.tables where table_schema = 'pharmacy'")
            .scalar(&db)
            .await
            .expect("the catalog counts pharmacy tables");
    assert_eq!(
        pharmacy_tables, 21,
        "six tables from 0001 and fifteen from 0002"
    );

    let claim_batch: i64 = raw("select count(*) from information_schema.columns \
         where table_schema = 'pharmacy' and table_name = 'insurance_claims' \
           and column_name = 'claim_batch_id'")
    .scalar(&db)
    .await
    .expect("the catalog counts columns");
    assert_eq!(claim_batch, 1, "0002 added claim_batch_id");

    // Second run: nothing pending, everything already recorded.
    let second = installer
        .install(&db)
        .await
        .expect("the second install runs");
    assert_eq!(
        second.total_applied(),
        0,
        "installing twice applies nothing new"
    );
    for report in &second.modules {
        assert!(
            report.already_applied > 0 || report.applied.is_empty(),
            "{} recorded something it should not have",
            report.module_key
        );
    }
    let still_recorded: i64 = raw("select count(*) from core.applied_migrations")
        .scalar(&db)
        .await
        .expect("the registry still counts");
    assert_eq!(still_recorded, i64::try_from(shipped).unwrap_or(i64::MAX));
}

/// A migration that changed after it was applied is refused before any DDL
/// runs, as plan step 2 asks.
#[rok_db::test]
async fn a_changed_migration_is_refused_before_anything_runs(db: Db) {
    let installer = installer();
    installer.install(&db).await.expect("the install succeeds");

    raw("update core.applied_migrations set checksum = 'tampered' \
         where module_key = 'pharmacy' \
           and migration_name = '0002_add_clinical_stock_and_compliance'")
    .execute(&db)
    .await
    .expect("the recorded checksum is tampered with");

    let refused = installer.install(&db).await;
    match refused {
        Err(InstallError::ChecksumChanged {
            module,
            migration,
            recorded,
            shipped,
        }) => {
            assert_eq!(module, "pharmacy");
            assert_eq!(migration, "0002_add_clinical_stock_and_compliance");
            assert_eq!(recorded, "tampered");
            assert_ne!(shipped, "tampered", "the file on disk is untouched");
        }
        Err(other) => panic!("the install was refused, but with the wrong error: {other}"),
        Ok(_) => panic!("a changed migration must be refused"),
    }
}

/// Rolling the modules back and applying them again is what the migration
/// verifier does; it proves every `.down.sql` really reverses its `.up.sql`.
#[rok_db::test]
async fn every_down_migration_reverses_then_reapplies(db: Db) {
    let installer = installer();
    installer.install(&db).await.expect("the install succeeds");

    let rolled = installer.rollback(&db).await.expect("the rollback runs");
    let rolled_back: usize = rolled.iter().map(|report| report.rolled_back.len()).sum();
    let shipped = shipped_migration_count(&installer);
    assert_eq!(
        rolled_back, shipped,
        "every up migration has a down migration that ran"
    );

    // Every schema a module owns is gone, asked of the installer's own list so
    // the assertion tracks whatever the repository ships.
    for module in installer.modules() {
        let schema = module.manifest.module.schema.as_str();
        let left: i64 = raw("select count(*) from information_schema.schemata \
             where schema_name = ?")
        .bind(schema)
        .scalar(&db)
        .await
        .expect("the catalog counts the module schema");
        assert_eq!(
            left, 0,
            "{}'s schema {schema} is gone",
            module.manifest.module.key
        );
    }

    let again = installer
        .install(&db)
        .await
        .expect("the second install runs");
    assert_eq!(again.total_applied(), shipped, "everything applied again");
}

/// Relaxing the cycle rule for `optional_depends_on` must not have removed
/// cycle detection: `depends_on` closing a circle still stops the install.
#[test]
fn a_cycle_of_required_dependencies_is_refused() {
    let root = std::env::temp_dir().join(format!(
        "rok-installer-cycle-{}",
        uuid::Uuid::now_v7().simple()
    ));
    for (key, dependency) in [("alpha", "beta"), ("beta", "alpha")] {
        let folder = root.join(key);
        std::fs::create_dir_all(folder.join("migrations")).expect("the module folder can be made");
        std::fs::write(
            folder.join("module.toml"),
            format!(
                "[module]\n\
                 key = \"{key}\"\n\
                 name = \"{key}\"\n\
                 version = \"1.0.0\"\n\
                 schema = \"{key}\"\n\
                 depends_on = [\"{dependency}\"]\n"
            ),
        )
        .expect("the manifest can be written");
    }

    match ModuleInstaller::load(&root) {
        Err(InstallError::DependencyCycle { chain }) => {
            assert!(
                chain.contains("alpha") && chain.contains("beta"),
                "the chain names both halves of the cycle: {chain}"
            );
        }
        Err(other) => panic!("the cycle was refused, but with the wrong error: {other}"),
        Ok(_) => panic!("a cycle of required dependencies must be refused"),
    }
    let _ = std::fs::remove_dir_all(&root);
}
