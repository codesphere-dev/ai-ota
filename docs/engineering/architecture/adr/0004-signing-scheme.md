# ADR 0004: Multi-stage chain of trust

- **Status:** Accepted (Ed25519 only in v1)
- **Date:** 2026-11-11
- **Related:** issues #4, #12

## Context

A device must be able to verify that a package comes from a trusted signer and
has not been tampered with, and must resist rollback to an older (possibly
vulnerable) version. The format must reserve space for this without committing
to a single algorithm.

## Decision

- **Package-level signature**: an Ed25519 signature over the signed region of
  the `.otapkg` (header + all sections before the signature section). Stored in
  the optional signature section (`0x03`), algorithm code `1`.
- **Anti-rollback counter**: a monotonic counter is carried in the manifest's
  model version (and may be extended with an explicit counter field in v2). The
  bootloader rejects a package whose counter is lower than the stored one.
- **Multi-stage**: the bootloader verifies the package signature with a key
  baked into the device (or provisioned during manufacturing). Key rotation and
  HSM integration are out of scope for v1 (see `docs/engineering/security/signing/hsm.md`).

ECDSA-P256 (`2`) and RSA-3072 (`3`) are reserved algorithm codes for future
work; only Ed25519 is implemented in v1.

## Consequences

**Positive**
- One algorithm, one verification path, no_std-friendly.
- The format does not need to change to add algorithms later.

**Negative**
- Devices must be provisioned with an Ed25519 public key at manufacturing.
- No key rotation in v1.

**Alternatives considered**
- Multiple signatures per package. Rejected for v1: complicates the parser and
  is not needed until multi-party signing lands.

## See also

- ADR 0001 (`.otapkg` format).
- `docs/engineering/security/signing/`.
EOF
echo written