//! Payload chunk sections.

use alloc::vec::Vec;

use crate::codec::{crc32, write_u32, Reader};
use crate::error::Error;

/// A single payload chunk: an index, a `CRC32` and a slice of the payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PayloadChunk<'a> {
    /// Zero-based index of the chunk within the payload.
    pub index: u32,
    /// `CRC32` of [`PayloadChunk::data`].
    pub crc32: u32,
    /// Raw payload bytes carried by this chunk.
    pub data: &'a [u8],
}

impl<'a> PayloadChunk<'a> {
    /// Build a chunk, computing its `CRC32`.
    #[must_use]
    pub fn new(index: u32, data: &'a [u8]) -> Self {
        Self {
            index,
            crc32: crc32(data),
            data,
        }
    }

    /// Encode the chunk body: `index`, `CRC32` and data.
    #[must_use]
    pub fn encode(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(8 + self.data.len());
        write_u32(&mut out, self.index);
        write_u32(&mut out, self.crc32);
        out.extend_from_slice(self.data);
        out
    }

    /// Parse a chunk body.
    ///
    /// # Errors
    ///
    /// Returns [`Error::Truncated`] when the body is shorter than the fixed
    /// `index` and `CRC32` fields.
    pub fn decode(body: &'a [u8]) -> Result<Self, Error> {
        let mut reader = Reader::new(body);
        let index = reader.u32()?;
        let crc = reader.u32()?;
        let data = reader.take(reader.remaining())?;
        Ok(Self {
            index,
            crc32: crc,
            data,
        })
    }

    /// Verify the chunk `CRC32`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ChunkCrcMismatch`] when the stored and computed
    /// checksums differ.
    pub fn verify(&self) -> Result<(), Error> {
        let computed = crc32(self.data);
        if computed == self.crc32 {
            Ok(())
        } else {
            Err(Error::ChunkCrcMismatch {
                index: self.index,
                stored: self.crc32,
                computed,
            })
        }
    }
}
