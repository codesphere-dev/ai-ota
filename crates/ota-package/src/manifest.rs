//! The manifest section: everything a device needs to accept a payload.

use alloc::string::String;
use alloc::vec::Vec;

use ota_core::Version;

use crate::codec::{write_string_u16, write_u16, write_u32, write_u64, Reader};
use crate::error::Error;

/// How the model weights in the payload are quantized.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
#[repr(u16)]
pub enum Quantization {
    /// Not a quantized model (for example raw firmware).
    None = 0,
    /// 8-bit integer weights.
    Int8 = 1,
    /// 4-bit integer weights.
    Int4 = 2,
    /// 16-bit float weights.
    Fp16 = 3,
    /// 32-bit float weights.
    Fp32 = 4,
    /// A mix of precisions within one model.
    Mixed = 5,
}

impl Quantization {
    /// Numeric code stored on the wire.
    #[must_use]
    pub const fn code(self) -> u16 {
        self as u16
    }

    /// Parse a wire code.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidQuantization`] for unknown codes.
    pub const fn from_code(code: u16) -> Result<Self, Error> {
        match code {
            0 => Ok(Self::None),
            1 => Ok(Self::Int8),
            2 => Ok(Self::Int4),
            3 => Ok(Self::Fp16),
            4 => Ok(Self::Fp32),
            5 => Ok(Self::Mixed),
            other => Err(Error::InvalidQuantization(other)),
        }
    }

    /// Short human-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::Int8 => "int8",
            Self::Int4 => "int4",
            Self::Fp16 => "fp16",
            Self::Fp32 => "fp32",
            Self::Mixed => "mixed",
        }
    }
}

/// The manifest section: identifies the model and describes the payload.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// Model or artifact name (for example `vision-v3`).
    pub model_name: String,
    /// Semantic version of the model.
    pub model_version: Version,
    /// Target identifier the payload is built for (for example `stm32h7`).
    pub target: String,
    /// Weight quantization of the payload.
    pub quantization: Quantization,
    /// Length in bytes of the reassembled payload.
    pub payload_len: u64,
    /// `SHA-256` digest of the reassembled payload.
    pub payload_sha256: [u8; 32],
    /// Nominal chunk size used when the payload was split.
    pub chunk_size: u32,
    /// Number of payload chunks that follow the manifest.
    pub chunk_count: u32,
}

impl Manifest {
    /// Encode the manifest body.
    ///
    /// # Errors
    ///
    /// Returns [`Error::StringTooLong`] if any string does not fit the `u16`
    /// length prefix.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let mut out = Vec::new();
        write_string_u16(&mut out, &self.model_name)?;
        write_u32(&mut out, self.model_version.major());
        write_u32(&mut out, self.model_version.minor());
        write_u32(&mut out, self.model_version.patch());
        write_string_u16(&mut out, self.model_version.pre().unwrap_or(""))?;
        write_string_u16(&mut out, &self.target)?;
        write_u16(&mut out, self.quantization.code());
        write_u64(&mut out, self.payload_len);
        out.extend_from_slice(&self.payload_sha256);
        write_u32(&mut out, self.chunk_size);
        write_u32(&mut out, self.chunk_count);
        Ok(out)
    }

    /// Parse a manifest body.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Truncated`] for a short body, [`Error::InvalidUtf8`] for
    /// a malformed string, or [`Error::InvalidQuantization`] for an unknown
    /// quantization code.
    pub fn decode(body: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(body);
        let model_name = reader.string_u16()?;
        let major = reader.u32()?;
        let minor = reader.u32()?;
        let patch = reader.u32()?;
        let pre = reader.string_u16()?;
        let model_version = if pre.is_empty() {
            Version::new(major, minor, patch)
        } else {
            Version::with_pre(major, minor, patch, pre)
        };
        let target = reader.string_u16()?;
        let quantization = Quantization::from_code(reader.u16()?)?;
        let payload_len = reader.u64()?;
        let payload_sha256 = reader.array32()?;
        let chunk_size = reader.u32()?;
        let chunk_count = reader.u32()?;
        Ok(Self {
            model_name,
            model_version,
            target,
            quantization,
            payload_len,
            payload_sha256,
            chunk_size,
            chunk_count,
        })
    }
}
