//! Core types, versioning and primitives shared by every `ai-ota` crate.
//!
//! The crate is `no_std` (with `alloc`) so it can be linked into the MCU-side
//! crates such as `ota-package` and `ota-boot`. Enable the `std` feature to get
//! [`std::error::Error`] implementations.

#![cfg_attr(not(feature = "std"), no_std)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod error;
pub mod version;

pub use error::Error;
pub use version::Version;

/// Convenient result alias used across the workspace.
pub type Result<T, E = Error> = core::result::Result<T, E>;
