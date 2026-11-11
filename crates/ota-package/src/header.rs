//! The fixed-size package header.

use alloc::vec::Vec;

use crate::codec::{crc32, write_u16, write_u32, Reader};
use crate::error::Error;

/// Magic bytes at the start of every package: `OTAPKG01`.
pub const MAGIC: [u8; 8] = *b"OTAPKG01";

/// Format version written by this build.
pub const FORMAT_VERSION: u16 = 1;

/// Size in bytes of the fixed header written by this build.
pub const HEADER_LEN: u16 = 64;

/// Largest header length this reader accepts from the wire.
pub const MAX_HEADER_LEN: u16 = 4096;

/// No feature flags are defined yet; the field is reserved for future use.
pub const FLAG_NONE: u32 = 0;

/// The fixed header that prefixes every package.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    /// Format version of the container.
    pub format_version: u16,
    /// Total header length in bytes, including reserved bytes.
    pub len: u16,
    /// Feature flags; currently always [`FLAG_NONE`].
    pub flags: u32,
    /// Number of sections before the optional signature section.
    pub section_count: u32,
    /// `CRC32` over the first 20 header bytes.
    pub crc32: u32,
}

impl Header {
    /// Build a header advertising `section_count` content sections.
    #[must_use]
    pub fn new(section_count: u32) -> Self {
        let mut header = Self {
            format_version: FORMAT_VERSION,
            len: HEADER_LEN,
            flags: FLAG_NONE,
            section_count,
            crc32: 0,
        };
        header.crc32 = header.compute_crc();
        header
    }

    /// Serialise the header to its 64-byte little-endian form.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(usize::from(HEADER_LEN));
        out.extend_from_slice(&MAGIC);
        write_u16(&mut out, self.format_version);
        write_u16(&mut out, self.len);
        write_u32(&mut out, self.flags);
        write_u32(&mut out, self.section_count);
        let crc = crc32(&out[..20]);
        write_u32(&mut out, crc);
        out.resize(usize::from(HEADER_LEN), 0);
        out
    }

    /// Parse a header from the start of `bytes`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::BadMagic`] for a wrong magic, [`Error::UnsupportedVersion`]
    /// for an unknown format version, [`Error::HeaderTooShort`] or
    /// [`Error::HeaderTooLong`] for an out-of-range header length, and
    /// [`Error::HeaderCrcMismatch`] when the checksum fails.
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        if reader.take(8)? != MAGIC {
            return Err(Error::BadMagic);
        }
        let format_version = reader.u16()?;
        if format_version != FORMAT_VERSION {
            return Err(Error::UnsupportedVersion(format_version));
        }
        let len = reader.u16()?;
        if len < HEADER_LEN {
            return Err(Error::HeaderTooShort {
                found: len,
                min: HEADER_LEN,
            });
        }
        if len > MAX_HEADER_LEN {
            return Err(Error::HeaderTooLong {
                found: len,
                max: MAX_HEADER_LEN,
            });
        }
        let flags = reader.u32()?;
        let section_count = reader.u32()?;
        let stored_crc = reader.u32()?;
        let computed = crc32(&bytes[..20]);
        if computed != stored_crc {
            return Err(Error::HeaderCrcMismatch {
                stored: stored_crc,
                computed,
            });
        }
        if bytes.len() < usize::from(len) {
            return Err(Error::Truncated);
        }
        Ok(Self {
            format_version,
            len,
            flags,
            section_count,
            crc32: stored_crc,
        })
    }

    /// `CRC32` over the first 20 header bytes.
    fn compute_crc(&self) -> u32 {
        let mut buf = [0u8; 20];
        buf[..8].copy_from_slice(&MAGIC);
        buf[8..10].copy_from_slice(&self.format_version.to_le_bytes());
        buf[10..12].copy_from_slice(&self.len.to_le_bytes());
        buf[12..16].copy_from_slice(&self.flags.to_le_bytes());
        buf[16..20].copy_from_slice(&self.section_count.to_le_bytes());
        crc32(&buf)
    }
}
