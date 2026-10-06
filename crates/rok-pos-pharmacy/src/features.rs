//! The pharmacy's features, one folder each.
//!
//! A feature owns its domain, its data access, its state machine, its screen
//! and its tests. Nothing outside a feature needs its widgets, and no feature
//! reaches into another feature's presentation.
//!
//! The pieces every screen is built from live in [`shared::board`], not in the
//! screens module. A feature's presentation imports `crate::features::shared::board`,
//! never `crate::screens::board`; the boundary tests at the bottom of this file
//! enforce that until the screens module is gone.

pub mod clinical_check;
pub mod dashboard;
pub mod licences;
pub mod patient_profile;
pub mod prescriptions;
pub mod reports;
pub mod shared;

pub use clinical_check::ClinicalCheckPage;
pub use dashboard::DashboardPage;
pub use licences::LicencesPage;
pub use patient_profile::PatientListPage;
pub use patient_profile::PatientProfilePage;
pub use prescriptions::PrescriptionsPage;
pub use reports::ReportsPage;

#[cfg(test)]
mod boundary_tests {
    use std::path::Path;

    /// Returns every feature's presentation source: its path relative to this
    /// crate's manifest directory, and its body.
    fn presentation_sources() -> Vec<(std::path::PathBuf, String)> {
        let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
        let features = manifest_dir.join("src").join("features");
        let mut sources = Vec::new();
        let mut entries = vec![features];
        while let Some(dir) = entries.pop() {
            let Ok(read) = std::fs::read_dir(&dir) else {
                continue;
            };
            for entry in read.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    entries.push(path);
                } else if path.extension().is_some_and(|ext| ext == "rs")
                    && path
                        .components()
                        .any(|component| component.as_os_str() == "presentation")
                {
                    let relative = path
                        .strip_prefix(manifest_dir)
                        .expect("sources live under the manifest dir");
                    let body = std::fs::read_to_string(&path)
                        .unwrap_or_else(|error| panic!("read {}: {error}", relative.display()));
                    sources.push((relative.to_path_buf(), body));
                }
            }
        }
        sources.sort();
        sources
    }

    /// The feature a presentation source belongs to, such as `prescriptions`.
    /// `None` when the source sits outside any feature, as `shared` does.
    fn feature_of(source: &Path) -> Option<&str> {
        let mut components = source.components();
        while let Some(component) = components.next() {
            if component.as_os_str() == "features" {
                return components.next().and_then(|next| next.as_os_str().to_str());
            }
        }
        None
    }

    #[test]
    fn a_features_presentation_imports_the_shared_board_not_the_screens_board() {
        let sources = presentation_sources();
        let offenders: Vec<String> = sources
            .iter()
            .filter(|(_, body)| body.contains("crate::screens::board"))
            .map(|(path, _)| path.display().to_string())
            .collect();
        assert_eq!(
            offenders.len(),
            0,
            "use features::shared::board: {offenders:?}"
        );
    }

    #[test]
    fn a_features_presentation_imports_no_other_feature() {
        let sources = presentation_sources();
        let offenders: Vec<String> = sources
            .iter()
            .filter_map(|(path, body)| {
                let feature = feature_of(path)?;
                let imports_other = body
                    .lines()
                    .filter_map(|line| line.trim().strip_prefix("use crate::features::"))
                    .filter_map(|rest| rest.split("::").next())
                    .any(|imported| imported != feature && imported != "shared");
                imports_other.then(|| format!("{} imports another feature", path.display()))
            })
            .collect();
        assert_eq!(
            offenders.len(),
            0,
            "features may not import each other: {offenders:?}"
        );
    }
}
