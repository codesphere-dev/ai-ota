# ADR 0008: Fleet orchestration and multi-model on-device

- **Status:** Accepted (single-model v1; multi-model deferred)
- **Date:** 2026-11-11
- **Related:** issues #11, #19

## Context

A fleet may need several models on one device (e.g. a detector and a
classifier), and the server must orchestrate rollouts across thousands of
devices. Both concerns are large and not needed for the first release.

## Decision

**v1 is single-model per device.** The package format identifies one model per
package (`model_name` + `model_version`); a device holds one active model.

Multi-model (slot index, hot-swap, shared memory) and fleet orchestration
(campaigns, staged rollouts, telemetry) are deferred to follow-up ADRs
(issues #11, #19). The package format does not need to change to support them:
a device simply holds several packages, each identified by `model_name`.

## Consequences

**Positive**
- v1 scope stays focused on the single most important path.

**Negative**
- No multi-model or fleet features in v1.

## See also

- ADR 0001 (`.otapkg` format).
- Issues #11, #19.
EOF
echo written