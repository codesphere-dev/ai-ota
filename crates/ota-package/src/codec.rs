//! Little-endian reader/writer primitives shared by the package codec.

use alloc::string::String;
use alloc::vec::Vec;

use sha2::{Digest, Sha256};

use crate::error::Error;

/// `CRC32` (IEEE) of `bytes`.
#[must_use]
pub(crate) fn crc32(bytes: &[u8]) -> u32 {
    let mut hasher = crc32fast::Hasher::new();
    hasher.update(bytes);
    hasher.finalize()
}

/// `SHA-256` digest of `bytes`.
#[must_use]
pub(crate) fn sha256(bytes: &[u8]) -> [u8; 32] {
    let digest = Sha256::digest(bytes);
    let mut out = [0u8; 32];
    out.copy_from_slice(&digest);
    out
}

/// A bounds-checked cursor over a byte slice.
pub(crate) struct Reader<'a> {
    buf: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    /// Wrap `buf`.
    pub(crate) const fn new(buf: &'a [u8]) -> Self {
        Self { buf, pos: 0 }
    }

    /// Number of bytes not yet consumed.
    pub(crate) const fn remaining(&self) -> usize {
        self.buf.len() - self.pos
    }

    /// Consume and return the next `len` bytes.
    pub(crate) fn take(&mut self, len: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(len).ok_or(Error::Truncated)?;
        if end > self.buf.len() {
            return Err(Error::Truncated);
        }
        let out = &self.buf[self.pos..end];
        self.pos = end;
        Ok(out)
    }

    /// Read a `u8`.
    pub(crate) fn u8(&mut self) -> Result<u8, Error> {
        Ok(self.take(1)?[0])
    }

    /// Read a little-endian `u16`.
    pub(crate) fn u16(&mut self) -> Result<u16, Error> {
        let b = self.take(2)?;
        Ok(u16::from_le_bytes([b[0], b[1]]))
    }

    /// Read a little-endian `u32`.
    pub(crate) fn u32(&mut self) -> Result<u32, Error> {
        let b = self.take(4)?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }

    /// Read a little-endian `u64`.
    pub(crate) fn u64(&mut self) -> Result<u64, Error> {
        let b = self.take(8)?;
        Ok(u64::from_le_bytes([
            b[0], b[1], b[2], b[3], b[4], b[5], b[6], b[7],
        ]))
    }

    /// Read a 32-byte array.
    pub(crate) fn array32(&mut self) -> Result<[u8; 32], Error> {
        let mut out = [0u8; 32];
        out.copy_from_slice(self.take(32)?);
        Ok(out)
    }

    /// Read a `u16`-length-prefixed `UTF-8` string.
    pub(crate) fn string_u16(&mut self) -> Result<String, Error> {
        let len = usize::from(self.u16()?);
        let bytes = self.take(len)?;
        let text = core::str::from_utf8(bytes).map_err(|_| Error::InvalidUtf8)?;
        Ok(String::from(text))
    }
}

/// Append a little-endian `u16`.
pub(crate) fn write_u16(out: &mut Vec<u8>, value: u16) {
    out.extend_from_slice(&value.to_le_bytes());
}

/// Append a little-endian `u32`.
pub(crate) fn write_u32(out: &mut Vec<u8>, value: u32) {
    out.extend_from_slice(&value.to_le_bytes());
}

/// Append a little-endian `u64`.
pub(crate) fn write_u64(out: &mut Vec<u8>, value: u64) {
    out.extend_from_slice(&value.to_le_bytes());
}

/// Append a `u16`-length-prefixed `UTF-8` string.
pub(crate) fn write_string_u16(out: &mut Vec<u8>, value: &str) -> Result<(), Error> {
    let bytes = value.as_bytes();
    let len = u16::try_from(bytes.len()).map_err(|_| Error::StringTooLong {
        len: bytes.len(),
        max: usize::from(u16::MAX),
    })?;
    write_u16(out, len);
    out.extend_from_slice(bytes);
    Ok(())
}
