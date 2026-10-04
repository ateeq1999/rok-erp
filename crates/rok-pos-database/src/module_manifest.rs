//! What a module says about itself in `module.toml`, and the order modules go in.
//!
//! Every module under `database/modules/<key>/` carries a `module.toml`. The
//! installer reads them all, resolves `depends_on` into an order, and refuses a
//! cycle rather than installing half of one.
//!
//! ```
//! use std::path::PathBuf;
//!
//! use rok_pos_database::module_manifest::{ModuleManifest, ModuleSource, in_dependency_order};
//!
//! let manifest: ModuleManifest = r#"
//!     [module]
//!     key = "pharmacy"
//!     name = "Pharmacy"
//!     version = "1.0.0"
//!     schema = "pharmacy"
//!     category = "industry_pack"
//!     description = "Prescriptions and dispensing."
//!     depends_on = ["core", "catalog"]
//! "#
//! .parse()
//! .expect("the manifest is readable TOML");
//!
//! assert_eq!(manifest.module.key, "pharmacy");
//! assert!(
//!     in_dependency_order(&[ModuleSource {
//!         manifest: manifest.clone(),
//!         directory: PathBuf::from("pharmacy"),
//!     }])
//!     .is_err(),
//!     "core and catalog are not here, so the order cannot be resolved"
//! );
//! ```
//!
//! Unknown keys are ignored, so a manifest written for a newer installer still
//! loads. What the installer needs is checked: the schema must be named after
//! the module, and every permission must start with the module's key.

use std::collections::{BTreeMap, BTreeSet};
use std::fmt;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

/// Everything that can go wrong reading modules, ordering them or installing
/// them.
#[derive(Debug)]
pub enum ModuleError {
    /// A file or folder could not be read.
    Io {
        /// The path that failed.
        path: PathBuf,
        /// What the operating system said.
        source: std::io::Error,
    },
    /// A `module.toml` was not readable TOML, or did not have the expected shape.
    Manifest {
        /// The manifest that failed.
        path: PathBuf,
        /// What toml said.
        message: String,
    },
    /// Two modules claim the same key.
    DuplicateModule {
        /// The key both claim.
        key: String,
        /// Where the second one was found.
        path: PathBuf,
    },
    /// A module depends on a module that is not in the folder.
    UnknownDependency {
        /// The module that asked.
        module: String,
        /// The dependency that is missing.
        depends_on: String,
    },
    /// Two modules depend on each other, directly or through others.
    DependencyCycle {
        /// The cycle, first key repeated at the end, e.g. `a -> b -> a`.
        path: String,
    },
    /// A manifest disagreed with itself.
    InvalidManifest {
        /// The module that is wrong.
        module: String,
        /// What is wrong with it.
        message: String,
    },
    /// A migration that was recorded as applied is not there any more.
    MissingMigration {
        /// The module that lost a migration.
        module: String,
        /// The migration name that is gone.
        migration: String,
    },
    /// A migration that shipped has changed since it was applied. Editing one
    /// that a database already ran leaves that database quietly wrong, so this
    /// stops instead: add a new migration.
    ChecksumChanged {
        /// The module the migration belongs to.
        module: String,
        /// The migration name.
        migration: String,
        /// The checksum recorded when it was applied.
        recorded: String,
        /// The checksum of the file as it is now.
        found: String,
    },
    /// The database said no.
    Database(rok_db::Error),
}

impl fmt::Display for ModuleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io { path, source } => {
                write!(f, "cannot read {}: {source}", path.display())
            }
            Self::Manifest { path, message } => {
                write!(
                    f,
                    "{} is not a readable module manifest: {message}",
                    path.display()
                )
            }
            Self::DuplicateModule { key, path } => {
                write!(
                    f,
                    "module `{key}` is defined twice, again at {}",
                    path.display()
                )
            }
            Self::UnknownDependency { module, depends_on } => {
                write!(
                    f,
                    "module `{module}` depends on `{depends_on}`, which is not installed"
                )
            }
            Self::DependencyCycle { path } => {
                write!(f, "modules depend on each other in a circle: {path}")
            }
            Self::InvalidManifest { module, message } => {
                write!(f, "module `{module}` cannot be installed: {message}")
            }
            Self::MissingMigration { module, migration } => write!(
                f,
                "module `{module}` recorded `{migration}` as applied, but the file is gone"
            ),
            Self::ChecksumChanged {
                module,
                migration,
                recorded,
                found,
            } => write!(
                f,
                "module `{module}` migration `{migration}` has changed since it was applied \
                 (recorded {recorded}, found {found}); add a new migration instead of editing it"
            ),
            Self::Database(source) => write!(f, "{source}"),
        }
    }
}

impl std::error::Error for ModuleError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io { source, .. } => Some(source),
            Self::Database(source) => Some(source),
            _ => None,
        }
    }
}

impl From<rok_db::Error> for ModuleError {
    fn from(source: rok_db::Error) -> Self {
        Self::Database(source)
    }
}

impl From<sqlx::Error> for ModuleError {
    fn from(source: sqlx::Error) -> Self {
        Self::Database(rok_db::Error::Database(source))
    }
}

/// One module's `module.toml`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    /// The `[module]` table: who the module is and what it needs.
    #[serde(rename = "module")]
    pub module: ModuleInfo,
    /// `[[permissions]]`: what a person may do, checked by key.
    #[serde(default)]
    pub permissions: Vec<Permission>,
    /// `[[pages]]`: screens the module adds.
    #[serde(default)]
    pub pages: Vec<Page>,
    /// `[[default_roles]]`: the roles a business gets when it switches the module on.
    #[serde(default, rename = "default_roles")]
    pub default_roles: Vec<DefaultRole>,
    /// `[[settings]]`: values written into `core.settings`.
    #[serde(default)]
    pub settings: Vec<Setting>,
    /// `[[number_sequences]]`: document numbers such as `RX-`.
    #[serde(default, rename = "number_sequences")]
    pub number_sequences: Vec<NumberSequence>,
    /// `[[assistant_rules]]`: when Msaidizi speaks up.
    #[serde(default, rename = "assistant_rules")]
    pub assistant_rules: Vec<AssistantRule>,
}

impl ModuleManifest {
    /// Parse a manifest from its TOML text.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError::Manifest`] when the text is not a manifest this
    /// version understands.
    pub fn parse(text: &str, path: &Path) -> Result<Self, ModuleError> {
        toml::from_str(text).map_err(|error| ModuleError::Manifest {
            path: path.to_path_buf(),
            message: error.message().to_string(),
        })
    }

    /// The module's key, which is also the name of the schema it owns.
    #[must_use]
    pub fn key(&self) -> &str {
        &self.module.key
    }

    /// Check the manifest against the conventions every module follows.
    ///
    /// # Errors
    ///
    /// Returns [`ModuleError::InvalidManifest`] when the schema is not named
    /// after the module, or a permission is keyed to another module, or one
    /// permission is declared twice.
    pub fn validate(&self) -> Result<(), ModuleError> {
        let wrong = |message: String| ModuleError::InvalidManifest {
            module: self.module.key.clone(),
            message,
        };

        if self.module.schema != self.module.key {
            return Err(wrong(format!(
                "schema is `{}` but the module is `{}`; a module owns the schema named after it",
                self.module.schema, self.module.key
            )));
        }

        let mut seen = BTreeSet::new();
        for permission in &self.permissions {
            if !permission.key.starts_with(&format!("{}.", self.module.key)) {
                return Err(wrong(format!(
                    "permission `{}` does not start with the module key",
                    permission.key
                )));
            }
            if !seen.insert(&permission.key) {
                return Err(wrong(format!(
                    "permission `{}` is declared twice",
                    permission.key
                )));
            }
        }

        Ok(())
    }
}

impl std::str::FromStr for ModuleManifest {
    type Err = ModuleError;

    /// Parse a manifest, with no path to blame if it fails.
    fn from_str(text: &str) -> Result<Self, Self::Err> {
        Self::parse(text, Path::new("module.toml"))
    }
}

/// The `[module]` table.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// The module's key: `pharmacy`, `point_of_sale`, `core`. Also its schema.
    pub key: String,
    /// The name a person sees.
    pub name: String,
    /// The version of the module's migrations.
    pub version: String,
    /// The PostgreSQL schema the module owns.
    pub schema: String,
    /// Which shelf it sits on: `platform`, `core_retail`, `industry_pack`.
    pub category: String,
    /// One line saying what the module does.
    pub description: String,
    /// The business types it is offered to, when it is not for everyone.
    #[serde(default, rename = "business_types")]
    pub business_types: Vec<String>,
    /// Modules that must be installed first.
    #[serde(default, rename = "depends_on")]
    pub depends_on: Vec<String>,
    /// Modules it works with if they are there.
    #[serde(default, rename = "optional_depends_on")]
    pub optional_depends_on: Vec<String>,
    /// Whether a business may switch it off again.
    #[serde(default, rename = "can_be_uninstalled")]
    pub can_be_uninstalled: bool,
}

/// One `[[permissions]]` entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Permission {
    /// The key a screen checks, such as `pharmacy.controlled_register.record`.
    pub key: String,
    /// What holding it allows.
    pub description: String,
    /// How careful a reviewer has to be. Defaults to `normal`.
    #[serde(default)]
    pub risk_level: RiskLevel,
}

/// How much damage a permission could do if it were misused.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RiskLevel {
    /// Reading and editing ordinary records.
    #[default]
    Normal,
    /// Money, controlled drugs, patient records: the audit trail matters.
    Sensitive,
    /// Changes what a customer is charged or paid.
    Money,
}

impl RiskLevel {
    /// The text stored in `core.permissions.risk_level`.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Normal => "normal",
            Self::Sensitive => "sensitive",
            Self::Money => "money",
        }
    }
}

/// One `[[pages]]` entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    /// The key the router and layouts use.
    pub page_key: String,
    /// The name on the tab.
    pub title: String,
    /// Where the screen lives.
    pub route: String,
}

/// One `[[default_roles]]` entry: a role a business gets, and what it may do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DefaultRole {
    /// Which role: `manager`, `cashier`, `accountant`, or one the module invents.
    pub role_key: String,
    /// The name to show, when the module brings its own role along.
    #[serde(default)]
    pub name: Option<String>,
    /// What the role may do.
    pub permissions: Vec<String>,
}

/// One `[[settings]]` entry, written into `core.settings` when a business
/// switches the module on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    /// The setting's key.
    pub key: String,
    /// Its value: a string, a number, a switch or a list.
    pub value: toml::Value,
}

/// One `[[number_sequences]]` entry: how document numbers are built.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NumberSequence {
    /// Which sequence, such as `pharmacy.prescription`.
    pub sequence_key: String,
    /// The text before the number, such as `RX-`.
    pub prefix: String,
    /// How many digits the number has.
    #[serde(default, rename = "padding_length")]
    pub padding_length: u32,
    /// `never`, `yearly`, `monthly` or `daily`.
    #[serde(default, rename = "reset_period")]
    pub reset_period: String,
}

/// One `[[assistant_rules]]` entry: when Msaidizi speaks up on a page.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantRule {
    /// The page the rule watches.
    pub page_key: String,
    /// The module the rule belongs to, when it is not the same as the page's.
    #[serde(default, rename = "module_key")]
    pub module_key: Option<String>,
    /// The number to watch.
    #[serde(default, rename = "trigger_metric_key")]
    pub trigger_metric_key: String,
    /// How the number is compared, such as `greater_than`.
    #[serde(default, rename = "trigger_comparison")]
    pub trigger_comparison: String,
    /// The number it is compared with.
    #[serde(default, rename = "trigger_threshold")]
    pub trigger_threshold: Option<toml::Value>,
    /// What Msaidizi says.
    #[serde(default, rename = "message_template")]
    pub message_template: String,
    /// The same message in other languages, keyed by language code.
    #[serde(default, rename = "message_translations")]
    pub message_translations: BTreeMap<String, String>,
    /// Which roles are shown the nudge.
    #[serde(default, rename = "visible_to_role_keys")]
    pub visible_to_role_keys: Vec<String>,
    /// How often it may speak up, such as `once_per_day`.
    #[serde(default)]
    pub frequency: String,
    /// The buttons under the message.
    #[serde(default, rename = "allowed_actions")]
    pub allowed_actions: Vec<toml::Value>,
}

/// A module and the folder it was read from.
#[derive(Debug, Clone)]
pub struct ModuleSource {
    /// What its manifest says.
    pub manifest: ModuleManifest,
    /// The module's folder, the one holding `migrations/`.
    pub directory: PathBuf,
}

impl ModuleSource {
    /// The module's key, which is also the name of the schema it owns.
    #[must_use]
    pub fn key(&self) -> &str {
        self.manifest.module.key.as_str()
    }

    /// Where this module's migrations live.
    #[must_use]
    pub fn migrations_directory(&self) -> PathBuf {
        self.directory.join("migrations")
    }
}

/// Read every module under `root`, one folder per module.
///
/// Folders without a `module.toml` are skipped: the root also holds
/// `MODULE_MAP.md` and may hold notes.
///
/// # Errors
///
/// Returns [`ModuleError::Manifest`] when a manifest cannot be read, and
/// [`ModuleError::DuplicateModule`] when two folders claim the same key.
pub fn discover(root: &Path) -> Result<Vec<ModuleSource>, ModuleError> {
    let entries = std::fs::read_dir(root).map_err(|source| ModuleError::Io {
        path: root.to_path_buf(),
        source,
    })?;

    let mut sources: Vec<ModuleSource> = Vec::new();
    for entry in entries {
        let directory = entry
            .map_err(|source| ModuleError::Io {
                path: root.to_path_buf(),
                source,
            })?
            .path();
        if !directory.is_dir() {
            continue;
        }
        let manifest_path = directory.join("module.toml");
        if !manifest_path.is_file() {
            continue;
        }

        let text = std::fs::read_to_string(&manifest_path).map_err(|source| ModuleError::Io {
            path: manifest_path.clone(),
            source,
        })?;
        let manifest = ModuleManifest::parse(&text, &manifest_path)?;
        sources.push(ModuleSource {
            manifest,
            directory,
        });
    }

    sources.sort_by(|left, right| left.key().cmp(right.key()));
    for (index, source) in sources.iter().enumerate() {
        let first = &sources[..index]
            .iter()
            .find(|earlier| earlier.key() == source.key())
            .map_or_else(String::new, |earlier| {
                earlier.directory.display().to_string()
            });
        if !first.is_empty() {
            return Err(ModuleError::DuplicateModule {
                key: source.key().to_string(),
                path: source.directory.clone(),
            });
        }
    }

    Ok(sources)
}

/// Sort modules so every module comes after the ones it depends on.
///
/// Ties are broken by key, so the order is the same on every machine and a
/// second install runs the same migrations in the same sequence.
///
/// # Errors
///
/// Returns [`ModuleError::UnknownDependency`] when a dependency is missing, and
/// [`ModuleError::DependencyCycle`] when modules depend on each other in a
/// circle.
pub fn in_dependency_order(sources: &[ModuleSource]) -> Result<Vec<&ModuleSource>, ModuleError> {
    let known: BTreeMap<&str, &ModuleSource> = sources
        .iter()
        .map(|source| (source.key(), source))
        .collect();
    for source in sources {
        source.manifest.validate()?;
        for dependency in &source.manifest.module.depends_on {
            if !known.contains_key(dependency.as_str()) {
                return Err(ModuleError::UnknownDependency {
                    module: source.key().to_string(),
                    depends_on: dependency.clone(),
                });
            }
        }
    }

    let mut placed: Vec<&ModuleSource> = Vec::new();
    let mut waiting: Vec<&ModuleSource> = sources.iter().collect();
    while !waiting.is_empty() {
        let next = waiting.iter().position(|source| {
            source
                .manifest
                .module
                .depends_on
                .iter()
                .all(|dependency| placed.iter().any(|done| done.key() == dependency))
        });

        let Some(index) = next else {
            let remaining: Vec<&str> = waiting.iter().map(|source| source.key()).collect();
            return Err(ModuleError::DependencyCycle {
                path: find_cycle(sources, &remaining).unwrap_or_else(|| remaining.join(" -> ")),
            });
        };
        placed.push(waiting.remove(index));
    }

    Ok(placed)
}

/// Walk the remaining modules to name the circle, so the error can show it.
fn find_cycle(sources: &[ModuleSource], remaining: &[&str]) -> Option<String> {
    let mut walking: Vec<&str> = Vec::new();
    for start in remaining {
        if let Some(cycle) = visit(sources, remaining, start, &mut walking) {
            return Some(cycle);
        }
    }
    None
}

fn visit<'a>(
    sources: &'a [ModuleSource],
    remaining: &[&str],
    key: &'a str,
    walking: &mut Vec<&'a str>,
) -> Option<String> {
    if let Some(start) = walking.iter().position(|seen| *seen == key) {
        let mut cycle: Vec<&str> = walking[start..].to_vec();
        cycle.push(key);
        return Some(cycle.join(" -> "));
    }

    let source = sources.iter().find(|source| source.key() == key)?;
    walking.push(key);
    let mut found = None;
    for dependency in &source.manifest.module.depends_on {
        if remaining.contains(&dependency.as_str()) {
            found = visit(sources, remaining, dependency.as_str(), walking);
            if found.is_some() {
                break;
            }
        }
    }
    walking.pop();
    found
}
