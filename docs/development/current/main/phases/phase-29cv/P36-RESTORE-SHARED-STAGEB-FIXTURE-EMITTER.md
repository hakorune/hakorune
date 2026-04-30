---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: restore the shared Stage-B fixture emitter as a thin wrapper over the current compat SSOT after P35 validation exposed live phase29bq callers.
Related:
  - docs/development/current/main/phases/phase-29cv/P26-DELETE-DEAD-STAGEB-FIXTURE-EMIT-WRAPPER.md
  - docs/development/current/main/phases/phase-29cv/P35-ARCHIVE-WEAK-STAGEB-SHAPE-CANARIES.md
  - tools/smokes/v2/lib/stageb_helpers.sh
  - tools/lib/program_json_v0_compat.sh
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
---

# P36 Restore Shared Stage-B Fixture Emitter

## Goal

Keep the stronger phase29bq Program(JSON) contract keepers live while avoiding
direct raw compat call duplication across many fixture scripts.

P35 validation showed that `phase29bq_hako_program_json_contract_pin_vm.sh` and
many `phase29bq_hako_mirbuilder_*` fixture pins still call
`stageb_emit_program_json_v0_fixture()`. The earlier P26 deletion was therefore
too aggressive for the current keeper set.

## Decision

- restore `stageb_emit_program_json_v0_fixture()` in `stageb_helpers.sh`
- implement it as a thin wrapper over
  `program_json_v0_compat_emit_to_file "$NYASH_BIN" ...`
- keep `tools/lib/program_json_v0_compat.sh` as the raw compat SSOT
- sync the historical inventory/docs so they no longer claim the helper is dead

## Non-goals

- do not reintroduce broad wrapper layering beyond this shared fixture helper
- do not change fixture assertions or contract semantics
- do not revive public Program(JSON) routes

## Acceptance

```bash
bash -n \
  tools/smokes/v2/lib/stageb_helpers.sh \
  tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
./tools/smokes/v2/run.sh --profile integration --filter 'phase29bq_hako_program_json_contract_pin_vm.sh'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
