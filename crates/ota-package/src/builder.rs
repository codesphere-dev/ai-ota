//! Assemble `.otapkg` files in memory.

use alloc::string::String;
use alloc::vec::Vec;

use ota_core::Version;

use crate::chunk::PayloadChunk;
use crate::codec::sha256;
use crate::error::Error;
use crate::header::Header;
use crate::manifest::{Manifest, Quantization};
use crate::reader::Package;
use crate::signature::{Signature, SignatureAlgorithm};
use crate::tlv::{write_section, TAG_MANIFEST, TAG_PAYLOAD_CHUNK, TAG_SIGNATURE};

/// Default payload chunk size: 64 `KiB`.
pub const DEFAULT_CHUNK_SIZE: u32 = 64 * 1024;

/// Builds an unsigned `.otapkg` package from a model payload.
///
/// ```
/// use ota_core::Version;
/// use ota_package::{PackageBuilder, Quantization};
///
/// let bytes = PackageBuilder::new("vision-v3", Version::new(1, 0, 0), "stm32h7", Quantization::Int8)
///     .with_payload(vec![0u8; 1024])
///     .build_unsigned()
///     .expect("valid package");
/// assert!(!bytes.is_empty());
/// ```
#[derive(Debug, Clone)]
pub struct PackageBuilder {
    model_name: String,
    model_version: Version,
    target: String,
    quantization: Quantization,
    chunk_size: u32,
    payload: Vec<u8>,
}

impl PackageBuilder {
    /// Start a package for `model_name` at `model_version`, targeting `target`.
    #[must_use]
    pub fn new(
        model_name: impl Into<String>,
        model_version: Version,
        target: impl Into<String>,
        quantization: Quantization,
    ) -> Self {
        Self {
            model_name: model_name.into(),
            model_version,
            target: target.into(),
            quantization,
            chunk_size: DEFAULT_CHUNK_SIZE,
            payload: Vec::new(),
        }
    }

    /// Override the payload chunk size. Values below one byte are clamped to one.
    #[must_use]
    pub fn with_chunk_size(mut self, chunk_size: u32) -> Self {
        self.chunk_size = chunk_size.max(1);
        self
    }

    /// Attach the payload bytes.
    #[must_use]
    pub fn with_payload(mut self, payload: impl Into<Vec<u8>>) -> Self {
        self.payload = payload.into();
        self
    }

    /// Number of chunks the payload will be split into.
    #[must_use]
    pub fn chunk_count(&self) -> u32 {
        if self.payload.is_empty() {
            return 0;
        }
        let size = usize::try_from(self.chunk_size)
            .unwrap_or(usize::MAX)
            .max(1);
        u32::try_from(self.payload.len().div_ceil(size)).unwrap_or(u32::MAX)
    }

    /// Build a complete, unsigned package.
    ///
    /// The optional signature section is neither written nor counted in the
    /// header, so the result is a valid unsigned package.
    ///
    /// # Errors
    ///
    /// Returns [`Error::StringTooLong`] if the model name, version prerelease or
    /// target does not fit the `u16` length prefix, or [`Error::SectionTooLong`]
    /// if a section body exceeds the `u32` limit.
    pub fn build_unsigned(&self) -> Result<Vec<u8>, Error> {
        let chunk_count = self.chunk_count();
        let manifest = Manifest {
            model_name: self.model_name.clone(),
            model_version: self.model_version.clone(),
            target: self.target.clone(),
            quantization: self.quantization,
            payload_len: u64::try_from(self.payload.len()).unwrap_or(u64::MAX),
            payload_sha256: sha256(&self.payload),
            chunk_size: self.chunk_size,
            chunk_count,
        };

        let mut out = Header::new(chunk_count.saturating_add(1)).encode();
        write_section(&mut out, TAG_MANIFEST, &manifest.encode()?)?;

        let size = usize::try_from(self.chunk_size)
            .unwrap_or(usize::MAX)
            .max(1);
        for (index, data) in self.payload.chunks(size).enumerate() {
            let index = u32::try_from(index).unwrap_or(u32::MAX);
            let chunk = PayloadChunk::new(index, data);
            write_section(&mut out, TAG_PAYLOAD_CHUNK, &chunk.encode())?;
        }

        Ok(out)
    }
}

/// Append a signature section to an unsigned package.
///
/// The header is left untouched: a signature covers the header and every
/// section before it, so appending the signature keeps the digest valid.
///
/// # Errors
///
/// Returns [`Error::AlreadySigned`] if `unsigned` already carries a signature,
/// a parse error if `unsigned` is not a valid package, or
/// [`Error::SignatureTooLong`] if the signature blob is too large.
pub fn append_signature(
    unsigned: &[u8],
    algorithm: SignatureAlgorithm,
    signature: &[u8],
) -> Result<Vec<u8>, Error> {
    let package = Package::parse(unsigned)?;
    if package.is_signed() {
        return Err(Error::AlreadySigned);
    }
    let body = Signature {
        algorithm,
        bytes: signature,
    }
    .encode()?;
    let mut out = Vec::with_capacity(unsigned.len() + 8 + body.len());
    out.extend_from_slice(unsigned);
    write_section(&mut out, TAG_SIGNATURE, &body)?;
    Ok(out)
}
