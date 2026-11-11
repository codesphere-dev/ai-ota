# ADR 0005: Transport constraints on the format

- **Status:** Accepted
- **Date:** 2026-11-11
- **Related:** issue #14

## Context

Packages are delivered over transports with very different MTUs and duty cycles
(BLE, CoAP, LoRa, HTTPS). The format must not assume a particular transport
while still enabling streaming and resume.

## Decision

- **Chunk boundaries are a format concern; framing is a transport concern.**
  The format defines a chunk index and per-chunk CRC32. Transports decide how
  to frame, retry and resume those chunks.
- **Chunk size is configurable at build time** (`--chunk-size`, default 64 KiB)
  so a package can be tuned for its transport without changing the format.
- **Resume is chunk-granular**: a device that loses connectivity mid-transfer
  resumes from the last fully received and CRC-verified chunk.
- **No transport-specific fields** appear in the manifest or header.

## Consequences

**Positive**
- One format across all transports.
- Resume works uniformly.

**Negative**
- Very small MTUs (e.g. LoRa) may need transport-level fragmentation on top of
  chunks, adding complexity in the agent.

**Alternatives considered**
- Per-transport package variants. Rejected: doubles the test surface and
  forces tooling to handle multiple formats.

## See also

- ADR 0001 (`.otapkg` format).
- Issue #14.
EOF
echo written