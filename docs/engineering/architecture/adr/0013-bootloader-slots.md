# ADR 0013: Bootloader and slots per silicon

- **Status:** Proposed
- **Date:** 2026-11-11
- **Related:** issue #12

## Context

The A/B slot layout (ADR 0003) must be realised differently on each silicon
family: MCUboot on STM32, otadata on ESP32, XIP on RP2040, U-Boot+RAUC on
Linux MPUs. The package format is storage-agnostic, but the bootloader is not.

## Decision

- **Common contract**: every bootloader exposes `stage(package)`,
  `commit()`, `rollback()` and `active_slot()`. The package format is the only
  input; storage layout is internal.
- **Per-silicon adapters**: each target provides an adapter implementing the
  contract over its native mechanism (MCUboot, otadata, XIP, U-Boot+RAUC).
- **Slot addressing** is by (model_name, slot_index) and is opaque to the
  package.

## Consequences

**Positive**
- One bootloader contract; multiple implementations.

**Negative**
- Per-silicon work remains substantial.

**Alternatives considered**
- A single universal bootloader. Rejected: cannot cover XIP or MPU boot flows.

## See also

- ADR 0003 (A/B partitions), ADR 0009 (secure boot).
- Issue #12.
EOF
echo written