# `.otapkg` v1 format

Reference specification for the OTA package container produced by
`ota package build` and consumed by `ota package inspect`, `ota sign` and the
device agent.

## Overview

An `.otapkg` file is a 64-byte fixed header followed by a sequence of
tag-length-value (TLV) sections. Every integer is little-endian. The layout is
designed so a device can validate the package while streaming it.

```
┌────────────┬───────────────────────────────────────────────┐
│  header    │  section, section, … , section                │
│ (64 bytes) │  (manifest, payload chunks, optional sig)     │
└────────────┴───────────────────────────────────────────────┘
```

## Header (64 bytes)

| Offset | Size | Field | Type | Notes |
| --- | --- | --- | --- | --- |
| 0 | 8 | magic | `[u8; 8]` | ASCII `OTAPKG01` |
| 8 | 2 | format_version | `u16` | `1` for this revision |
| 10 | 2 | header_len | `u16` | `64` |
| 12 | 4 | flags | `u32` | reserved, must be `0` |
| 16 | 4 | section_count | `u32` | number of sections **excluding** the signature section |
| 20 | 4 | header_crc32 | `u32` | CRC32 over bytes 0..20 |
| 24 | 40 | reserved | `[u8; 40]` | must be zero |

## Section framing (8-byte header + body)

| Field | Size | Type | Notes |
| --- | --- | --- | --- |
| tag | 1 | `u8` | `0x01` manifest, `0x02` payload chunk, `0x03` signature |
| reserved | 3 | `[u8; 3]` | must be zero |
| length | 4 | `u32` LE | byte length of the body |

The body immediately follows. Sections appear in order: one manifest, then zero
or more payload chunks, then optionally one signature section.

## Manifest section (`0x01`)

| Field | Type | Notes |
| --- | --- | --- |
| model_name | `u16`-length-prefixed UTF-8 string | for example `vision-v3` |
| major | `u32` | model version major |
| minor | `u32` | model version minor |
| patch | `u32` | model version patch |
| prerelease | `u16`-length-prefixed UTF-8 string | empty when absent |
| target | `u16`-length-prefixed UTF-8 string | for example `stm32h7` |
| quantization | `u16` | see quantization codes below |
| payload_len | `u64` | length of the reassembled payload |
| payload_sha256 | `[u8; 32]` | SHA-256 of the reassembled payload |
| chunk_size | `u32` | nominal chunk size used when splitting |
| chunk_count | `u32` | number of payload chunks that follow |

### Quantization codes

| Code | Name |
| --- | --- |
| 0 | `none` |
| 1 | `int8` |
| 2 | `int4` |
| 3 | `fp16` |
| 4 | `fp32` |
| 5 | `mixed` |

## Payload chunk section (`0x02`)

| Field | Type | Notes |
| --- | --- | --- |
| index | `u32` | zero-based, must be contiguous and in order |
| crc32 | `u32` | CRC32 of `data` |
| data | bytes | chunk bytes |

Chunks are concatenated in index order to reassemble the payload. The total
length must equal `payload_len` and `chunk_count` must equal the number of
chunks.

## Signature section (`0x03`, optional, always last)

| Field | Type | Notes |
| --- | --- | --- |
| algorithm | `u8` | `1` Ed25519, `2` ECDSA-P256, `3` RSA-3072 |
| reserved | `u8` | must be zero |
| length | `u16` | byte length of `signature` |
| signature | bytes | algorithm-specific signature bytes |

Only `Ed25519` (algorithm `1`) is implemented today; the other codes are
reserved.

## Signing

The signed region is the header followed by every section *before* the signature
section. The header's `section_count` excludes the signature section, so
appending a signature never changes the signed region and never requires
recomputing the header.

For `Ed25519`, the signature is computed over `SHA-256(signed_region)` by the
caller; the package format itself only stores the opaque signature bytes. The
`ota` CLI wires `Package::signing_digest()` to the key.

## Validation rules

A conforming parser must reject a package when any of the following holds:

- The magic is not `OTAPKG01` or the format version is unknown.
- The header CRC32 does not match.
- A section tag is unknown, or the signature section is not last.
- The manifest is missing, duplicated, or not the first section.
- A payload chunk index is out of order or the chunk count does not match the
  manifest.
- The reassembled payload length does not match `payload_len`.
- A chunk CRC32 does not match.
- The reassembled payload SHA-256 does not match `payload_sha256`.
- Trailing bytes remain after the last section.

## CLI

- `ota package build` — assemble an unsigned package from a payload file.
- `ota package inspect` — parse, verify the payload and print the manifest.
- `ota sign keygen` — generate an Ed25519 key pair.
- `ota sign package` — sign an unsigned package with a private key.
- `ota sign verify` — verify a signed package against a public key.

## See also

- ADR 0001: `.otapkg` v1 container format
  (`docs/engineering/architecture/adr/0001-package-format.md`).
- `crates/ota-package` — reference implementation.
- `docs/reference/cli/package.md`, `docs/reference/cli/sign.md`.
EOF
echo written