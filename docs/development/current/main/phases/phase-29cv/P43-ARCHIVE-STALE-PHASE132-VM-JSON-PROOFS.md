---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive stale phase-132x root proof wrappers that are no longer active gates.
Related:
  - docs/development/current/main/phases/phase-132x/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - tools/archive/manual-smokes/README.md
---

# P43 Archive Stale Phase-132x VM/JSON Proofs

## Goal

Stop carrying failed root proof wrappers as active keepers after the Program(JSON
v0) and explicit-VM cleanup made stronger or newer proof routes authoritative.

## Decision

Move these root wrappers to `tools/archive/manual-smokes/`:

- `tools/selfhost_json_guard_smoke.sh`
- `tools/selfhost_parser_json_smoke.sh`
- `tools/ny_stage2_new_method_smoke.sh`

They are preserved as historical/manual evidence only. They are not active
gates and should not be used as proof for the current MIR-first route.

## Current Failure Reading

- `selfhost_json_guard_smoke.sh` now trips the runtime route contract instead
  of proving a current default route.
- `selfhost_parser_json_smoke.sh` trips the old selfhost compiler JSON smoke
  path with `Undefined variable: StageBMod`.
- `ny_stage2_new_method_smoke.sh` trips an old VM/ConsoleBox method expectation
  in the current environment.

## Non-goals

- do not touch active `tools/ny_stage2_shortcircuit_smoke.sh`
- do not touch active `tools/ny_parser_bridge_smoke.sh`
- do not change VM or JSON loader behavior

## Acceptance

```bash
bash -n \
  tools/archive/manual-smokes/selfhost_json_guard_smoke.sh \
  tools/archive/manual-smokes/selfhost_parser_json_smoke.sh \
  tools/archive/manual-smokes/ny_stage2_new_method_smoke.sh
! rg -g '!docs/development/current/main/phases/phase-29cv/P43-ARCHIVE-STALE-PHASE132-VM-JSON-PROOFS.md' --fixed-strings \
  -e 'tools/selfhost_json_guard_smoke.sh' \
  -e 'tools/selfhost_parser_json_smoke.sh' \
  -e 'tools/ny_stage2_new_method_smoke.sh' \
  docs/development/current/main docs/development/testing tools src lang Makefile dev README.md README.ja.md
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
