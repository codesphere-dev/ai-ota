# ADR 0003: A/B partitions and commit points

- **Status:** Accepted (sketch; per-target details in follow-ups)
- **Date:** 2026-11-11
- **Related:** issues #3, #5, #12, #13

## Context

A device needs a safe place to stage an incoming package and a way to commit
it atomically, surviving power loss mid-write. The package format defines the
artifact; the storage layout defines where it lands.

## Decision

- **Two slots (A/B) for the active model**, plus a staging area for the
  incoming package. The bootloader selects the slot with the higher valid
  version on boot.
- **The commit point is anchored on per-target erase boundaries**: a package is
  considered staged only after its last chunk is written and its manifest
  SHA-256 verified; the swap is a single metadata write (slot selector +
  version) performed after verification.
- **Power loss before commit** leaves the active slot untouched; the device
  re-stages on next boot.
- **Power loss during commit** is handled by an idempotent slot selector that
  can be re-applied.

Per-silicon slot layouts (MCUboot, otadata, RP2040 XIP, U-Boot+RAUC) and
watchdog/flash-busy policy are deferred to follow-up ADRs (#12, #13).

## Consequences

**Positive**
- The package format stays storage-agnostic; only the bootloader knows the
  layout.
- Rollback is "boot the other slot".

**Negative**
- Doubles flash usage for the model. Acceptable for v1.

**Alternatives considered**
- Single slot with journal. Rejected: a failed write can brick the device; A/B
  is strictly safer and simpler to reason about.

## See also

- ADR 0001 (`.otapkg` format).
- Issues #3, #5, #12, #13.
EOF
echo written