---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: classify Stage-B/Core/JoinIR fixture helper callers and prune an unused fallback helper.
Related:
  - docs/development/current/main/design/json-v0-route-map-ssot.md
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/selfhost/lib/stageb_program_json_capture.sh
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
---

# P65 Fixture Caller Ownership Split

## Goal

Stop reading `stageb_helpers.sh` as only a phase29bq MirBuilder keeper. It is
also the fixture helper for current Stage-B/Core smoke lanes.

## Decision

- classify `stageb_emit_program_json_v0_fixture()` as the direct Stage-0
  Program(JSON v0) fixture emitter for phase29bq `.hako` MirBuilder pins
- classify `stageb_compile_to_json*()` and related assertion helpers as the
  Stage-B compiler stdout -> Program(JSON v0) fixture path for Stage-B/Core
  smoke lanes
- keep `tools/selfhost/lib/stageb_program_json_capture.sh` as the shared
  Stage-B stdout extraction SSOT
- delete the unused `stageb_compile_via_rust_mir()` fallback helper from
  `stageb_helpers.sh`

## Non-goals

- do not rewrite Stage-B/Core smokes to MIR-first in this card
- do not delete `program_json_v0_compat.sh`
- do not weaken the phase29bq Program(JSON) contract pin

## Acceptance

```bash
bash -n tools/smokes/v2/lib/stageb_helpers.sh tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
! rg --fixed-strings "stageb_compile_via_rust_mir" tools/smokes/v2/lib/stageb_helpers.sh
./tools/smokes/v2/run.sh --profile integration --filter 'phase29bq_hako_program_json_contract_pin_vm.sh'
bash tools/smokes/v2/profiles/integration/stageb/stageb_binop_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
