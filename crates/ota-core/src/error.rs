use alloc::string::String;

/// Errors returned by the core types.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A version string could not be parsed as `MAJOR.MINOR.PATCH[-PRERELEASE]`.
    #[error("invalid version string: {0:?}")]
    InvalidVersion(String),
}
