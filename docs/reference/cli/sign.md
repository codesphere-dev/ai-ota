# `ota sign`

Generate keys, sign packages and verify signatures.

## `ota sign keygen`

Generate an Ed25519 key pair and write it to disk.

```
ota sign keygen --output <PRIVATE_KEY_FILE> [--public-output <PUBLIC_KEY_FILE>]
```

Writes the private key to `--output` and the public key to `--public-output`,
or to `<output>.pub` when `--public-output` is omitted. Keys are stored in the
enveloped lowercase-hex format described in the package format reference.

Entropy is read from `/dev/urandom`, so key generation requires a Unix host.

## `ota sign package`

Sign an unsigned `.otapkg` with a private key.

```
ota sign package --key <PRIVATE_KEY_FILE> --input <UNSIGNED> --output <SIGNED>
```

## `ota sign verify`

Verify a signed `.otapkg` against a public key.

```
ota sign verify --key <PUBLIC_KEY_FILE> <SIGNED>
```

Verifies the Ed25519 signature over the package's signed region. Exits non-zero
on any failure.

## See also

- `.otapkg` format reference (`docs/reference/formats/package-format.md`).
- `ota package` (`docs/reference/cli/package.md`).
EOF
echo written