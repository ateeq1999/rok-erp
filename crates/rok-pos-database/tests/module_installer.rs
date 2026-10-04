//! Phase 1: the installer applies the eight modules in dependency order,
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
    assert_eq!(order.len(), 8, "eight modules ship");
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
/// up with all eight modules, and installing again changes nothing.
#[rok_db::test]
async fn an_empty_database_installs_every_module_then_changes_nothing(db: Db) {
    let installer = installer();
    let shipped = shipped_migration_count(&installer);

    let report = installer.install(&db).await.expect("the install succeeds");
    assert_eq!(report.modules.len(), 8, "eight modules installed");
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
    assert_eq!(installed, 8, "all eight modules are marked installed");

    // Progress: one succeeded job per module, every one carrying steps.
    let jobs: i64 = raw("select count(*) from core.module_jobs \
         where job_type = 'install' and status = 'succeeded' \
           and jsonb_array_length(steps) > 0")
    .scalar(&db)
    .await
    .expect("the jobs count their steps");
    assert_eq!(jobs, 8, "every module recorded its steps");

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

    let schemas_left: i64 = raw("select count(*) from information_schema.schemata \
         where schema_name in ('core', 'catalog', 'inventory', 'customers', \
                                'point_of_sale', 'purchasing', 'marketplace', 'pharmacy')")
    .scalar(&db)
    .await
    .expect("the catalog counts schemas");
    assert_eq!(schemas_left, 0, "every module schema is gone");

    let again = installer
        .install(&db)
        .await
        .expect("the second install runs");
    assert_eq!(again.total_applied(), shipped, "everything applied again");
}
