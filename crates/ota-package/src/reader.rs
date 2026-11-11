//! Zero-copy parsing of `.otapkg` files.

use alloc::vec::Vec;

use sha2::{Digest, Sha256};

use crate::chunk::PayloadChunk;
use crate::error::Error;
use crate::header::Header;
use crate::manifest::Manifest;
use crate::signature::Signature;
use crate::tlv::{
    SectionHeader, SECTION_HEADER_LEN, TAG_MANIFEST, TAG_PAYLOAD_CHUNK, TAG_SIGNATURE,
};

/// A section located while scanning the package.
struct RawSection<'a> {
    tag: u8,
    body: &'a [u8],
    end: usize,
}

/// A parsed, borrowed view over an `.otapkg` byte buffer.
///
/// Parsing validates the header, the section framing, the manifest and the
/// chunk layout. It does not verify payload checksums; call
/// [`Package::verify_payload`] for that, and [`Package::signing_digest`] to
/// obtain the digest a signature must cover.
#[derive(Debug, Clone)]
pub struct Package<'a> {
    bytes: &'a [u8],
    header: Header,
    manifest: Manifest,
    chunks: Vec<PayloadChunk<'a>>,
    signature: Option<Signature<'a>>,
    content_end: usize,
}

impl<'a> Package<'a> {
    /// Parse and validate the framing of `bytes`.
    ///
    /// # Errors
    ///
    /// Returns an error if the header, the section framing, the manifest or the
    /// chunk layout is malformed.
    pub fn parse(bytes: &'a [u8]) -> Result<Self, Error> {
        let header = Header::decode(bytes)?;
        let mut offset = usize::from(header.len);
        let mut manifest = None;
        let mut chunks = Vec::new();

        for index in 0..header.section_count {
            let section = read_section(bytes, offset)?;
            offset = section.end;
            match section.tag {
                TAG_MANIFEST => {
                    if manifest.is_some() {
                        return Err(Error::DuplicateManifest);
                    }
                    if index != 0 {
                        return Err(Error::ManifestNotFirst);
                    }
                    manifest = Some(Manifest::decode(section.body)?);
                }
                TAG_PAYLOAD_CHUNK => {
                    if manifest.is_none() {
                        return Err(Error::ManifestNotFirst);
                    }
                    chunks.push(PayloadChunk::decode(section.body)?);
                }
                TAG_SIGNATURE => return Err(Error::SignatureNotLast),
                other => return Err(Error::UnknownSectionTag(other)),
            }
        }

        let content_end = offset;
        let signature = match read_optional_section(bytes, offset, TAG_SIGNATURE)? {
            Some(section) => {
                offset = section.end;
                Some(Signature::decode(section.body)?)
            }
            None => None,
        };

        if offset != bytes.len() {
            return Err(Error::TrailingData);
        }

        let package = Self {
            bytes,
            header,
            manifest: manifest.ok_or(Error::MissingManifest)?,
            chunks,
            signature,
            content_end,
        };
        package.validate_layout()?;
        Ok(package)
    }

    /// The package header.
    #[must_use]
    pub const fn header(&self) -> &Header {
        &self.header
    }

    /// The parsed manifest.
    #[must_use]
    pub const fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    /// The payload chunks, in order.
    #[must_use]
    pub fn chunks(&self) -> &[PayloadChunk<'a>] {
        &self.chunks
    }

    /// The signature, when the package is signed.
    #[must_use]
    pub const fn signature(&self) -> Option<&Signature<'a>> {
        self.signature.as_ref()
    }

    /// Whether the package carries a signature.
    #[must_use]
    pub const fn is_signed(&self) -> bool {
        self.signature.is_some()
    }

    /// The exact byte range a signature must cover.
    ///
    /// This is the header followed by every section that precedes the signature
    /// section. Appending a signature never changes it.
    #[must_use]
    pub fn signed_region(&self) -> &'a [u8] {
        &self.bytes[..self.content_end]
    }

    /// `SHA-256` digest of [`Package::signed_region`].
    #[must_use]
    pub fn signing_digest(&self) -> [u8; 32] {
        let digest = Sha256::digest(self.signed_region());
        let mut out = [0u8; 32];
        out.copy_from_slice(&digest);
        out
    }

    /// Reassemble the payload into a contiguous buffer.
    #[must_use]
    pub fn payload(&self) -> Vec<u8> {
        let capacity = usize::try_from(self.manifest.payload_len).unwrap_or(0);
        let mut out = Vec::with_capacity(capacity);
        for chunk in &self.chunks {
            out.extend_from_slice(chunk.data);
        }
        out
    }

    /// Verify every chunk `CRC32` and the overall payload `SHA-256`.
    ///
    /// # Errors
    ///
    /// Returns [`Error::ChunkCrcMismatch`] for a corrupt chunk or
    /// [`Error::PayloadHashMismatch`] when the payload does not match the
    /// manifest.
    pub fn verify_payload(&self) -> Result<(), Error> {
        let mut hasher = Sha256::new();
        for chunk in &self.chunks {
            chunk.verify()?;
            hasher.update(chunk.data);
        }
        let digest = hasher.finalize();
        let mut computed = [0u8; 32];
        computed.copy_from_slice(&digest);
        if computed == self.manifest.payload_sha256 {
            Ok(())
        } else {
            Err(Error::PayloadHashMismatch)
        }
    }

    /// Check that the chunks match the manifest's count, order and total length.
    fn validate_layout(&self) -> Result<(), Error> {
        let found = u32::try_from(self.chunks.len()).unwrap_or(u32::MAX);
        if found != self.manifest.chunk_count {
            return Err(Error::ChunkCountMismatch {
                expected: self.manifest.chunk_count,
                found,
            });
        }
        let mut total: u64 = 0;
        for (position, chunk) in self.chunks.iter().enumerate() {
            let expected = u32::try_from(position).unwrap_or(u32::MAX);
            if chunk.index != expected {
                return Err(Error::ChunkIndexOutOfOrder {
                    expected,
                    found: chunk.index,
                });
            }
            total = total.saturating_add(u64::try_from(chunk.data.len()).unwrap_or(u64::MAX));
        }
        if total != self.manifest.payload_len {
            return Err(Error::PayloadLengthMismatch {
                expected: self.manifest.payload_len,
                found: total,
            });
        }
        Ok(())
    }
}

/// Read the section starting at `offset`.
fn read_section(bytes: &[u8], offset: usize) -> Result<RawSection<'_>, Error> {
    let header_end = offset
        .checked_add(SECTION_HEADER_LEN)
        .ok_or(Error::Truncated)?;
    let prefix = bytes.get(offset..header_end).ok_or(Error::Truncated)?;
    let header = SectionHeader::decode(prefix)?;
    let body_end = header_end
        .checked_add(usize::try_from(header.length).unwrap_or(usize::MAX))
        .ok_or(Error::Truncated)?;
    let body = bytes.get(header_end..body_end).ok_or(Error::Truncated)?;
    Ok(RawSection {
        tag: header.tag,
        body,
        end: body_end,
    })
}

/// Read a section at `offset` only when its tag matches, otherwise return `None`.
fn read_optional_section(
    bytes: &[u8],
    offset: usize,
    tag: u8,
) -> Result<Option<RawSection<'_>>, Error> {
    if bytes.len().saturating_sub(offset) < SECTION_HEADER_LEN {
        return Ok(None);
    }
    let prefix = &bytes[offset..offset + SECTION_HEADER_LEN];
    let header = SectionHeader::decode(prefix)?;
    if header.tag == tag {
        read_section(bytes, offset).map(Some)
    } else {
        Ok(None)
    }
}
