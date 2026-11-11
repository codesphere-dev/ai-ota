# ADR 0001: `.otapkg` v1 container format

- **Status:** Accepted
- **Date:** 2026-11-11
- **Deciders:** ai-ota maintainers
- **Supersedes:** none
- **Related:** `docs/reference/formats/package-format.md`

## Context

`ota-package` started as an empty crate: there was no wire format for the update
artifacts that the rest of the system (agent, bootloader, server, CLI) needs to
exchange. Every downstream crate (`ota-delta`, `ota-agent`, `ota-bootloader`)
assumes a single self-describing container it can stream, parse and verify on a
device, so the format had to be settled before any of them could be implemented.

Constraints that shaped the choice:

1. **`no_std` with `alloc`.** `ota-package`, `ota-agent` and `ota-bootloader`
   must build without `std` (DEV.md). The format therefore cannot depend on
   `serde` with derived `Deserialize`, on a runtime allocator beyond `alloc`,
   or on OS services.
2. **Streaming on a constrained device.** The bootloader may have tens of
   kilobytes of RAM and must validate the package while reading it off flash or
   the wire, so the header must be fixed-size and every section self-describing.
3. **Tamper evidence.** The manifest carries the payload length and SHA-256, and
   every chunk carries its own CRC32, so a corrupt or tampered package is
   detected before any byte is applied.
4. **Signable without rewriting.** Attaching a signature must not change the
   bytes the signature covers, otherwise the signature could never be verified
   after the fact.
5. **Append-only signing.** A package may be built unsigned and signed later by
   a different party (the release pipeline, an HSM), so signing must be a pure
   append operation.

## Decision

Define a custom binary format, `.otapkg` v1, consisting of a 64-byte fixed
header followed by a sequence of tag-length-value (TLV) sections:

- **Header** (64 bytes): magic `OTAPKG01`, format version `u16`, header length
  `u16`, flags `u32`, section count `u32`, header CRC32 `u32`, and reserved
  bytes. All integers are little-endian.
- **Sections**: tag `u8`, reserved `[u8; 3]`, length `u32` LE, body. Tags:
  `0x01` manifest, `0x02` payload chunk, `0x03` signature.
- **Manifest**: model name, model version, target, quantization, payload length,
  payload SHA-256, chunk size, chunk count. Hand-rolled binary encoding (no
  `serde`) so the crate stays `no_std`.
- **Payload chunks**: index `u32`, CRC32 `u32`, data. Chunks are split by the
  builder at a configurable chunk size (default 64 KiB).
- **Signature section** (optional, always last): algorithm code `u8`,
  reserved `u8`, signature length `u16`, signature bytes.

The signature covers the header and every section *before* the signature
section. The header's `section_count` excludes the signature section, so
appending a signature never changes the signed region and never requires
recomputing the header. `append_signature` is therefore a pure append.

Signature algorithms are identified by a numeric code (`1 = Ed25519`,
`2 = ECDSA-P256`, `3 = RSA-3072`), but only `Ed25519` is implemented in this
slice; the others are reserved for future work.

## Consequences

**Positive**

- `ota-package` builds and tests on `no_std` (verified in CI with
  `--no-default-features`), and the format is simple enough to re-implement on
  a device in a few hundred lines.
- Packages can be built unsigned and signed later without rewriting the body.
- Tamper detection is layered: chunk CRC32, whole-payload SHA-256, and the
  Ed25519 signature over the whole signed region.
- The format is self-describing: a parser does not need out-of-band knowledge
  of chunk counts or sizes.

**Negative**

- A custom format must be documented and kept in sync with the implementation;
  this ADR and `docs/reference/formats/package-format.md` are the source of
  truth.
- `ota-crypto` is kept deliberately independent of `ota-package`: it signs
  opaque byte strings, and the CLI is what wires `Package::signing_digest` to
  the key. Two crates must agree on what the signed bytes are.
- The signature covers the whole signed region, so verification requires reading
  the package twice (once to hash, once to verify) unless the device caches it.

**Alternatives considered**

- **ZIP / tarball.** Rejected: requires an allocator-heavy parser and gives no
  native tamper evidence.
- **CBOR / COSE.** Rejected: pulls in a generic CBOR implementation whose
  memory footprint and `no_std` story are heavier than the hand-rolled TLV
  framing, and the device still needs a custom verifier.
- **A signed manifest pointing at a payload file.** Rejected: splits the
  artifact into two files, complicating transport, storage and rollback.

## See also

- `docs/reference/formats/package-format.md` — byte-level specification.
- `docs/reference/cli/package.md` and `docs/reference/cli/sign.md` — CLI usage.
- `crates/ota-package/src/` — reference implementation.
EOF
echo "written"