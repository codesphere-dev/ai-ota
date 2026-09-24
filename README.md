<div align="center">

# ai-ota

**OTA updates for embedded AI models, from training artifact to deployed fleet.**

Sign, ship, verify, roll back. Atomically. On MCU, MPU, or both.

[![Status](https://img.shields.io/badge/status-alpha-orange)]()
[![Targets](https://img.shields.io/badge/targets-MCU%20%7C%20MPU-blue)]()
[![License](https://img.shields.io/badge/license-GPL--3.0-green)]()
[![Docs](https://img.shields.io/badge/docs-diátaxis-purple)](./docs/)

</div>

---

## What is ai-ota?

`ai-ota` is an end-to-end OTA (Over-The-Air) update system purpose-built for **AI models running on embedded devices**: microcontrollers, microprocessors, and everything in between.

Existing OTA tools (Mender, RAUC, SWUpdate) treat a firmware blob as an opaque payload. `ai-ota` treats the **model** as a first-class citizen: quantization-aware packaging, delta compression tuned for weight distributions, A/B partition layouts that account for model size, and rollback policies that consider inference health, not just checksums.

**Design principles:**

- **Fleet-first**: designed for 10 devices and 10,000 alike.
- **Atomic by default**: every update is reversible, every boot is safe.
- **Delta-first**: bandwidth is the scarcest resource at the edge.
- **Secure boot to signed package**: a single chain of trust.
- **MCU + MPU, one tool**: Zephyr, FreeRTOS, Linux, Yocto, bare-metal.

---

## Features

| | |
|---|---|
| **Package format** | Custom `.otapkg`: chunked, TLV-structured, signed, streaming-friendly. |
| **Delta updates** | `zstd --patch-from` for large models, `bsdiff` fallback, MCU-optimized patcher. |
| **A/B partitions** | Atomic slot switch, metadata journal, recovery slot, wear-leveling aware. |
| **Signing** | Ed25519 (default), ECDSA P-256, RSA-3072. HSM/PKCS#11 support. |
| **Secure boot** | ROM → bootloader → agent → model runtime, verified at each link. |
| **Transports** | MQTT, HTTPS/CDN, CoAP, BLE, cellular (LTE-M/NB-IoT), LoRa. |
| **Campaigns** | Canary, phased rollout, targeting rules, auto-halt on failure thresholds. |
| **Rollback** | Automatic on healthcheck failure, manual via CLI, forced via bootloader. |
| **Fleet** | Device registry, tagging, telemetry, shadow state, compliance audit trail. |
| **HIL-ready** | Power-loss tests, fuzzed parsers, hardware-in-the-loop CI. |

---

## Quickstart

Get a device updated and rolled back in under 10 minutes.

```bash
# 1. Install
pip install ai-ota

# 2. Spin up a local OTA server + emulated device
ai-ota dev up

# 3. Build a package from your model
ai-ota package build \
  --model ./models/vision-v3.onnx \
  --target stm32h7 \
  --output vision-v3.otapkg

# 4. Sign it
ai-ota sign keygen --out keys/
ai-ota sign package vision-v3.otapkg --key keys/ed25519.pem

# 5. Publish and push
ai-ota package publish vision-v3.otapkg
ai-ota deploy push --device emu-001 --release vision-v3

# 6. Watch it install
ai-ota monitor device emu-001

# 7. Break it on purpose, then watch it roll back
ai-ota deploy push --device emu-001 --release vision-v3-broken
ai-ota monitor device emu-001    # → ROLLBACK_PENDING → ROLLED_BACK
```

Full walkthrough → docs/tutorials/quickstart/ten_minute_demo.md

## Architecture

```
┌────────────────────────────────────────────────────────────────────┐
│                         ai-ota server                             │
│                                                                    │
│  ┌──────────────┐  ┌──────────────┐  ┌───────────────────────┐    │
│  │  Package     │  │  Campaign    │  │   Device Registry     │    │
│  │  Registry    │  │  Engine      │  │   + Fleet Shadow      │    │
│  └──────┬───────┘  └──────┬───────┘  └──────────┬────────────┘    │
│         │                 │                     │                 │
│         └────────┬────────┴─────────────────────┘                 │
│                  ▼                                                │
│         ┌──────────────────┐      ┌──────────────────┐            │
│         │  Signing Service │      │  Transport Layer │            │
│         │  (HSM / KMS)     │      │  MQTT / HTTPS    │            │
│         └──────────────────┘      │  CoAP / BLE      │            │
│                                   └────────┬─────────┘            │
└────────────────────────────────────────────┼──────────────────────┘
                                             │
                          ┌──────────────────┴──────────────────┐
                          │                                     │
                    ┌─────▼─────┐                        ┌──────▼──────┐
                    │  MCU      │                        │   MPU       │
                    │  Zephyr   │                        │   Linux     │
                    │  FreeRTOS │                        │   Yocto     │
                    │  bare-mtl │                        │   Android   │
                    ├───────────┤                        ├─────────────┤
                    │ Bootloader│                        │ Bootloader  │
                    │ Slot A/B  │                        │ Slot A/B    │
                    │ Agent     │                        │ Agent       │
                    │ Runtime   │                        │ Runtime     │
                    └───────────┘                        └─────────────┘
```

Design docs → docs/explanation/architecture/

## Supported targets

| Family | Examples | Bootloader | Agent |
|---|---|---|---|
| ARM Cortex-M | STM32, nRF52, RP2040, ESP32 | MCUboot, custom | Rust / C |
| ARM Cortex-A | i.MX, RPi CM4, Rockchip | U-Boot + RAUC-like | Rust |
| RISC-V | ESP32-C, GD32V | MCUboot | Rust / C |
| x86 embedded | Intel NUC, UP Board | systemd-boot | Rust |
| RTOS | Zephyr, FreeRTOS, NuttX | native / custom | Rust / C |
| Linux distros | Yocto, Buildroot, Debian | U-Boot / GRUB | Rust |

Add a new target → docs/how_to_guides/dev/configure-hil-bench.md

## Documentation

`ai-ota` docs follow the Diátaxis framework. Pick your entry point:

| Section | What you'll find | Start here |
|---|---|---|
| Tutorials | Guided end-to-end learning | [Quickstart](docs/tutorials/quickstart/) |
| How-to guides | Recipes for a specific task | [Push an update](docs/how_to_guides/update/push-update-to-single-device.md) |
| Reference | CLI, schemas, formats, errors | [CLI reference](docs/reference/cli/) |
| Explanation | Design rationale, trade-offs | [Why A/B partitions?](docs/explanation/design-decisions/why-ab-partitions.md) |
| Engineering | ADRs, C4, requirements, tests | [Architecture index](docs/engineering/) |

## Security model

- **Signed packages**: Ed25519 by default; ECDSA P-256 / RSA-3072 available for compliance.
- **Signed manifests**: the manifest describes the payload; both are signed and verified.
- **Anti-rollback**: monotonic counters prevent downgrade attacks.
- **Secure boot chain**: ROM → bootloader → agent → runtime, verified at each stage.
- **Replay protection**: nonces + monotonic version in the transport protocol.
- **Supply chain**: SBOM emitted per release, SLSA-3 provenance, signed builds.

Threat model → docs/engineering/security/threat-model.md
Controls → docs/engineering/security/controls.md

## Status

`ai-ota` is alpha. The package format and signing scheme are stable; the CLI surface and campaign API may still change before 1.0.

| Component | Status |
|---|---|
| Package format v1 | ✅ Stable |
| Ed25519 signing | ✅ Stable |
| A/B partitions (MCU) | ✅ Stable |
| A/B partitions (MPU) | 🟡 Beta |
| Delta updates | 🟡 Beta |
| Campaign engine | 🟡 Beta |
| CoAP transport | 🟠 Alpha |
| BLE transport | 🟠 Alpha |
| LoRa fragmentation | 🔴 Planned |
| Multi-target unified campaigns | 🟠 Alpha |

## Roadmap

- [ ] **1.0**: Frozen package format, stable CLI, first HIL-validated release
- [ ] Delta updates for int4-quantized models
- [ ] On-device model fine-tuning + hot-swap (no reboot)
- [ ] SUIT / Uptane manifest compatibility
- [ ] Hardware-backed attestation (TPM 2.0, SE050, ATECC608)
- [ ] Web dashboard for fleet observability

## Contributing

We welcome contributions. Before opening a PR:

- Read docs/engineering/standards/coding-conventions.md
- Follow docs/engineering/standards/git-workflow.md
- Run the local test suite: `make test`
- HIL tests require a physical device : voir docs/engineering/tests/hil.md

Discussion happens in GitHub Discussions.
Security issues : voir [SECURITY.md] ; ne pas ouvrir de ticket public.

## License

GPL-3.0 (see [LICENSE](LICENSE)).
<div align="center"> <sub>Built for people who ship models to devices that don't come back.</sub> </div>