# Developer Guide

Everything you need to build, test, and contribute to `ai-ota`.

> **Audience** : this document is for people **modifying** `ai-ota`.
> If you just want to **use** it, start with [`README.md`](./README.md) or [`docs/tutorials/quickstart/`](./docs/tutorials/quickstart/).

---

## Table of contents

1. [Repository layout](#repository-layout)
2. [Prerequisites](#prerequisites)
3. [Bootstrap](#bootstrap)
4. [Build](#build)
5. [Test](#test)
6. [Run locally](#run-locally)
7. [HIL bench](#hil-bench)
8. [Workflow](#workflow)
9. [Conventions](#conventions)
10. [Architecture in 5 minutes](#architecture-in-5-minutes)
11. [Where to make changes](#where-to-make-changes)
12. [Troubleshooting](#troubleshooting)

---

## Repository layout

```
ai-ota/
├── crates/                          # Rust workspace
│   ├── ota-package/                 # Package format, manifest, TLV codec
│   ├── ota-delta/                   # Delta engine (zstd, bsdiff wrappers)
│   ├── ota-crypto/                  # Signing, verification, key management
│   ├── ota-agent/                   # Device-side agent (no_std friendly)
│   ├── ota-bootloader/              # Reference bootloader (MCU + U-Boot hooks)
│   ├── ota-server/                  # Server: registry, campaigns, targeting
│   ├── ota-transport/               # MQTT, HTTPS, CoAP, BLE abstractions
│   ├── ota-cli/                     # ai-ota admin CLI
│   └── ota-hil/                     # Hardware-in-the-loop test harness
├── python/                          # Python SDK (packaging, server, agent bindings)
│   ├── ai_ota/
│   ├── tests/
│   └── pyproject.toml
├── firmware/                        # Reference firmware for supported targets
│   ├── stm32h7/
│   ├── nrf52840/
│   ├── esp32/
│   ├── imx8/
│   └── rpi-cm4/
├── docs/                            # Diátaxis documentation
│   ├── tutorials/
│   ├── how_to_guides/
│   ├── reference/
│   ├── explanation/
│   └── engineering/                 # ADRs, C4, requirements, security, tests
├── examples/                        # End-to-end examples
├── scripts/                         # Dev, CI, and release tooling
├── docker/                          # Local stack (server, broker, registry)
├── Makefile                         # Top-level entry points
├── Cargo.toml                       # Rust workspace
└── DEV.md               # ← you are here
```

Full file tree → [`docs/reference/appendices/file-tree.md`](./docs/reference/appendices/file-tree.md)

---

## Prerequisites

| Tool | Version | Why |
|---|---|---|
| **Rust** | 1.98+ (pinned via `rust-toolchain.toml`) | Core crates, agent, bootloader, server |
| **Python** | 3.11+ | SDK, packaging, tooling |
| **Docker** | 24+ | Local stack, cross-compilation, HIL emulation |
| **Docker Compose** | v2 | Local stack orchestration |
| **`just`** or **`make`** | any | Task runner |
| **`cargo-nextest`** | latest | Faster test runs |
| **`cargo-fuzz`** | latest | Parser fuzzing (nightly) |
| **`cross`** | latest | Cross-compilation for embedded targets |
| **`qemu-system-*`** | 8.0+ | Emulated device tests |
| **`renode`** | 1.15+ | MCU emulation (alternative to QEMU) |
| **`protoc`** | 3.21+ | If you touch the transport proto |
| **`afl++`** | latest | Bootloader fuzzing |

Target-specific toolchains (only needed if you work on those targets):

- **ARM**: `gcc-arm-none-eabi`, `probe-rs`
- **RISC-V**: `gcc-riscv64-unknown-elf`
- **ESP32**: `esp-rs` toolchain via `espup`
- **i.MX**: `gcc-aarch64-linux-gnu`, Yocto SDK

Install everything at once:

```bash
make bootstrap       # detects OS, installs toolchains, sets up git hooks
```

## Bootstrap

```bash
git clone https://github.com/codesphere-dev/ai-ota
cd ai-ota
make bootstrap
```

`make bootstrap` does:

- Verifies prerequisites, installs missing ones (macOS: Homebrew, Linux: apt/dnf).
- Installs the pinned Rust toolchain from `rust-toolchain.toml` (1.98.1 + clippy/rustfmt; nightly for fuzzing; targets: thumbv7em-none-eabihf, thumbv6m-none-eabi, riscv32imac-unknown-none-elf, aarch64-unknown-linux-gnu, riscv64gc-unknown-linux-gnu, x86_64-unknown-linux-gnu).
- Creates a Python venv at `.venv/` and installs dev dependencies.
- Installs git hooks (pre-commit: fmt + clippy + tests-lite).
- Pulls Docker images for the local stack.
- Generates dev keys under `~/.ai-ota/dev-keys/` (Ed25519, never commit these).

Verify:

```bash
make doctor          # checks toolchain, ports, docker, dev keys
```

Expected output ends with `✓ All checks passed`.

## Build

The workspace is Rust-first; the Python SDK is a thin wrapper around the Rust core via PyO3.

```bash
# Everything (debug)
make build

# Release
make build-release

# A single crate
cargo build -p ota-agent

# Python SDK (editable install)
make python-build
```

### Cross-compilation

```bash
# MCU (ARM Cortex-M)
make build-mcu TARGET=stm32h7

# MPU (ARM Cortex-A, Linux)
make build-mpu TARGET=imx8

# All reference firmwares
make build-firmware
```

Cross builds go through `cross` with a Docker backend. The `.cargo/config.toml` per target pins the linker and `-C target-cpu` flags.

### Build matrix

| Target | Toolchain | Backend | Artifact |
|---|---|---|---|
| stm32h7 | thumbv7em-none-eabihf | cross | .elf, .bin, .otapkg |
| nrf52840 | thumbv7em-none-eabihf | cross | .hex, .otapkg |
| esp32c3 | riscv32imc-unknown-none-elf | esp-rs | .bin, .otapkg |
| imx8 | aarch64-unknown-linux-gnu | cross | .tar.zst, .otapkg |
| rpi-cm4 | aarch64-unknown-linux-gnu | cross | .tar.zst, .otapkg |
| native | host | n/a | used for tests |

## Test

```bash
# Fast, default for local dev
make test

# Full suite: unit + integration + property + fuzz seeds + emulated
make test-all

# A specific layer
make test-unit
make test-integration
make test-property
make test-fuzz              # requires nightly
make test-emulated          # QEMU / Renode
make test-hil               # requires a physical device
```

### Test layers

| Layer | What it covers | Runner | Time |
|---|---|---|---|
| unit | Per-crate logic | cargo nextest | <30 s |
| property | Invariants (codec round-trip, delta apply/revert) | proptest | <2 min |
| integration | Server ↔ agent ↔ transport | docker compose | <5 min |
| emulated | Full OTA cycle on QEMU/Renode | scripts/test-emulated.sh | <10 min |
| fuzz | Package parser, manifest parser | cargo-fuzz, afl++ | hours (CI: 15 min smoke) |
| HIL | Real hardware, power-loss, rollback | ota-hil harness | varies |

Rule of thumb : a PR must pass `make test-all` minus HIL. HIL runs nightly in CI and on-demand for release branches.

### Coverage

```bash
make coverage        # generates target/coverage/index.html
```

Coverage targets (see `docs/engineering/tests/coverage-and-reports.md`):

- Core crates (ota-package, ota-crypto, ota-delta): **>90 %**
- Agent, server: **>80 %**
- Bootloader: **>95 %** (it's the one you can't fix post-deploy)

## Run locally

The local stack brings up a server, an MQTT broker, a package registry, and an emulated device.

```bash
make up                 # docker compose up -d
make logs               # tail all services
make down               # tear down
```

### Services

| Service | Port | URL |
|---|---|---|
| OTA server | 8080 | http://localhost:8080 |
| MQTT broker (Mosquitto) | 1883 | n/a |
| Registry (MinIO) | 9000/9001 | http://localhost:9001 |
| Emulated device (emu-001) | n/a | Attaches to broker |

### Manual smoke test

```bash
# 1. Build a package
make example-package

# 2. Sign it
ai-ota sign package examples/vision-v3.otapkg \
  --key ~/.ai-ota/dev-keys/ed25519.pem

# 3. Publish and push
ai-ota package publish examples/vision-v3.otapkg
ai-ota deploy push --device emu-001 --release vision-v3

# 4. Watch
ai-ota monitor device emu-001
```

If something looks wrong, `make doctor` first. If that's clean, see [Troubleshooting](#troubleshooting).

### Emulated devices

```bash
# QEMU (Cortex-M)
make emu-up TARGET=stm32h7

# Renode (more accurate peripherals)
make emu-up TARGET=nrf52840 RUNTIME=renode

# Attach to logs
make emu-logs
```

The emulator is scripted to fail every 3rd update (healthcheck hook returns false) so rollback paths are exercised by default. Disable with `OTA_EMU_FAULT=0`.

## HIL bench

HIL is required for any change touching `ota-bootloader`, `ota-delta`, flash layout, or the power-loss path. See [docs/how_to_guides/testing/run-hil-tests.md](./docs/how_to_guides/testing/run-hil-tests.md).

### Supported benches

| Bench | Board | Interface |
|---|---|---|
| hil-stm32h7 | NUCLEO-H743ZI2 | ST-Link + UART |
| hil-nrf52840 | nRF52840-DK | J-Link + UART |
| hil-esp32c3 | ESP32-C3-DevKit | USB-JTAG |
| hil-imx8 | i.MX8M Mini EVK | Ethernet + UART |
| hil-rpi-cm4 | RPi CM4 IO Board | Ethernet + UART |

```bash
make hil TARGET=stm32h7 BENCH=hil-stm32h7

# Power-loss test (the one that matters)
make hil-power-loss TARGET=stm32h7 \
  CUT_AT=erase|write|verify|switch|boot
```

Power-loss tests physically cut power via a relay at a configurable point. Any PR touching the bootloader must include a power-loss report.

## Workflow

```
main ────●────●──────────────●───────●──── (releases tagged vX.Y.Z)
          \                    \
           ●───●───●  feature/xyz (squash-merged)
                        \
                         ●───●  fix/abc (squash-merged)
```

- Branch from `main`: `feature/`, `fix/`, `docs/`, `chore/`.
- Commit using **Conventional Commits**:
  `feat(agent): add exponential backoff on transport error`
- Push and open a PR. The PR template asks for:
  - What changed and why
  - Which targets it affects
  - Whether HIL is required
  - A rollback plan (yes, for code too)
- CI runs: fmt, clippy, unit, integration, doc, coverage, fuzz smoke.
- Review : at least one maintainer, plus one HIL sign-off if the bootloader or delta engine is touched.
- Merge : squash only. Delete the branch.

Full policy → [docs/engineering/standards/git-workflow.md](./docs/engineering/standards/git-workflow.md)
Review process → [docs/engineering/standards/review-process.md](./docs/engineering/standards/review-process.md)

### ADRs

Structural decisions go through an ADR before code lands.

```bash
make adr-new TITLE="use-uptane-manifests"
# → docs/engineering/architecture/adr/NNNN-use-uptane-manifests.md
```

See existing ADRs → [docs/engineering/architecture/adr/](./docs/engineering/architecture/adr/)

### Release process

```bash
make release VERSION=0.4.0 CHANNEL=beta
```

This: bumps versions, builds the matrix, signs artifacts, generates SBOM, publishes to PyPI + crates.io + registry, tags, and opens a release PR against `main`.

Full runbook → [docs/engineering/operations/runbooks/](./docs/engineering/operations/runbooks/)

## Conventions

### Rust

- `rustfmt` with the workspace config (`.rustfmt.toml`).
- `clippy` at `-D warnings`. No `#[allow(...)]` without a comment explaining why.
- `no_std` compatibility is required for `ota-agent`, `ota-bootloader`, `ota-package`. If you pull in `std`, the MCU build breaks in CI.
- Error types: `thiserror` for libraries, `anyhow` only at binary boundaries.
- No `unwrap()` / `expect()` outside tests and `main()`. Enforced by `clippy::unwrap_used`.

### Python

- `ruff` for lint + format.
- `mypy --strict` on `python/ai_ota/`.
- Public API is stable; internal modules are prefixed `_`.

### Docs

- Diátaxis. Every doc belongs to exactly one of: tutorial, how-to, reference, explanation.
- Engineering docs (ADRs, C4, requirements) live under `docs/engineering/`.
- Use `just`-style code blocks with a shell prompt for anything copy-pasteable.
- Every reference doc must have a **See also** section.

Full standards → [docs/engineering/standards/](./docs/engineering/standards/)

### Commits & PRs

| Prefix | Meaning |
|---|---|
| `feat` | New feature (minor bump) |
| `fix` | Bug fix (patch) |
| `docs` | Documentation only |
| `chore` | Tooling, deps, build |
| `refactor` | No behavior change |
| `perf` | Performance |
| `test` | Tests only |
| `security` | Security-relevant |
| `BREAKING CHANGE:` | Footer : major bump |

## Architecture in 5 minutes

Read these, in order:

1. [docs/explanation/architecture/update-pipeline-philosophy.md](./docs/explanation/architecture/update-pipeline-philosophy.md)
2. [docs/explanation/architecture/fleet-first-design.md](./docs/explanation/architecture/fleet-first-design.md)
3. [docs/explanation/architecture/rollback-safety-by-design.md](./docs/explanation/architecture/rollback-safety-by-design.md)
4. [docs/engineering/architecture/c4/containers.md](./docs/engineering/architecture/c4/containers.md)
5. [docs/engineering/architecture/ota-core/update-flow.md](./docs/engineering/architecture/ota-core/update-flow.md)

### Crate dependency graph

```
ota-package  ←  ota-delta
     ↑            ↑
     │            │
ota-crypto ───────┤
     ↑            │
     │            │
ota-agent  ←  ota-transport
     ↑            ↑
     │            │
ota-server ───────┘
     ↑
ota-cli
```

`ota-bootloader` depends only on `ota-package` + `ota-crypto` (`no_std`, no alloc in the critical path).

### Update state machine

```
IDLE
  │  check()
  ▼
CHECKING ──(no update)──▶ IDLE
  │  (update available)
  ▼
DOWNLOADING ──(error)──▶ RETRY_BACKOFF ──▶ DOWNLOADING
  │  (complete)
  ▼
VERIFYING ──(bad sig)──▶ REJECTED ──▶ IDLE
  │  (ok)
  ▼
STAGING ──(flash error)──▶ FAILED ──▶ IDLE
  │  (staged)
  ▼
REBOOT_PENDING
  │  (bootloader switches slot)
  ▼
BOOTING_NEW ──(healthcheck ok)──▶ COMMITTED ──▶ IDLE
           └──(healthcheck fail)─▶ ROLLBACK_PENDING ──▶ ROLLED_BACK ──▶ IDLE
```

Every transition is logged, persisted, and reported to the server. If you add a transition, you add a test and a doc update.

## Where to make changes

| You want to… | Touch |
|---|---|
| Change the package format | `crates/ota-package/`, write an ADR, bump the format version |
| Add a delta engine | `crates/ota-delta/`, feature-gate it |
| Support a new signature scheme | `crates/ota-crypto/`, ADR + threat model update |
| Add a transport | `crates/ota-transport/`, plus a how-to guide |
| Add a target | `firmware/<target>/`, plus a HIL recipe |
| Change the update protocol | `crates/ota-agent/` + `crates/ota-server/`, protocol version bump |
| Add a campaign strategy | `crates/ota-server/src/campaign/`, test the halt path |
| Add a CLI command | `crates/ota-cli/`, update `docs/reference/cli/` |
| Add a doc page | `docs/`, follow the Diátaxis section that fits |

If a change crosses more than two of these, open an ADR first.

## Troubleshooting

| Symptom | First thing to try |
|---|---|
| `make doctor` fails on toolchain | `make bootstrap --force` |
| Emulated device never receives updates | `docker compose logs broker` (check topic ACLs) |
| Signature verification fails locally | You're using stale dev keys : `make dev-keys-reset` |
| `cross` build hangs | Docker daemon not running, or `cross` image pull stalled |
| Python SDK can't find Rust core | `make python-build` after any Rust change |
| HIL: device bootloops after test | `ai-ota device force-boot --slot b --device <id>` |
| Fuzz finds a crash | Do not open a public issue : email security@ first |

Full troubleshooting → [docs/tutorials/appendices/troubleshooting.md](./docs/tutorials/appendices/troubleshooting.md)
Recovery runbooks → [docs/engineering/operations/runbooks/](./docs/engineering/operations/runbooks/)

## Getting help

- **Questions** : GitHub Discussions
- **Bugs** : GitHub Issues (use the bug template)
- **Security** : `remiboivin021@gmail.com` (voir [SECURITY.md](./SECURITY.md))
- **Chat** : Matrix room `#ai-ota:matrix.org` (link to be added)

<div align="center"> <sub>If you're reading this, you're about to change how updates reach devices in the field. Make it count.</sub> </div>