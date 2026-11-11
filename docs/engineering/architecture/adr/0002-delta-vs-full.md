# ADR 0002: Full packages vs. weight-space deltas

- **Status:** Accepted (for v1: full packages only)
- **Date:** 2026-11-11
- **Related:** ADR 0001 (`.otapkg` format), issue #6

## Context

Bandwidth to a device is expensive (cellular, LoRa, BLE). Shipping a full
model on every update is wasteful when only a fraction of the weights change
between versions. A delta update ("patch from base package to target package")
could save most of that bandwidth.

## Decision

**v1 ships full packages only.** The `.otapkg` format reserves the *identity*
primitives a future delta needs (model name, version, payload SHA-256, chunk
index), but no delta encoding is defined in v1.

A delta, when introduced later, will be expressed as a new package whose
manifest references a `base_package_sha256` and whose payload is a patch
stream. Verification remains anchored on the **target** package: the device
reconstructs the target payload, then checks its SHA-256 against the target
manifest. The base package is only needed to apply the patch.

## Consequences

**Positive**
- One format, one parser, one verification path for v1.
- The format does not need to change to add deltas later; only a new payload
  encoding and a manifest field.

**Negative**
- v1 cannot save bandwidth. Acceptable for the first release where correctness
  matters more than size.

**Alternatives considered**
- Ship deltas in v1. Rejected: the patcher, the base-cache management and the
  resume semantics are not designed yet; premature complexity risks bricking
  devices.

## See also

- ADR 0001 (`.otapkg` format).
- Issue #6 (weight-space delta for quantized models).
EOF
echo written