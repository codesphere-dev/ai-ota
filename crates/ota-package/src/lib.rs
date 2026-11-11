//! The versioned `.otapkg` OTA package container format.
//!
//! An `.otapkg` file is a small fixed-size header followed by a sequence of
//! tag-length-value sections:
//!
//! ```text
//! ┌────────────┬───────────────────────────────────────────────┐
//! │  header    │  section, section, … , section                │
//! │  (64 bytes)│  (manifest, payload chunks, optional sig)     │
//! └────────────┴───────────────────────────────────────────────┘
//! ```
//!
//! The layout is intentionally boring: every integer is little-endian, every
//! section is self-describing, and the whole thing is designed so a device can
//! validate it while streaming it off the wire.
//!
//! # Format at a glance
//!
//! * **Header** (64 bytes): magic `OTAPKG01`, format version, flags, section
//!   count and a `CRC32` over the fixed fields.
//! * **Manifest section** (`0x01`): model name, model version, target,
//!   quantization, payload length and hash, and chunking parameters.
//! * **Payload chunk sections** (`0x02`): an index, a `CRC32` and a slice of
//!   the payload.
//! * **Signature section** (`0x03`, optional, always last): the signing
//!   algorithm and an opaque signature blob.
//!
//! The signature covers the header and every section *before* the signature
//! section, and the header's section count excludes the signature section, so
//! appending a signature never invalidates the digest. See
//! [`Package::signing_digest`].
//!
//! # `no_std`
//!
//! The crate is `no_std` (with `alloc`). Enable the `std` feature to get
//! [`std::error::Error`] implementations from `thiserror`.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod builder;
pub mod chunk;
mod codec;
pub mod error;
pub mod header;
pub mod manifest;
pub mod reader;
pub mod signature;
pub mod tlv;

pub use builder::{append_signature, PackageBuilder, DEFAULT_CHUNK_SIZE};
pub use chunk::PayloadChunk;
pub use error::Error;
pub use header::{Header, FLAG_NONE, FORMAT_VERSION, HEADER_LEN, MAGIC};
pub use manifest::{Manifest, Quantization};
pub use reader::Package;
pub use signature::{Signature, SignatureAlgorithm};

/// Convenient result alias used across this crate.
pub type Result<T, E = Error> = core::result::Result<T, E>;
