//! The optional signature section.

use alloc::vec::Vec;

use crate::codec::{write_u16, Reader};
use crate::error::Error;

/// Signing algorithms understood by this version of the format.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum SignatureAlgorithm {
    /// `Ed25519` (the default).
    Ed25519,
    /// `ECDSA` over `P-256`.
    EcdsaP256,
    /// `RSA-3072`.
    Rsa3072,
}

impl SignatureAlgorithm {
    /// Numeric code stored on the wire.
    #[must_use]
    pub const fn code(self) -> u8 {
        match self {
            Self::Ed25519 => 1,
            Self::EcdsaP256 => 2,
            Self::Rsa3072 => 3,
        }
    }

    /// Parse a wire code.
    ///
    /// # Errors
    ///
    /// Returns [`Error::UnsupportedAlgorithm`] for unknown codes.
    pub const fn from_code(code: u8) -> Result<Self, Error> {
        match code {
            1 => Ok(Self::Ed25519),
            2 => Ok(Self::EcdsaP256),
            3 => Ok(Self::Rsa3072),
            other => Err(Error::UnsupportedAlgorithm(other)),
        }
    }

    /// Short human-readable name.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ed25519 => "ed25519",
            Self::EcdsaP256 => "ecdsa-p256",
            Self::Rsa3072 => "rsa-3072",
        }
    }
}

/// A parsed signature section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature<'a> {
    /// Algorithm that produced [`Signature::bytes`].
    pub algorithm: SignatureAlgorithm,
    /// Opaque signature bytes.
    pub bytes: &'a [u8],
}

impl<'a> Signature<'a> {
    /// Encode the signature section body: algorithm, reserved byte and a
    /// `u16`-length-prefixed blob.
    ///
    /// # Errors
    ///
    /// Returns [`Error::SignatureTooLong`] if the blob does not fit the `u16`
    /// length prefix.
    pub fn encode(&self) -> Result<Vec<u8>, Error> {
        let len = u16::try_from(self.bytes.len())
            .map_err(|_| Error::SignatureTooLong(self.bytes.len()))?;
        let mut out = Vec::with_capacity(4 + self.bytes.len());
        out.push(self.algorithm.code());
        out.push(0);
        write_u16(&mut out, len);
        out.extend_from_slice(self.bytes);
        Ok(out)
    }

    /// Parse a signature section body.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Truncated`] for a short body or
    /// [`Error::UnsupportedAlgorithm`] for an unknown algorithm code.
    pub fn decode(body: &'a [u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(body);
        let algorithm = SignatureAlgorithm::from_code(reader.u8()?)?;
        let _reserved = reader.u8()?;
        let len = usize::from(reader.u16()?);
        let bytes = reader.take(len)?;
        Ok(Self { algorithm, bytes })
    }
}
