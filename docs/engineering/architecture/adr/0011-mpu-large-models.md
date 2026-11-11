# ADR 0011: MPU large models (stream-apply, ExternalSlot, multi-GB resume)

- **Status:** Proposed
- **Date:** 2026-11-11
- **Related:** issue #10

## Context

MPU targets can run models of hundreds of MB or GB. Such models do not fit in
device RAM or flash in one piece, and a cellular drop mid-transfer must be
resumable without restarting.

## Decision

- **Stream-apply**: the agent applies chunks to flash as they arrive and are
  verified, never holding the whole payload in RAM.
- **ExternalSlot**: on MPUs with external flash/SD, the staging area lives on
  external storage; the active model may also live there with an on-chip
  cache.
- **Multi-GB resume**: resume is chunk-granular (ADR 0005). A transfer state
  file records the last verified chunk index; on reboot the agent resumes from
  there.
- The `.otapkg` format needs no change: chunk index + per-chunk CRC32 already
  support streaming and resume.

## Consequences

**Positive**
- Models larger than RAM/flash can be updated.
- Drops are recoverable at chunk granularity.

**Negative**
- External storage adds a failure mode (corrupt SD) that must be detected
  via per-chunk CRC.

**Alternatives considered**
- Buffer the whole payload. Rejected: impossible for GB-scale models on
  constrained devices.

## See also

- ADR 0001 (`.otapkg` format), ADR 0005 (transport/resume).
- Issue #10.
EOF
echo written