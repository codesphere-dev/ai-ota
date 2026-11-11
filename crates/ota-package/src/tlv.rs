//! Section framing: an 8-byte header followed by a body.

use alloc::vec::Vec;

use crate::codec::Reader;
use crate::error::Error;

/// Section tag for the manifest.
pub const TAG_MANIFEST: u8 = 0x01;
/// Section tag for a payload chunk.
pub const TAG_PAYLOAD_CHUNK: u8 = 0x02;
/// Section tag for the optional signature.
pub const TAG_SIGNATURE: u8 = 0x03;

/// Size in bytes of a section header (`tag` + 3 reserved + `u32` length).
pub const SECTION_HEADER_LEN: usize = 8;

/// The 8-byte prefix of every section.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SectionHeader {
    /// Section tag.
    pub tag: u8,
    /// Length in bytes of the section body that follows.
    pub length: u32,
}

impl SectionHeader {
    /// Encode the section header.
    #[must_use]
    pub fn encode(self) -> [u8; SECTION_HEADER_LEN] {
        let mut out = [0u8; SECTION_HEADER_LEN];
        out[0] = self.tag;
        out[4..8].copy_from_slice(&self.length.to_le_bytes());
        out
    }

    /// Parse a section header.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Truncated`] when fewer than [`SECTION_HEADER_LEN`] bytes
    /// are available.
    pub fn decode(bytes: &[u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(bytes);
        let tag = reader.u8()?;
        let _reserved = reader.take(3)?;
        let length = reader.u32()?;
        Ok(Self { tag, length })
    }
}

/// Append a complete section (header followed by body) to `out`.
pub(crate) fn write_section(out: &mut Vec<u8>, tag: u8, body: &[u8]) -> Result<(), Error> {
    let length = u32::try_from(body.len()).map_err(|_| Error::SectionTooLong)?;
    out.extend_from_slice(&SectionHeader { tag, length }.encode());
    out.extend_from_slice(body);
    Ok(())
}
