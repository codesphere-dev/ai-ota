# ADR 0006: Rollback strategy

- **Status:** Accepted (policy TBD; mechanism defined)
- **Date:** 2026-11-11
- **Related:** issue #7

## Context

A newly committed model may turn out to be worse than the previous one
(latency, accuracy, stability). The device needs a way to revert without a
full re-download.

## Decision

- **Mechanism**: A/B slots (ADR 0003) make rollback a slot switch: booting the
  previous slot restores the previous model. No package re-download is needed.
- **Policy** (inference health, rollback triggers, "safe model" definition) is
  deferred to a follow-up ADR (issue #7). The mechanism is intentionally
  policy-agnostic.

## Consequences

**Positive**
- Rollback is O(1) and does not depend on network availability.

**Negative**
- Requires enough flash for two model slots.

## See also

- ADR 0003 (A/B partitions).
- Issue #7.
EOF
echo written