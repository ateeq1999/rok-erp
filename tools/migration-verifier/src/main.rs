//! Proves that every module's migrations can be applied, rolled back and applied
//! again, on a PostgreSQL nobody is using.
//!
//! It builds a throwaway database from `DATABASE_URL`, runs every module's up
//! migrations in dependency order, runs every down migration in reverse, checks
//! that nothing was left behind, and builds it all again. Any failure is
//! returned as an error, which the binary turns into a non-zero exit: this is
//! what CI runs on every pull request.
//!
//! Without `DATABASE_URL` it says so and verifies nothing, so a checkout with no
//! PostgreSQL still builds.

use migration_verifier::{Options, Summary, verify};

#[tokio::main]
async fn main() -> std::process::ExitCode {
    let options = match Options::from_args(std::env::args().skip(1)) {
        Ok(options) => options,
        Err(error) => {
            eprintln!("migration-verifier: {error}");
            return std::process::ExitCode::FAILURE;
        }
    };

    match verify(options).await.and_then(Summary::into_verified) {
        Ok(summary) => {
            println!("{summary}");
            std::process::ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("migration-verifier: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
