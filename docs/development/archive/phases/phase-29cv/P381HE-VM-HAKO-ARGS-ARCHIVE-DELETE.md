# P381HE vm_hako args Archive Delete

Date: 2026-05-06
Scope: clear the last doc-fix-first hold for the archived vm_hako args singleton,
then delete only `tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh`.

## Context

The delete-last proof is now coherent:

- `target/smoke_inventory/archive_inventory.tsv` classifies
  `tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh` as
  `orphan_candidate` with `fullpath_ref_count = 0`, `basename_ref_count = 0`,
  and `suite_hit_count = 0`.
- `docs/development/current/main/phases/phase-96x/96x-92-execution-plan.md`,
  `96x-91-task-board.md`, and `96x-90-vm-hako-llvm-cutover-ssot.md` already
  record `96xC1a` as landed retirement with
  `tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
  as the explicit green replacement anchor.
- The remaining hold was docs-only: phase-291x cards still listed the archive
  smoke in `Related` / `Acceptance`, and the phase-29y feature matrix still
  pointed RVP-C02 at a stale non-existent smoke path.

## Updated Doc Pointers

- `docs/development/current/main/phases/phase-291x/291x-253-birth-compat-deletion-criteria-card.md`
  now points at the phase-96x cutover proof plus the live
  `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` owner anchor.
- `docs/development/current/main/phases/phase-291x/291x-254-birth-emit-kind-prune-card.md`
  now treats the args row as retired-by-phase-96x instead of archive-owned.
- `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
  now points RVP-C02 at the live replacement smoke.

## Deleted Path

- `tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh`

## Held Path

- `tools/smokes/v2/profiles/archive/vm_hako_caps/args/boxcall_args_gt1_ported_vm.sh`

The held `boxcall_args_gt1` witness stays untouched because phase-96x still
documents it as retired APP-1 seam evidence without an exact non-vm_hako live
owner.

## Result

This slice only resolves the args archive singleton:

- the live owner proof stays phase-96x + `phase29x_runtime_data_dispatch_llvm_e2e_vm.sh`
- phase-291x birth docs no longer depend on a deleted archive smoke
- no non-args archive family changed

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381he_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/args/boxcall_args_gt1_ported_vm.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
