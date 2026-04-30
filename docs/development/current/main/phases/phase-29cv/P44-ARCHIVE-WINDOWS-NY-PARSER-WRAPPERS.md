---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive stale Windows Ny parser wrapper scripts from the active tools root.
Related:
  - docs/development/current/main/phases/phase-29cv/P42-REPAIR-NY-ROUNDTRIP-SMOKE-CONTRACT.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - tools/archive/manual-smokes/README.md
---

# P44 Archive Windows Ny Parser Wrappers

## Goal

Apply the faster cleanup rule to platform-specific root wrappers:

```text
active refなし + current PASSなし + capsule ownerなし = archive/delete
```

## Decision

Move these root PowerShell wrappers to `tools/archive/manual-smokes/`:

- `tools/ny_parser_bridge_smoke.ps1`
- `tools/ny_parser_run.ps1`
- `tools/ny_roundtrip_smoke.ps1`

The Linux active wrappers remain in `tools/`:

- `tools/ny_parser_bridge_smoke.sh`
- `tools/ny_roundtrip_smoke.sh`
- `tools/ny_stage2_shortcircuit_smoke.sh`

## Non-goals

- do not change active Linux smoke behavior
- do not change JSON v0 loader behavior
- do not add a Windows gate

## Acceptance

```bash
test -f tools/archive/manual-smokes/ny_parser_bridge_smoke.ps1
test -f tools/archive/manual-smokes/ny_parser_run.ps1
test -f tools/archive/manual-smokes/ny_roundtrip_smoke.ps1
! rg -g '!docs/development/current/main/phases/phase-29cv/P44-ARCHIVE-WINDOWS-NY-PARSER-WRAPPERS.md' --fixed-strings \
  -e 'tools/ny_parser_bridge_smoke.ps1' \
  -e 'tools/ny_parser_run.ps1' \
  -e 'tools/ny_roundtrip_smoke.ps1' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
