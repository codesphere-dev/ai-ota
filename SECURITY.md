# Security Policy

`ai-ota` moves signed software to devices in the field. A vulnerability in the package format, signing scheme, device agent, or server side of that chain can put fleets at risk. We take this seriously and ask that you do too: **report security issues privately, never in a public issue.**

## Scope

The following are in scope for security reports:

- Package format codec (`ota-package`), delta engine (`ota-delta`)
- Signing, key management, verification (`ota-crypto`)
- Bootloader and secure-boot chain (`ota-bootloader`)
- Device agent and its transports (`ota-agent`, `ota-transport`)
- OTA server, campaign engine, device registry (`ota-server`)
- CLI (`ota-cli`) and Python SDK
- Build, signing, and release pipelines (supply chain)

Out of scope: use GitHub Issues instead:

- General bugs with no security impact
- Documentation issues
- Misconfigurations of a user's own environment

## Reporting a vulnerability

Send a private report to **remiboivin021@gmail.com**.

Include as much of the following as you can:

- Affected component(s) and versions
- Description of the vulnerability and its potential impact
- Attack scenario (who can exploit it, from where)
- Reproduction steps or proof of concept
- Suggested fix or mitigation, if you have one

If you don't get an acknowledgment within **48 hours**, follow up. Do not open issues or PRs that detail a vulnerability before it is fixed.

## What happens next

| Step | Target |
|---|---|
| Acknowledgment | 48 h |
| Triage and severity assessment | 5 business days |
| Fix / mitigation plan | 10 business days |
| Coordinated public disclosure | 90 days after fix ships (or sooner if agreed) |

We follow **coordinated disclosure**: we will keep you informed as a fix lands, credit you in the advisory (unless you prefer anonymity), and coordinate the public announcement with you.

## Our security model

The controls we rely on are documented under [`docs/engineering/security/`](./docs/engineering/security/):

- [Threat model](./docs/engineering/security/threat-model.md): MITM, replay, rollback, supply-chain attacks
- [Controls](./docs/engineering/security/controls.md): signing, secure boot, anti-rollback, replay protection
- [Key management](./docs/engineering/security/signing/key-management.md) and [HSM](./docs/engineering/security/signing/hsm.md)
- [Compliance](./docs/engineering/security/compliance.md): UNECE R156, ISO 24089, IEC 62443

## Handling compromise

If you know or suspect that signing keys have been compromised, treat it as the highest-priority incident and follow the [key-compromise runbook](./docs/engineering/operations/runbooks/key-compromise.md), then email remiboivin021@gmail.com.

## Recognition

We are grateful to researchers who report responsibly. With your consent, your name will be listed in the advisory. We do not currently run a paid bug bounty program.