//! Text encoding for `Ed25519` keys.
//!
//! Keys are stored as a small envelope around a lowercase hex payload:
//!
//! ```text
//! -----BEGIN AI-OTA ED25519 PRIVATE KEY-----
//! 000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f
//! -----END AI-OTA ED25519 PRIVATE KEY-----
//! ```
//!
//! The envelope is deliberately trivial to parse on a constrained device and
//! keeps key files diff-friendly. `PKCS#8` / `PEM` interoperability is left for
//! a later revision.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::ed25519::{SigningKey, VerifyingKey};
use crate::error::Error;

/// Envelope header for a private key file.
pub const PRIVATE_KEY_HEADER: &str = "-----BEGIN AI-OTA ED25519 PRIVATE KEY-----";
/// Envelope footer for a private key file.
pub const PRIVATE_KEY_FOOTER: &str = "-----END AI-OTA ED25519 PRIVATE KEY-----";
/// Envelope header for a public key file.
pub const PUBLIC_KEY_HEADER: &str = "-----BEGIN AI-OTA ED25519 PUBLIC KEY-----";
/// Envelope footer for a public key file.
pub const PUBLIC_KEY_FOOTER: &str = "-----END AI-OTA ED25519 PUBLIC KEY-----";

/// Encode `bytes` as lowercase hex.
#[must_use]
pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for &byte in bytes {
        out.push(char::from(HEX[usize::from(byte >> 4)]));
        out.push(char::from(HEX[usize::from(byte & 0x0f)]));
    }
    out
}

/// Decode lowercase or uppercase hex into bytes.
///
/// # Errors
///
/// Returns [`Error::InvalidHexLength`] for an odd character count, or
/// [`Error::InvalidHexByte`] for a non-hex character.
pub fn hex_decode(text: &str) -> Result<Vec<u8>, Error> {
    let bytes = text.as_bytes();
    if !bytes.len().is_multiple_of(2) {
        return Err(Error::InvalidHexLength(bytes.len()));
    }
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.as_chunks::<2>().0 {
        out.push((nibble(pair[0])? << 4) | nibble(pair[1])?);
    }
    Ok(out)
}

/// Encode a private key as an enveloped hex file.
#[must_use]
pub fn encode_private_key(key: &SigningKey) -> String {
    format!(
        "{PRIVATE_KEY_HEADER}\n{}\n{PRIVATE_KEY_FOOTER}\n",
        hex_encode(&key.to_bytes())
    )
}

/// Decode a private key from an enveloped hex file.
///
/// # Errors
///
/// Returns [`Error::MalformedKeyFile`] for a bad envelope, [`Error::EmptyKeyFile`]
/// when the payload is empty, or a hex/decoding error.
pub fn decode_private_key(text: &str) -> Result<SigningKey, Error> {
    let payload = envelope_payload(text, PRIVATE_KEY_HEADER, PRIVATE_KEY_FOOTER)?;
    SigningKey::from_bytes(&hex_decode(payload)?)
}

/// Encode a public key as an enveloped hex file.
#[must_use]
pub fn encode_public_key(key: &VerifyingKey) -> String {
    format!(
        "{PUBLIC_KEY_HEADER}\n{}\n{PUBLIC_KEY_FOOTER}\n",
        hex_encode(&key.to_bytes())
    )
}

/// Decode a public key from an enveloped hex file.
///
/// # Errors
///
/// Returns [`Error::MalformedKeyFile`] for a bad envelope, [`Error::EmptyKeyFile`]
/// when the payload is empty, or a hex/decoding error.
pub fn decode_public_key(text: &str) -> Result<VerifyingKey, Error> {
    let payload = envelope_payload(text, PUBLIC_KEY_HEADER, PUBLIC_KEY_FOOTER)?;
    VerifyingKey::from_bytes(&hex_decode(payload)?)
}

fn nibble(byte: u8) -> Result<u8, Error> {
    match byte {
        b'0'..=b'9' => Ok(byte - b'0'),
        b'a'..=b'f' => Ok(byte - b'a' + 10),
        b'A'..=b'F' => Ok(byte - b'A' + 10),
        other => Err(Error::InvalidHexByte(other)),
    }
}

fn envelope_payload<'a>(text: &'a str, header: &str, footer: &str) -> Result<&'a str, Error> {
    let mut lines = text.lines().map(str::trim).filter(|line| !line.is_empty());
    if lines.next() != Some(header) {
        return Err(Error::MalformedKeyFile);
    }
    let payload = lines.next().ok_or(Error::MalformedKeyFile)?;
    if lines.next() != Some(footer) {
        return Err(Error::MalformedKeyFile);
    }
    if lines.next().is_some() {
        return Err(Error::MalformedKeyFile);
    }
    if payload.is_empty() {
        return Err(Error::EmptyKeyFile);
    }
    Ok(payload)
}
