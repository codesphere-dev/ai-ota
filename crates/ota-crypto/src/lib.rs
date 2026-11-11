//! Signing, hashing and verification primitives for secure updates.
//!
//! The crate is `no_std` (with `alloc`) so the verification path can be linked
//! into the MCU-side crates. It stays deliberately independent of
//! `ota-package`: it signs and verifies opaque byte strings, and the caller
//! decides what those bytes are (for `.otapkg`, the package's signing digest).
//!
//! # Key management
//!
//! [`SigningKey`] can be built from a 32-byte seed with
//! [`SigningKey::from_seed`]. Generating that seed from OS entropy is left to
//! the caller (the `ota` CLI does it), so this crate never needs an RNG and
//! stays portable to bare-metal targets.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod ed25519;
pub mod error;
pub mod keyring;

pub use ed25519::{Signature, SigningKey, VerifyingKey, PUBLIC_KEY_LEN, SEED_LEN, SIGNATURE_LEN};
pub use error::Error;
pub use keyring::{
    decode_private_key, decode_public_key, encode_private_key, encode_public_key, hex_decode,
    hex_encode,
};

/// Convenient result alias used across this crate.
pub type Result<T, E = Error> = core::result::Result<T, E>;
