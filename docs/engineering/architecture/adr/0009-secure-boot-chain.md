# ADR 0009: Secure boot chain

- **Status:** Accepted (sketch; per-target details in follow-ups)
- **Date:** 2026-11-11
- **Related:** issues #4, #12

## Context

The package signature (ADR 0004) protects the package, but a device must also
ensure that the bootloader and firmware running the verification are themselves
trusted. This is the secure boot chain.

## Decision

- **Anchor**: a root of trust baked into the device (OTP/eFuse or immutable
  ROM) holds the public key used to verify the bootloader.
- **Chain**: bootloader verifies firmware/agent; firmware verifies the package
  (ADR 0004). Each stage's signature is checked before handing control to the
  next.
- **Anti-rollback**: each stage carries a monotonic version; a lower version is
  rejected.
- Per-silicon secure boot mechanisms (STM32, nRF, RP2040, i.MX) are deferred
  to follow-up work (issue #12).

## Consequences

**Positive**
- A compromised package cannot run even if an attacker controls the update
  channel.

**Negative**
- Per-silicon details remain unspecified; porting requires per-target work.

## See also

- ADR 0004 (signing scheme).
- Issue #12.
EOF
echo written