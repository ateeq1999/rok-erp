//! Module manifests, with no database: what a module says, and the order the
//! modules go in.

use std::path::{Path, PathBuf};

use rok_pos_database::module_manifest::{ModuleError, discover, in_dependency_order};

fn modules_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../../database/modules")
}

/// A folder with one module in it, written from a manifest body.
fn scratch(modules: &[(&str, &str)]) -> PathBuf {
    let root = std::env::temp_dir().join(format!("rok-pos-manifests-{}", uuid::Uuid::now_v7()));
    for (key, manifest) in modules {
        let directory = root.join(key);
        std::fs::create_dir_all(directory.join("migrations"))
            .expect("the module folder can be made");
        std::fs::write(directory.join("module.toml"), manifest)
            .expect("the manifest can be written");
    }
    root
}

fn manifest(key: &str, depends_on: &[&str]) -> String {
    let depends = depends_on
        .iter()
        .map(|key| format!("\"{key}\""))
        .collect::<Vec<_>>()
        .join(", ");
    format!(
        "[module]\n\
         key = \"{key}\"\n\
         name = \"{key}\"\n\
         version = \"1.0.0\"\n\
         schema = \"{key}\"\n\
         category = \"platform\"\n\
         description = \"A module.\"\n\
         depends_on = [{depends}]\n\
         optional_depends_on = []\n\
         can_be_uninstalled = true\n"
    )
}

#[test]
fn every_shipped_module_is_readable_and_ordered_after_its_dependencies() {
    let sources = discover(&modules_root()).expect("every module manifest is readable");
    assert_eq!(
        sources.len(),
        25,
        "the database ships 25 modules and all of them are here"
    );

    let ordered = in_dependency_order(&sources).expect("the dependency order resolves");

    for (position, source) in ordered.iter().enumerate() {
        for dependency in &source.manifest.module.depends_on {
            let earlier = ordered[..position]
                .iter()
                .position(|done| done.key() == dependency)
                .unwrap_or_else(|| {
                    panic!(
                        "`{}` was installed after `{}`, which it depends on",
                        source.key(),
                        dependency
                    )
                });
            assert!(
                earlier < position,
                "`{}` comes after `{}`, which it depends on",
                source.key(),
                dependency
            );
        }
    }

    let keys: Vec<&str> = ordered.iter().map(|source| source.key()).collect();
    assert_eq!(
        keys.first(),
        Some(&"core"),
        "nothing can be installed before core, so core is first"
    );
}

#[test]
fn modules_that_depend_on_each_other_are_refused() {
    let root = scratch(&[
        ("alpha", &manifest("alpha", &["beta"])),
        ("beta", &manifest("beta", &["alpha"])),
    ]);

    let sources = discover(&root).expect("both manifests are readable");
    let error = in_dependency_order(&sources).expect_err("a circle cannot be installed");

    assert!(
        matches!(&error, ModuleError::DependencyCycle { path } if path.contains("alpha") && path.contains("beta")),
        "the error names the circle: {error}"
    );
}

#[test]
fn a_module_that_needs_something_missing_is_refused() {
    let root = scratch(&[("pharmacy", &manifest("pharmacy", &["core"]))]);

    let sources = discover(&root).expect("the manifest is readable");
    let error = in_dependency_order(&sources).expect_err("core is not here");

    assert!(
        matches!(&error, ModuleError::UnknownDependency { module, depends_on }
            if module == "pharmacy" && depends_on == "core"),
        "the error names what is missing: {error}"
    );
}

#[test]
fn a_manifest_that_disagrees_with_itself_is_refused() {
    let root = scratch(&[(
        "pharmacy",
        "[module]\n\
         key = \"pharmacy\"\n\
         name = \"Pharmacy\"\n\
         version = \"1.0.0\"\n\
         schema = \"clinic\"\n\
         category = \"industry_pack\"\n\
         description = \"Wrong schema.\"\n",
    )]);

    let sources = discover(&root).expect("the manifest is readable TOML");
    let error = in_dependency_order(&sources).expect_err("the schema is not the module's own");

    assert!(
        matches!(&error, ModuleError::InvalidManifest { module, .. } if module == "pharmacy"),
        "the error names the module: {error}"
    );
}

#[test]
fn a_permission_belonging_to_another_module_is_refused() {
    let root = scratch(&[(
        "pharmacy",
        "[module]\n\
         key = \"pharmacy\"\n\
         name = \"Pharmacy\"\n\
         version = \"1.0.0\"\n\
         schema = \"pharmacy\"\n\
         category = \"industry_pack\"\n\
         description = \"Wrong permission.\"\n\
         \n\
         [[permissions]]\n\
         key = \"catalog.products.view\"\n\
         description = \"Someone else's permission.\"\n",
    )]);

    let sources = discover(&root).expect("the manifest is readable TOML");
    let error = in_dependency_order(&sources).expect_err("the permission is catalog's");

    assert!(
        error.to_string().contains("catalog.products.view"),
        "the error quotes the permission: {error}"
    );
}

#[test]
fn an_unknown_key_does_not_stop_a_newer_manifest_from_loading() {
    let root = scratch(&[(
        "pharmacy",
        "[module]\n\
         key = \"pharmacy\"\n\
         name = \"Pharmacy\"\n\
         version = \"1.0.0\"\n\
         schema = \"pharmacy\"\n\
         category = \"industry_pack\"\n\
         description = \"Written by a newer installer.\"\n\
         something_added_later = 42\n",
    )]);

    let sources = discover(&root).expect("a key we do not know is ignored, not refused");
    assert_eq!(
        sources[0].manifest.module.description,
        "Written by a newer installer."
    );
}

#[test]
fn a_folder_without_a_manifest_is_not_a_module() {
    let root = scratch(&[("pharmacy", &manifest("pharmacy", &[]))]);
    std::fs::create_dir_all(root.join("notes")).expect("a plain folder can be made");

    let sources = discover(&root).expect("discovery skips folders without a manifest");

    assert_eq!(sources.len(), 1, "only pharmacy is a module");
}
