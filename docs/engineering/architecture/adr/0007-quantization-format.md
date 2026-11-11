# ADR 0007: Quantization and model metadata in the manifest

- **Status:** Accepted (v1 minimal; extended metadata deferred)
- **Date:** 2026-11-11
- **Related:** issue #8

## Context

ML engineers need to know what a package contains (quantization, dimensions,
provenance) without unpacking the payload. The manifest is the natural place.

## Decision

**v1 manifest carries the minimal set needed to verify and place the model:**
model name, model version, target, quantization, payload length, payload
SHA-256, chunk size, chunk count.

**Extended metadata** (arena size, opset, expected latency, license,
provenance, weight hash) is deferred to v2. Adding it later is a
backward-compatible manifest extension (new fields appended) because the
manifest is TLV-framed at the section level and length-prefixed at the field
level.

## Consequences

**Positive**
- v1 manifest stays small and parseable in constant RAM.
- No breaking change needed to add metadata later.

**Negative**
- v1 tooling cannot display extended metadata.

**Alternatives considered**
- Full metadata in v1. Rejected: the exact fields are not settled and would
  bloat the manifest before the MCU budget is measured.

## See also

- ADR 0001 (`.otapkg` format).
- Issue #8.
EOF
echo written