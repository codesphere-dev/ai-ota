//! `Ed25519` signing and verification.

use ed25519_dalek::{
    Signature as DalekSignature, Signer, SigningKey as DalekSigningKey, Verifier,
    VerifyingKey as DalekVerifyingKey,
};

use crate::error::Error;

/// Length in bytes of an `Ed25519` seed (private key material).
pub const SEED_LEN: usize = 32;
/// Length in bytes of an `Ed25519` public key.
pub const PUBLIC_KEY_LEN: usize = 32;
/// Length in bytes of an `Ed25519` signature.
pub const SIGNATURE_LEN: usize = 64;

/// An `Ed25519` signing (private) key.
///
/// The key material is zeroized on drop by `ed25519-dalek`, and [`Debug`]
/// redacts it so a key can never be leaked through a log line.
pub struct SigningKey(DalekSigningKey);

impl SigningKey {
    /// Build a signing key from a 32-byte seed.
    #[must_use]
    pub fn from_seed(seed: &[u8; SEED_LEN]) -> Self {
        Self(DalekSigningKey::from_bytes(seed))
    }

    /// Build a signing key from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidKeyLength`] unless `bytes` is exactly
    /// [`SEED_LEN`] bytes long.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let seed = to_array::<SEED_LEN>(bytes).ok_or(Error::InvalidKeyLength {
            expected: SEED_LEN,
            found: bytes.len(),
        })?;
        Ok(Self::from_seed(&seed))
    }

    /// The 32-byte seed that reproduces this key.
    #[must_use]
    pub fn to_bytes(&self) -> [u8; SEED_LEN] {
        self.0.to_bytes()
    }

    /// The matching public key.
    #[must_use]
    pub fn verifying_key(&self) -> VerifyingKey {
        VerifyingKey(self.0.verifying_key())
    }

    /// Sign `message`.
    #[must_use]
    pub fn sign(&self, message: &[u8]) -> Signature {
        Signature(self.0.sign(message))
    }
}

impl core::fmt::Debug for SigningKey {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str("SigningKey(<redacted>)")
    }
}

/// An `Ed25519` public key.
#[derive(Debug, Clone, Copy)]
pub struct VerifyingKey(DalekVerifyingKey);

impl VerifyingKey {
    /// Build a public key from 32 bytes.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidKeyLength`] for a wrong length, or
    /// [`Error::Verification`] when the bytes are not a valid curve point.
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, Error> {
        let array = to_array::<PUBLIC_KEY_LEN>(bytes).ok_or(Error::InvalidKeyLength {
            expected: PUBLIC_KEY_LEN,
            found: bytes.len(),
        })?;
        DalekVerifyingKey::from_bytes(&array)
            .map(Self)
            .map_err(|_| Error::Verification)
    }

    /// The 32-byte compressed public key.
    #[must_use]
    pub fn to_bytes(self) -> [u8; PUBLIC_KEY_LEN] {
        self.0.to_bytes()
    }

    /// Verify `signature` over `message`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Verification`] when the signature does not verify.
    pub fn verify(&self, message: &[u8], signature: &Signature) -> Result<(), Error> {
        self.0
            .verify(message, &signature.0)
            .map_err(|_| Error::Verification)
    }
}

impl PartialEq for VerifyingKey {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bytes() == other.0.to_bytes()
    }
}

impl Eq for VerifyingKey {}

/// An `Ed25519` signature.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature(DalekSignature);

impl Signature {
    /// Build a signature from its 64-byte encoding.
    #[must_use]
    pub fn from_bytes(bytes: &[u8; SIGNATURE_LEN]) -> Self {
        Self(DalekSignature::from_bytes(bytes))
    }

    /// Build a signature from a byte slice.
    ///
    /// # Errors
    ///
    /// Returns [`Error::InvalidSignatureLength`] unless `bytes` is exactly
    /// [`SIGNATURE_LEN`] bytes long.
    pub fn from_slice(bytes: &[u8]) -> Result<Self, Error> {
        let array = to_array::<SIGNATURE_LEN>(bytes).ok_or(Error::InvalidSignatureLength {
            expected: SIGNATURE_LEN,
            found: bytes.len(),
        })?;
        Ok(Self::from_bytes(&array))
    }

    /// The 64-byte encoding of the signature.
    #[must_use]
    pub fn to_bytes(self) -> [u8; SIGNATURE_LEN] {
        self.0.to_bytes()
    }
}

/// Copy a slice into a fixed-size array, or `None` on a length mismatch.
fn to_array<const N: usize>(bytes: &[u8]) -> Option<[u8; N]> {
    <[u8; N]>::try_from(bytes).ok()
}
