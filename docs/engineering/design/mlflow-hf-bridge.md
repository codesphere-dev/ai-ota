# MLflow / HuggingFace to registry bridge

Design note for issue #20.

## Goal

Let ML engineers publish a model from MLflow or HuggingFace directly to the
ai-ota registry, carrying the model card, dataset provenance and a
weight-hash for deduplication, without manual packaging.

## Flow

1. `ota model import --from mlflow://run/<id>` or `--from hf://<org>/<repo>`:
   - Fetch the model artifact and its metadata.
   - Compute `weight_sha256` over the raw weights.
   - Build an `.otapkg` via `ota package build` with the model card fields
     mapped into the extended manifest (ADR 0007, v2).
2. Dedup: if `weight_sha256` already exists in the registry, skip upload and
   return the existing package id.
3. Attach provenance: dataset id/version, training run id, framework, license.
4. Push to the registry; the registry validates the package (parse + verify).

## Model card mapping

| MLflow/HF field | Manifest field (v2) |
| --- | --- |
| model name | `model_name` |
| version | `model_version` |
| license | `license` (v2) |
| tags | `tags` (v2) |
| dataset | `dataset_provenance` (v2) |
| weight hash | `weight_sha256` (v2) |

## Out of scope for v1

- v1 manifest has no extended fields (ADR 0007); the bridge requires v2.
- Registry server implementation is separate.

## See also

- ADR 0007 (quantization and metadata).
- Issue #20.
EOF
echo written