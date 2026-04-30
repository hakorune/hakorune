---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: archive the weaker phase2160 Stage-B Program(JSON) shape canaries after confirming stronger contract coverage remains in phase29bq.
Related:
  - docs/development/current/main/phases/phase-29cv/P33-DELETE-LAST-BLOCKERS-ONLY.md
  - docs/development/current/main/phases/phase-29cv/P34-SYNC-DELETE-LAST-BLOCKER-GUIDANCE.md
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
  - tools/smokes/v2/profiles/integration/core/phase2160/run_all.sh
  - tools/smokes/v2/run_quick.sh
---

# P35 Archive Weak Stage-B Shape Canaries

## Goal

Reduce duplicate Program(JSON v0) proof surface without weakening the actual
keeper contract.

The phase2160 Stage-B shape canaries only asserted lightweight Program(JSON)
shapes such as `Program`, `Local`, `Loop`, `Return`, or `Method:length`. The
stronger `phase29bq_hako_program_json_contract_pin_vm.sh` already keeps the
explicit Program(JSON)->.hako contract pinned with stronger shape checks plus
runtime `--mir-json-file` execution checks.

## Decision

- archive the weaker phase2160 Stage-B shape canaries
- remove their active suite references
- keep `phase29bq_hako_program_json_contract_pin_vm.sh` as the live stronger
  Program(JSON) contract keeper

## Archived Files

- `tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_shape_canary_vm.sh`
- `tools/smokes/v2/profiles/integration/core/phase2160/stageb_program_json_method_shape_canary_vm.sh`
- `tools/smokes/v2/profiles/integration/core/phase2160/stageb_multi_method_shape_canary_vm.sh`

## Non-goals

- do not archive `phase29bq_hako_program_json_contract_pin_vm.sh`
- do not change `stageb_helpers.sh` or `stageb_program_json_capture.sh`
- do not weaken Program(JSON)->.hako fixture coverage

## Acceptance

```bash
bash -n \
  tools/smokes/v2/run_quick.sh \
  tools/smokes/v2/profiles/integration/core/phase2160/run_all.sh \
  tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh
./tools/smokes/v2/run.sh --profile integration --filter 'phase29bq_hako_program_json_contract_pin_vm.sh'
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
