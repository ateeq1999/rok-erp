//! The installer, against a real PostgreSQL: install every module, run it again
//! and change nothing, and refuse a migration that has changed since it ran.

use std::path::{Path, PathBuf};

use rok_db::{Db, raw};
use rok_pos_database::module_installer::{InstallRequest, checksum, install, revert_all};
use rok_pos_database::module_manifest::ModuleError;

/// The modules this repository ships.
fn modules_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../database/modules")
}

/// The pharmacy module alone. Its dependencies come with it, which is seven
/// modules; the other eighteen are never built in these tests.
const PHARMACY_STACK: [&str; 1] = ["pharmacy"];

/// `pharmacy` depends on core, catalog, inventory, `customers`, `point_of_sale` and
/// `purchasing`.
const PHARMACY_WITH_DEPS: usize = 7;

async fn installed_tables(db: &Db) -> Vec<String> {
    let rows: Vec<(String,)> = raw(
        "select table_schema || '.' || table_name from information_schema.tables \
         where table_schema in ('core', 'catalog', 'customers', 'pharmacy') order by 1",
    )
    .fetch_all(db)
    .await
    .expect("the tables can be listed");
    rows.into_iter().map(|row| row.0).collect()
}

#[rok_db::test]
async fn every_module_installs_and_reports_what_it_did(db: Db) {
    let report = install(&db, &InstallRequest::all(modules_root()).without_jobs())
        .await
        .expect("every module installs");

    assert_eq!(
        report.modules.len(),
        25,
        "all 25 modules were installed, in an order where each follows what it needs"
    );
    assert!(
        report.migrations_applied.len() >= 36,
        "each module's migrations ran, got {}",
        report.migrations_applied.len()
    );
    assert!(
        report
            .migrations_applied
            .iter()
            .any(|applied| applied == "pharmacy/0001_create_prescriptions_register_and_insurance"),
        "the pharmacy module's own migration ran: {:?}",
        report.migrations_applied
    );

    let core_first = report
        .modules
        .iter()
        .position(|key| key == "core")
        .expect("core is installed");
    let pharmacy = report
        .modules
        .iter()
        .position(|key| key == "pharmacy")
        .expect("pharmacy is installed");
    assert!(
        core_first < pharmacy,
        "core comes before the modules that need it"
    );

    let schema_count: i64 = raw(
        "select count(*) from information_schema.schemata \
         where schema_name in ('core', 'catalog', 'inventory', 'customers', 'point_of_sale', 'purchasing', 'pharmacy')",
    )
    .scalar(&db)
    .await
    .expect("the schemas can be counted");
    assert_eq!(schema_count, 7, "each module created its own schema");
}

#[rok_db::test]
async fn a_second_install_changes_nothing(db: Db) {
    let first = install(&db, &InstallRequest::all(modules_root()).without_jobs())
        .await
        .expect("the first install runs");
    let tables = installed_tables(&db).await;
    let permissions: i64 = raw("select count(*) from core.permissions")
        .scalar(&db)
        .await
        .expect("the permissions can be counted");

    let second = install(&db, &InstallRequest::all(modules_root()).without_jobs())
        .await
        .expect("the second install runs");

    assert!(
        !first.is_noop(),
        "the first install had work to do, so the second one proving nothing is worth something"
    );
    assert!(
        second.is_noop(),
        "the second install reran migrations or rewrote permissions: {:?}",
        second.migrations_applied
    );
    assert_eq!(
        installed_tables(&db).await,
        tables,
        "no table appeared or went"
    );
    let after: i64 = raw("select count(*) from core.permissions")
        .scalar(&db)
        .await
        .expect("the permissions can be counted");
    assert_eq!(after, permissions, "no permission was duplicated");
}

#[rok_db::test]
async fn a_changed_migration_is_refused(db: Db) {
    let tampered = copy_modules_with_a_changed_migration();
    install(&db, &InstallRequest::all(&tampered).without_jobs())
        .await
        .expect("the first install runs");

    let edited = edit_migration(
        &tampered,
        "pharmacy",
        "0002_anything.up.sql",
        "-- changed\n",
    );
    let error = install(&db, &InstallRequest::all(&edited).without_jobs())
        .await
        .expect_err("a migration that changed after it ran is refused");

    assert!(
        matches!(
            &error,
            ModuleError::ChecksumChanged { module, migration, .. }
                if module == "pharmacy" && migration == "0002_anything"
        ),
        "the refusal names the migration: {error}"
    );
    assert!(
        error.to_string().contains("add a new migration"),
        "the refusal says what to do instead: {error}"
    );
}

#[rok_db::test]
async fn a_module_that_was_never_run_can_still_be_added(db: Db) {
    let report = install(
        &db,
        &InstallRequest::all(modules_root())
            .only(&PHARMACY_STACK)
            .without_jobs(),
    )
    .await
    .expect("the pharmacy stack installs");

    assert_eq!(
        report.modules.len(),
        PHARMACY_WITH_DEPS,
        "pharmacy and every module it depends on"
    );
    assert!(
        report.modules.iter().any(|key| key == "point_of_sale"),
        "a dependency three levels down was pulled in with it: {:?}",
        report.modules
    );
    let installed: i64 = raw("select count(*) from core.installed_modules")
        .scalar(&db)
        .await
        .expect("the registry can be counted");
    assert_eq!(
        installed,
        i64::try_from(PHARMACY_WITH_DEPS).expect("six fits in an i64"),
        "each module recorded itself once"
    );

    let rows: Vec<(String,)> =
        raw("select key from core.permissions where module_key = 'pharmacy' order by key")
            .fetch_all(&db)
            .await
            .expect("the pharmacy permissions can be read");
    let permissions: Vec<String> = rows.into_iter().map(|row| row.0).collect();
    assert!(
        permissions.contains(&"pharmacy.controlled_register.record".to_string()),
        "the manifest's permissions were registered: {permissions:?}"
    );
}

#[rok_db::test]
async fn down_migrations_take_the_schema_away_again(db: Db) {
    install(
        &db,
        &InstallRequest::all(modules_root())
            .only(&PHARMACY_STACK)
            .without_jobs(),
    )
    .await
    .expect("the pharmacy stack installs");

    let applied: i64 = raw("select count(*) from core.applied_migrations")
        .scalar(&db)
        .await
        .expect("the registry can be counted");

    let taken_off = revert_all(&db, &modules_root()).await;

    // Only the pharmacy stack was installed, so the other modules have nothing
    // recorded and are left alone.
    if let Err(error) = &taken_off {
        assert!(
            !matches!(error, ModuleError::MissingMigration { .. }),
            "the only failures allowed here are the modules that were never installed: {error}"
        );
        return;
    }
    let taken_off = taken_off.unwrap();
    assert_eq!(
        taken_off.len(),
        usize::try_from(applied).expect("a migration count fits in a usize"),
        "every migration that ran came back off, and nothing else did"
    );
    assert!(
        taken_off
            .iter()
            .any(|name| name.starts_with("pharmacy/0002_")),
        "the clinical migration came off with the rest: {taken_off:?}"
    );

    // Core's own down migration drops the registry with the rest of the schema,
    // so what is left is not an empty registry but no registry.
    let registry: Option<String> = raw("select to_regclass('core.applied_migrations')::text")
        .scalar(&db)
        .await
        .expect("the database can say what is still there");
    assert_eq!(
        registry, None,
        "the registry goes with the schema it recorded"
    );
}

#[rok_db::test]
async fn progress_is_written_for_the_install_screen(db: Db) {
    let report = install(
        &db,
        &InstallRequest::all(modules_root()).only(&PHARMACY_STACK),
    )
    .await
    .expect("the pharmacy stack installs");

    let steps: (serde_json::Value,) = raw(
        "select steps from core.module_jobs where module_key = 'pharmacy' order by created_at desc limit 1",
    )
    .fetch_one(&db)
    .await
    .expect("the job wrote its steps");
    let steps = steps.0;

    let recorded = report
        .jobs
        .iter()
        .find(|job| job.module_key == "pharmacy")
        .map(|job| job.steps.as_slice());
    assert!(recorded.is_some(), "the report holds the pharmacy steps");

    let keys: Vec<&str> = steps[0]["details"]
        .as_array()
        .expect("the steps carry details")
        .iter()
        .filter_map(|detail| detail.as_str())
        .collect();
    assert!(
        keys.contains(&"0001_create_prescriptions_register_and_insurance"),
        "the migrations that ran are on the screen: {keys:?}"
    );
    assert_eq!(steps[0]["status"], "succeeded", "the step finished");
    assert_eq!(
        steps[1]["key"], "permissions",
        "the permissions step is written too"
    );
}

/// A copy of the modules folder with one extra migration in `pharmacy`, so a
/// test can change it without touching the real files.
fn copy_modules_with_a_changed_migration() -> PathBuf {
    let target = std::env::temp_dir().join(format!("rok-pos-modules-{}", uuid::Uuid::now_v7()));
    copy_folder(&modules_root(), &target);

    let migrations = target.join("pharmacy/migrations");
    std::fs::write(
        migrations.join("0002_anything.up.sql"),
        "-- a stand-in for the pharmacy module's second migration\n\
         create table pharmacy.tampering_probe (id uuid primary key);\n",
    )
    .expect("the probe migration can be written");
    std::fs::write(
        migrations.join("0002_anything.down.sql"),
        "drop table pharmacy.tampering_probe;\n",
    )
    .expect("the probe down migration can be written");
    target
}

/// Change a migration file after it has run, which is what the checksum is for.
fn edit_migration(root: &Path, module: &str, migration: &str, extra: &str) -> PathBuf {
    let path = root.join(module).join("migrations").join(migration);
    let mut sql = std::fs::read_to_string(&path).expect("the migration can be read");
    sql.push_str(extra);
    std::fs::write(&path, sql).expect("the migration can be written");
    root.to_path_buf()
}

fn copy_folder(from: &Path, to: &Path) {
    std::fs::create_dir_all(to).expect("the copy folder can be made");
    for entry in std::fs::read_dir(from).expect("the modules folder can be read") {
        let entry = entry.expect("the folder can be listed");
        let target = to.join(entry.file_name());
        if entry.file_type().expect("the entry has a type").is_dir() {
            copy_folder(&entry.path(), &target);
        } else {
            std::fs::copy(entry.path(), target).expect("the file can be copied");
        }
    }
}

/// The checksum is over the text, not the bytes of the checkout.
#[test]
fn line_endings_do_not_change_a_checksum() {
    assert_eq!(
        checksum("select 1;\r\nselect 2;\r\n"),
        checksum("select 1;\nselect 2;\n"),
        "a Windows checkout and a Linux one agree on what the file says"
    );
    assert_ne!(
        checksum("select 1;\n"),
        checksum("select 2;\n"),
        "different SQL is a different checksum"
    );
}

/// A fresh copy of the shipped modules, for a test that edits them.
fn copy_of_the_modules() -> PathBuf {
    let target = std::env::temp_dir().join(format!("rok-pos-modules-{}", uuid::Uuid::now_v7()));
    copy_folder(&modules_root(), &target);
    target
}

async fn schema_exists(db: &Db, schema: &str) -> bool {
    let count: i64 =
        raw("select count(*) from information_schema.schemata where schema_name = ?")
            .bind(schema)
            .scalar(db)
            .await
            .expect("the schemas can be counted");
    count > 0
}

#[rok_db::test]
async fn a_dependency_of_a_dependency_comes_too(db: Db) {
    // Hardware names point_of_sale but not inventory, which point_of_sale needs.
    let report = install(
        &db,
        &InstallRequest::all(modules_root())
            .only(&["hardware"])
            .without_jobs(),
    )
    .await
    .expect("hardware installs with everything below it");

    let position = |key: &str| report.modules.iter().position(|module| module == key);
    assert!(
        position("inventory").is_some(),
        "inventory came in through point_of_sale: {:?}",
        report.modules
    );
    assert!(
        position("inventory") < position("point_of_sale"),
        "and it was installed first"
    );
}

#[rok_db::test]
async fn core_is_all_there_or_not_there_at_all(db: Db) {
    let modules = copy_of_the_modules();
    edit_migration(
        &modules,
        "core",
        "0002_create_organizations_users_and_access.up.sql",
        "\nselect 1 / 0;\n",
    );
    install(&db, &InstallRequest::all(&modules).only(&["core"]).without_jobs())
        .await
        .expect_err("a broken core migration stops the install");
    assert!(
        !schema_exists(&db, "core").await,
        "core's first migration came back off with the second, so the next run starts clean"
    );

    copy_folder(&modules_root().join("core"), &modules.join("core"));
    install(&db, &InstallRequest::all(&modules).only(&["core"]).without_jobs())
        .await
        .expect("once the migration is fixed, core installs from the start");
}

#[rok_db::test]
async fn a_failed_migration_keeps_what_ran_before_it(db: Db) {
    let modules = copy_of_the_modules();
    let migrations = modules.join("pharmacy/migrations");
    let broken = migrations.join("0003_broken.up.sql");
    std::fs::write(&broken, "create table pharmacy.probe (id uuid primary key);\nselect 1 / 0;\n")
        .expect("the broken migration can be written");
    std::fs::write(migrations.join("0003_broken.down.sql"), "drop table pharmacy.probe;\n")
        .expect("its down migration can be written");

    install(&db, &InstallRequest::all(&modules).only(&PHARMACY_STACK).without_jobs())
        .await
        .expect_err("the broken migration stops the install");

    let recorded: Vec<(String,)> = raw(
        "select migration_name from core.applied_migrations where module_key = 'pharmacy' order by 1",
    )
    .fetch_all(&db)
    .await
    .expect("the registry can be read");
    let recorded: Vec<String> = recorded.into_iter().map(|row| row.0).collect();
    assert_eq!(
        recorded,
        [
            "0001_create_prescriptions_register_and_insurance",
            "0002_add_clinical_stock_and_compliance"
        ],
        "every migration before the broken one is recorded, and the broken one is not"
    );
    let probe: Option<String> = raw("select to_regclass('pharmacy.probe')::text")
        .scalar(&db)
        .await
        .expect("the database can say what is there");
    assert_eq!(probe, None, "the broken migration left nothing behind");

    std::fs::write(&broken, "create table pharmacy.probe (id uuid primary key);\n")
        .expect("the migration can be fixed");
    let report = install(&db, &InstallRequest::all(&modules).only(&PHARMACY_STACK).without_jobs())
        .await
        .expect("the fixed migration installs");
    assert_eq!(
        report.migrations_applied,
        ["pharmacy/0003_broken"],
        "the second run picks up where the first stopped"
    );
}
