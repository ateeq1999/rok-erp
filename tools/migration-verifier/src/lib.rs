//! The verification itself, kept beside the binary so it can be tested.

use std::fmt;
use std::path::PathBuf;

use rok_db::testing::TestDb;
use rok_db::{Db, raw};
use rok_pos_database::module_installer::{InstallRequest, install, revert_all};
use rok_pos_database::module_manifest::ModuleError;

/// How to run, from the command line.
#[derive(Debug, Clone)]
pub struct Options {
    /// Where the modules live.
    pub modules_root: PathBuf,
    /// Only these module keys, or empty for all of them.
    pub only: Vec<String>,
}

impl Default for Options {
    fn default() -> Self {
        Self {
            modules_root: PathBuf::from("database/modules"),
            only: Vec::new(),
        }
    }
}

impl Options {
    /// Read `--modules <folder>` and `--only <key,key>`, or take the defaults.
    ///
    /// # Errors
    ///
    /// Returns the message to show when an argument is not understood.
    pub fn from_args<I: IntoIterator<Item = String>>(args: I) -> Result<Self, String> {
        let mut options = Self::default();
        let mut args = args.into_iter();
        while let Some(argument) = args.next() {
            match argument.as_str() {
                "--modules" => {
                    options.modules_root = PathBuf::from(
                        args.next()
                            .ok_or_else(|| "--modules needs a folder".to_string())?,
                    );
                }
                "--only" => {
                    let list = args
                        .next()
                        .ok_or_else(|| "--only needs a comma separated list".to_string())?;
                    options.only = list
                        .split(',')
                        .map(str::trim)
                        .filter(|key| !key.is_empty())
                        .map(str::to_string)
                        .collect();
                }
                "--help" | "-h" => return Err(usage()),
                other => return Err(format!("unknown argument `{other}`\n\n{}", usage())),
            }
        }
        Ok(options)
    }
}

fn usage() -> String {
    "usage: migration-verifier [--modules <folder>] [--only <key,key>]".to_string()
}

/// Everything that can go wrong, including the two ways a schema can be wrong
/// without any statement failing.
#[derive(Debug)]
pub enum VerifyError {
    /// A manifest could not be read, or a migration failed.
    Modules(ModuleError),
    /// There was nothing to install, so nothing was proved.
    NoModules(PathBuf),
    /// The down migrations ran but left schemas behind.
    Leftover(Vec<String>),
    /// The second build had nothing to do, so the first rollback did not take.
    NothingRebuilt,
}

impl fmt::Display for VerifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Modules(error) => write!(f, "{error}"),
            Self::NoModules(path) => write!(
                f,
                "no modules were found under {}, so nothing was verified",
                path.display()
            ),
            Self::Leftover(schemas) => write!(
                f,
                "the down migrations left {} behind, so a module cannot be rolled back cleanly: {}",
                schemas.len(),
                schemas.join(", ")
            ),
            Self::NothingRebuilt => write!(
                f,
                "the schema did not build again after being rolled back, so the down migrations \
                 did not do their job"
            ),
        }
    }
}

impl std::error::Error for VerifyError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Modules(error) => Some(error),
            _ => None,
        }
    }
}

impl From<ModuleError> for VerifyError {
    fn from(error: ModuleError) -> Self {
        Self::Modules(error)
    }
}

impl From<rok_db::Error> for VerifyError {
    fn from(error: rok_db::Error) -> Self {
        Self::Modules(ModuleError::from(error))
    }
}

/// Build everything, take it down, build it again, and prove nothing was left
/// behind.
///
/// # Errors
///
/// Returns the first thing that went wrong: a manifest that cannot be read, a
/// migration that fails, a schema left behind, or a second build with nothing to
/// do.
pub async fn verify(options: Options) -> Result<Summary, VerifyError> {
    let Some(test_db) = TestDb::create().await? else {
        return Ok(Summary::Skipped);
    };
    let db = test_db.db();
    let request = request(&options);

    let first = install(db, &request).await?;
    if first.modules.is_empty() {
        return Err(VerifyError::NoModules(options.modules_root.clone()));
    }

    let installed: i64 = raw("select count(*) from core.installed_modules")
        .scalar(db)
        .await?;
    let permissions: i64 = raw("select count(*) from core.permissions")
        .scalar(db)
        .await?;

    let taken_off = revert_all(db, &options.modules_root).await?;
    let left = schemas_left(db).await?;

    let second = install(db, &request).await?;

    Ok(Summary::Verified {
        modules: first.modules.len(),
        migrations: first.migrations_applied.len(),
        permissions,
        installed_modules: installed,
        rolled_back: taken_off.len(),
        schemas_left: left,
        rebuilt: second.migrations_applied.len(),
    })
}

fn request(options: &Options) -> InstallRequest {
    let keys: Vec<&str> = options.only.iter().map(String::as_str).collect();
    InstallRequest::all(&options.modules_root)
        .without_jobs()
        .only(&keys)
}

/// Every schema that is not PostgreSQL's own.
async fn schemas_left(db: &Db) -> Result<Vec<String>, ModuleError> {
    let rows: Vec<(String,)> = raw("select schema_name from information_schema.schemata \
         where schema_name not in ('public', 'information_schema', 'pg_catalog', 'pg_toast') \
         order by schema_name")
    .fetch_all(db)
    .await?;
    Ok(rows.into_iter().map(|row| row.0).collect())
}

/// What the verification found.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Summary {
    /// There was no `DATABASE_URL`, so nothing was checked.
    Skipped,
    /// Everything held.
    Verified {
        /// How many modules were built.
        modules: usize,
        /// How many migrations ran the first time.
        migrations: usize,
        /// How many permissions the manifests declared.
        permissions: i64,
        /// How many rows `core.installed_modules` held.
        installed_modules: i64,
        /// How many down migrations ran.
        rolled_back: usize,
        /// Which schemas survived the down migrations, which must be none.
        schemas_left: Vec<String>,
        /// How many migrations ran the second time, which must be the same as
        /// the first time.
        rebuilt: usize,
    },
}

impl Summary {
    /// Refuse a verification that proved nothing, so the binary exits non-zero.
    ///
    /// # Errors
    ///
    /// Returns [`VerifyError::Leftover`] when a schema survived the down
    /// migrations, and [`VerifyError::NothingRebuilt`] when the second build had
    /// nothing to do.
    pub fn into_verified(self) -> Result<Self, VerifyError> {
        let Self::Verified {
            migrations,
            schemas_left,
            rebuilt,
            ..
        } = &self
        else {
            return Ok(self);
        };

        if !schemas_left.is_empty() {
            return Err(VerifyError::Leftover(schemas_left.clone()));
        }
        if *rebuilt != *migrations {
            return Err(VerifyError::NothingRebuilt);
        }
        Ok(self)
    }
}

impl fmt::Display for Summary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Skipped => write!(
                f,
                "DATABASE_URL is not set, so no database was built and nothing was verified"
            ),
            Self::Verified {
                modules,
                migrations,
                permissions,
                installed_modules,
                rolled_back,
                rebuilt,
                ..
            } => write!(
                f,
                "OK: {modules} modules, {migrations} migrations and {permissions} permissions \
                 applied, {installed_modules} modules recorded, {rolled_back} rolled back with \
                 nothing left behind, and {rebuilt} migrations applied again"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arguments_are_read_or_refused() {
        let options = Options::from_args(["--modules".to_string(), "db/modules".to_string()])
            .expect("a folder is given");
        assert_eq!(options.modules_root, PathBuf::from("db/modules"));

        let only =
            Options::from_args(["--only".to_string(), "core, pharmacy ,".to_string()]).unwrap();
        assert_eq!(
            only.only,
            vec!["core", "pharmacy"],
            "keys are trimmed and empty ones dropped"
        );

        assert_eq!(
            Options::from_args(Vec::<String>::new())
                .unwrap()
                .modules_root,
            PathBuf::from("database/modules"),
            "the default is where the modules live"
        );
        assert!(Options::from_args(["--wat".to_string()]).is_err());
        assert!(Options::from_args(["--modules".to_string()]).is_err());
    }

    /// A run that leaves a schema behind is a failure, not a summary.
    #[test]
    fn a_leftover_schema_is_refused() {
        let leftover = Summary::Verified {
            modules: 25,
            migrations: 36,
            permissions: 146,
            installed_modules: 25,
            rolled_back: 30,
            schemas_left: vec!["pharmacy".to_string()],
            rebuilt: 36,
        };

        let error = leftover
            .clone()
            .into_verified()
            .expect_err("a leftover schema is not a pass");
        assert!(
            matches!(error, VerifyError::Leftover(ref schemas) if schemas == &["pharmacy"]),
            "the error names the schema: {error}"
        );
        assert!(error.to_string().contains("pharmacy"));
    }

    /// A second build with nothing to do means the rollback did not take.
    #[test]
    fn a_second_build_with_nothing_to_do_is_refused() {
        let summary = Summary::Verified {
            modules: 25,
            migrations: 36,
            permissions: 146,
            installed_modules: 25,
            rolled_back: 36,
            schemas_left: Vec::new(),
            rebuilt: 0,
        };

        assert!(matches!(
            summary.into_verified(),
            Err(VerifyError::NothingRebuilt)
        ));
    }

    /// A clean run passes and reads as clean.
    #[test]
    fn a_clean_verification_says_so() {
        let summary = Summary::Verified {
            modules: 25,
            migrations: 36,
            permissions: 146,
            installed_modules: 25,
            rolled_back: 36,
            schemas_left: Vec::new(),
            rebuilt: 36,
        };

        let summary = summary.into_verified().expect("a clean run passes");
        assert!(summary.to_string().contains("OK"), "{summary}");
    }

    #[test]
    fn skipping_is_not_failing() {
        let summary = Summary::Skipped
            .into_verified()
            .expect("no database, no verdict");
        assert!(summary.to_string().contains("DATABASE_URL"), "{summary}");
    }
}
