# ADR 0010: Target fit and memory budget

- **Status:** Proposed
- **Date:** 2026-11-11
- **Related:** issue #9

## Context

Embedded targets range from Cortex-M0 (tens of KB RAM) to MPUs with hundreds
of MB. A model that fits one target may not fit another. The package format
must be evaluable against a target's constraints before any bytes are written
to flash.

## Decision

- **Target profile**: each target declares `ram_bytes`, `flash_bytes`,
  `max_package_bytes`, and `parse_budget_bytes` (RAM available to the parser
  during streaming verification).
- **Fit check**: `ota package build` (and a future `ota model fit`) compares
  the model's declared arena size and the package's payload size against the
  target profile and reports pass/fail with the limiting resource.
- **Parse budget**: the `.otapkg` parser must operate in O(1) RAM relative to
  payload size (streaming, one chunk at a time). The target's
  `parse_budget_bytes` sets the ceiling; v1 targets <= 16 KB.
- **Model metadata** (arena size, opset) needed for the fit check is part of
  the extended manifest (ADR 0007, deferred to v2); until then, the fit check
  uses payload size and a user-supplied `--arena` hint.

## Consequences

**Positive**
- A model that cannot fit is rejected before flashing, not after.

**Negative**
- v1 fit checks are coarse (payload size + hint) until extended metadata lands.

**Alternatives considered**
- Per-target package variants. Rejected: doubles the test surface.

## See also

- ADR 0001 (`.otapkg` format), ADR 0007 (metadata).
- Issue #9.
EOF
echo written