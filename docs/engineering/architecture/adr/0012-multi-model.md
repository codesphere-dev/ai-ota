# ADR 0012: Multi-model on-device

- **Status:** Proposed
- **Date:** 2026-11-11
- **Related:** issue #11

## Context

A device may run several models (e.g. a detector and a classifier) that
update independently. The single-model v1 assumption must be relaxed without
breaking the package format.

## Decision

- **Slot index**: each model occupies a named slot keyed by `model_name`.
  Slots are independent: updating one model does not affect others.
- **Hot-swap**: a model can be swapped at runtime only if the runtime
  supports it (inference engine dependent); otherwise the swap happens on next
  boot.
- **Shared memory**: models may share a common arena/pool when the inference
  engine allows it; the package format carries no such information (it is a
  runtime concern).
- **No format change**: a device holds several `.otapkg` files, each
  identified by `model_name` + `model_version`.

## Consequences

**Positive**
- Independent updates per model.
- No change to `.otapkg`.

**Negative**
- Slot management and memory sharing are runtime concerns outside the format.

**Alternatives considered**
- One combined package for all models. Rejected: forces all-or-nothing updates
  and wastes bandwidth.

## See also

- ADR 0001 (`.otapkg` format), ADR 0008 (fleet).
- Issue #11.
EOF
echo written