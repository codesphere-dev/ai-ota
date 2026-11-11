//! Errors produced while building, parsing or validating a package.

/// Errors produced while building, parsing or validating an `.otapkg` package.
#[derive(Debug, thiserror::Error, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// The byte stream ended before a complete value could be read.
    #[error("package is truncated")]
    Truncated,
    /// The leading magic bytes were not `OTAPKG01`.
    #[error("bad package magic")]
    BadMagic,
    /// The package advertises a format version this build cannot read.
    #[error("unsupported package format version: {0}")]
    UnsupportedVersion(u16),
    /// The header length is smaller than the minimum fixed header.
    #[error("header length {found} is smaller than the minimum {min}")]
    HeaderTooShort {
        /// Header length found on the wire.
        found: u16,
        /// Minimum accepted header length.
        min: u16,
    },
    /// The header length is larger than the reader accepts.
    #[error("header length {found} exceeds the maximum {max}")]
    HeaderTooLong {
        /// Header length found on the wire.
        found: u16,
        /// Maximum accepted header length.
        max: u16,
    },
    /// The header checksum does not match its contents.
    #[error("header CRC32 mismatch (stored {stored:#010x}, computed {computed:#010x})")]
    HeaderCrcMismatch {
        /// Checksum stored in the header.
        stored: u32,
        /// Checksum recomputed from the header bytes.
        computed: u32,
    },
    /// A section carried a tag this build does not know.
    #[error("unknown section tag {0:#04x}")]
    UnknownSectionTag(u8),
    /// The manifest section is missing.
    #[error("package has no manifest section")]
    MissingManifest,
    /// More than one manifest section was found.
    #[error("package has more than one manifest section")]
    DuplicateManifest,
    /// A section appeared before the manifest.
    #[error("the manifest section must be the first section")]
    ManifestNotFirst,
    /// The signature section is not the last section.
    #[error("the signature section must be the last section")]
    SignatureNotLast,
    /// Bytes remained after the last section.
    #[error("unexpected trailing data after the last section")]
    TrailingData,
    /// The number of chunks disagrees with the manifest.
    #[error("chunk count mismatch (manifest {expected}, found {found})")]
    ChunkCountMismatch {
        /// Chunk count declared by the manifest.
        expected: u32,
        /// Chunk count actually present.
        found: u32,
    },
    /// Chunk indices are not sequential.
    #[error("chunk index out of order (expected {expected}, found {found})")]
    ChunkIndexOutOfOrder {
        /// Index expected at this position.
        expected: u32,
        /// Index found instead.
        found: u32,
    },
    /// The summed chunk lengths disagree with the manifest.
    #[error("payload length mismatch (manifest {expected}, found {found})")]
    PayloadLengthMismatch {
        /// Payload length declared by the manifest.
        expected: u64,
        /// Payload length actually present.
        found: u64,
    },
    /// A chunk failed its `CRC32` check.
    #[error("chunk {index} CRC32 mismatch (stored {stored:#010x}, computed {computed:#010x})")]
    ChunkCrcMismatch {
        /// Index of the corrupt chunk.
        index: u32,
        /// Checksum stored in the chunk.
        stored: u32,
        /// Checksum recomputed from the chunk data.
        computed: u32,
    },
    /// The reassembled payload failed its `SHA-256` check.
    #[error("payload SHA-256 mismatch")]
    PayloadHashMismatch,
    /// A package string was not valid `UTF-8`.
    #[error("invalid UTF-8 in package string")]
    InvalidUtf8,
    /// A package string exceeded the `u16` length prefix.
    #[error("string too long ({len} bytes, maximum {max})")]
    StringTooLong {
        /// Length of the offending string.
        len: usize,
        /// Maximum length the format allows.
        max: usize,
    },
    /// The manifest carried an unknown quantization code.
    #[error("invalid quantization code: {0}")]
    InvalidQuantization(u16),
    /// The signature section carried an unknown algorithm code.
    #[error("unsupported signature algorithm: {0}")]
    UnsupportedAlgorithm(u8),
    /// The signature blob exceeded the `u16` length prefix.
    #[error("signature too long (maximum {0} bytes)")]
    SignatureTooLong(usize),
    /// A section body exceeded the `u32` length prefix.
    #[error("section body too long")]
    SectionTooLong,
    /// A signature was appended to a package that already had one.
    #[error("package is already signed")]
    AlreadySigned,
}
