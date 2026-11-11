# ADR 0014: Commit windows and watchdog / flash-busy policy

- **Status:** Proposed
- **Date:** 2026-11-11
- **Related:** issue #13

## Context

Committing a package writes to flash, which can be slow and may starve the
watchdog. A device that resets mid-commit must recover deterministically.

## Decision

- **Commit window**: the slot-selector write is a single, short operation
  performed after full verification. Flash writes for the payload happen during
  staging, outside the commit window.
- **Watchdog**: the agent feeds the watchdog between chunks during staging; the
  commit window itself must complete within the watchdog timeout or the device
  must use a hardware watchdog with a long enough period.
- **Flash-busy**: the agent must not hold a mutex across a flash erase/write;
  flash operations are awaited, not polled in a tight loop.

## Consequences

**Positive**
- Deterministic recovery: a reset during staging re-stages; a reset during
  commit re-applies the idempotent slot selector.

**Negative**
- Per-target watchdog periods may need tuning.

## See also

- ADR 0003 (A/B partitions).
- Issue #13.
EOF
echo written