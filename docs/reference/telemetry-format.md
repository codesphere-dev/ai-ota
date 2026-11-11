# Inference telemetry event schema

Reference for the events emitted by the device agent and consumed by the fleet
server. All events are JSON objects sent over the transport (MQTT/HTTPS).

## Events

### `model_committed`

Emitted when a staged package is committed and becomes the active model.

```json
{
  "event": "model_committed",
  "device_id": "string",
  "model_name": "string",
  "model_version": "string",
  "package_sha256": "hex (64 chars)",
  "timestamp_ms": 0,
  "slot": "A | B"
}
```

### `health_score`

Emitted periodically or after an inference run.

```json
{
  "event": "health_score",
  "device_id": "string",
  "model_name": "string",
  "model_version": "string",
  "score": 0.0,
  "metric": "accuracy | latency_p99_ms | error_rate",
  "timestamp_ms": 0
}
```

### `rollback_reason`

Emitted when the device rolls back to the previous model.

```json
{
  "event": "rollback_reason",
  "device_id": "string",
  "from_version": "string",
  "to_version": "string",
  "reason": "health_threshold | watchdog | manual | corrupt",
  "timestamp_ms": 0
}
```

### `delta_saved_bytes`

Emitted after a delta update is applied, reporting bandwidth saved.

```json
{
  "event": "delta_saved_bytes",
  "device_id": "string",
  "model_name": "string",
  "base_version": "string",
  "target_version": "string",
  "full_size_bytes": 0,
  "delta_size_bytes": 0,
  "saved_bytes": 0,
  "timestamp_ms": 0
}
```

## Common fields

- `device_id` — stable device identifier.
- `timestamp_ms` — Unix epoch milliseconds.
- All string versions follow the `MAJOR.MINOR.PATCH[-pre]` form.

## See also

- ADR 0008 (fleet orchestration).
- Issue #19.
EOF
echo written