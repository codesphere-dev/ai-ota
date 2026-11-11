# `ota package`

Build and inspect `.otapkg` packages.

## `ota package build`

Assemble an unsigned `.otapkg` from a model payload file.

```
ota package build \
  --model <NAME> \
  --version <SEMVER> \
  --target <TARGET> \
  --input <PAYLOAD_FILE> \
  --output <PACKAGE_FILE> \
  [--quantization <QUANT>] \
  [--chunk-size <BYTES>]
```

Options:

- `--model` — model or artifact name, for example `vision-v3`.
- `--version` — semantic version, for example `1.2.0` or `1.2.0-rc.1`.
- `--target` — target identifier, for example `stm32h7`.
- `--input` — payload file to package.
- `--output` — destination `.otapkg` file.
- `--quantization` — `none`, `int8` (default), `int4`, `fp16`, `fp32`, `mixed`.
- `--chunk-size` — payload chunk size in bytes (default `65536`).

## `ota package inspect`

Parse a `.otapkg`, verify its payload checksums and print its manifest.

```
ota package inspect <PACKAGE_FILE>
```

Prints the model name, version, target, quantization, payload length, chunk
size, chunk count, payload SHA-256, signature status and a `payload: ok` line
when every chunk CRC32 and the overall SHA-256 match.

## See also

- `.otapkg` format reference (`docs/reference/formats/package-format.md`).
- `ota sign` (`docs/reference/cli/sign.md`).
EOF
echo written