//! Where the licences board is read from.

use std::future::Future;

use crate::features::licences::domain::entities::Licences;

/// A read that did not work. The source's own error stops here.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RepositoryError {
    /// The source could not be reached.
    Unavailable(String),
    /// The source answered with something this feature cannot read.
    Unexpected(String),
}

/// The pharmacy's licences, readiness and document folder, as one reading.
pub trait LicenceRepository: Send {
    /// Read the folder.
    fn get_licences(&self) -> impl Future<Output = Result<Licences, RepositoryError>> + Send;
}
