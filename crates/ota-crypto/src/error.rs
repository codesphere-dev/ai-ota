//! Errors produced while handling keys, signatures and digests.

/// Errors produced while handling keys, signatures and digests.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A signature did not verify against the message and key.
    #[error("signature verification failed")]
    Verification,
    /// A key blob had the wrong length.
    #[error("invalid key length: expected {expected}, found {found}")]
    InvalidKeyLength {
        /// Length the algorithm requires.
        expected: usize,
        /// Length that was supplied.
        found: usize,
    },
    /// A signature blob had the wrong length.
    #[error("invalid signature length: expected {expected}, found {found}")]
    InvalidSignatureLength {
        /// Length the algorithm requires.
        expected: usize,
        /// Length that was supplied.
        found: usize,
    },
    /// A hex string had an odd number of characters.
    #[error("invalid hex length: {0} characters (must be even)")]
    InvalidHexLength(usize),
    /// A hex string contained a character that is not a hex digit.
    #[error("invalid hex character: {0:#04x}")]
    InvalidHexByte(u8),
    /// A key file did not match the expected envelope.
    #[error("malformed key file")]
    MalformedKeyFile,
    /// A key file was empty.
    #[error("empty key file")]
    EmptyKeyFile,
}
