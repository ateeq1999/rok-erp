//! Apply every module's up migrations, then every down migration, then the
//! ups again, and fail loudly if any of the three rounds disagrees.
//!
//! This is the Rust port of the verifier the plan asks for (phase 1, step 6).
//! A down migration that does not reverse its up one is a migration nobody
//! finds out about until a customer's upgrade has to be undone, so the pair is
//! exercised on a throwaway database in CI rather than in anger.
//!
//! It runs against a database of its own on the server `.env` or
//! `DATABASE_URL` names and drops it afterwards, so it can never touch
//! development data:
//!
//! ```text
//! cargo run -p migration-verifier
//! ```

use rok_db::testing::TestDb;
use rok_pos_database::{ModuleInstaller, default_modules_directory};

/// One round of the verifier: what the installer did, in numbers.
#[derive(Debug)]
struct Round {
    /// The migrations applied, module by module.
    applied: usize,
    /// The modules that ran, in dependency order.
    modules: Vec<String>,
}

/// Run one install round against `db`.
async fn install(installer: &ModuleInstaller, db: &rok_db::Db) -> Round {
    let report = installer
        .install(db)
        .await
        .expect("every up migration applies");
    Round {
        applied: report.total_applied(),
        modules: report
            .modules
            .iter()
            .map(|module| module.module_key.clone())
            .collect(),
    }
}

/// Read `.env` into the environment before the verifier asks for
/// `DATABASE_URL`.
///
/// dotenvy looks in the working directory and then its parents, and a name
/// already set in the environment wins over the file. A missing `.env` is not
/// an error: CI sets `DATABASE_URL` itself.
fn load_environment() {
    if let Ok(path) = dotenvy::dotenv() {
        println!("using {}", path.display());
    }
}

#[tokio::main]
async fn main() {
    load_environment();
    let installer = ModuleInstaller::load(&default_modules_directory())
        .expect("every module.toml reads and orders itself");

    // A database of its own: the verifier is destructive by design, so it is
    // never pointed at a database anyone works in.
    let Some(test_db) = TestDb::create().await.expect("the database creates") else {
        eprintln!("migration-verifier: DATABASE_URL is not set, nothing to verify");
        std::process::exit(2);
    };
    let db = test_db.db();
    println!("verifying against {}", test_db.name());

    // 1. Every up migration, on an empty database.
    let first = install(&installer, db).await;
    println!(
        "ups:    {} migrations across {} modules",
        first.applied,
        first.modules.len()
    );

    // 2. Every down migration, in reverse order.
    let rolled_back = installer
        .rollback(db)
        .await
        .expect("every down migration reverses its up one");
    let reversed: usize = rolled_back
        .iter()
        .map(|report| report.rolled_back.len())
        .sum();
    println!("downs:  {reversed} migrations reversed");
    assert_eq!(
        reversed, first.applied,
        "every up migration has a down migration that runs"
    );

    // 3. The ups again, from the now-empty database: the same migrations, in
    // the same order, which is what proves the downs left nothing behind.
    let second = install(&installer, db).await;
    println!("ups:    {} migrations again", second.applied);
    assert_eq!(
        second.applied, first.applied,
        "a rolled-back database installs exactly as an empty one does"
    );
    assert_eq!(
        second.modules, first.modules,
        "the modules install in the same dependency order both times"
    );

    // 4. And a fourth round that must change nothing: installing twice in a
    // row is what `rok-pharmacy --install` will do on every start.
    let third = install(&installer, db).await;
    assert_eq!(
        third.applied, 0,
        "an installed database applies nothing the second time"
    );
    println!("ups:    nothing left to apply");

    println!("migration-verifier: ok");
}
