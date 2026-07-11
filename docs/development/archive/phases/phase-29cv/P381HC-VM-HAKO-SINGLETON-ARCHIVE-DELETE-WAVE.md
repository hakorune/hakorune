# P381HC vm_hako Singleton Archive Delete Wave

Date: 2026-05-06
Scope: delete the proven zero-ref vm_hako archive singleton scripts whose live
owners were already migrated or retired, while holding the args singleton until
its remaining doc-owned acceptance refs are cleared.

## Context

`target/smoke_inventory/archive_inventory.tsv` currently classifies these four
archive singleton scripts as `orphan_candidate` with
`fullpath_ref_count = 0`, `basename_ref_count = 0`, and `suite_hit_count = 0`:

- `tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/atomic/atomic_fence_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/file/filebox_newbox_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/tls/tls_last_error_ported_vm.sh`

Delete-last is only safe where the archive inventory and the owner/retirement
SSOT agree, and where no current repo-owned acceptance doc still points at the
archive singleton.

## Proven Owner Migration / Retirement Evidence

- `atomic_fence_ported_vm.sh`
  - `docs/development/current/main/phases/phase-96x/96x-90-vm-hako-llvm-cutover-ssot.md`
    says the `atomic/` family moved to
    `apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` via `presubmit.txt`.
  - `docs/development/current/main/phases/phase-96x/README.md` and
    `96x-92-execution-plan.md` record `96xC2a` as landed retirement.
- `tls_last_error_ported_vm.sh`
  - the same phase-96x sources record the shared
    `apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` owner anchor and landed
    `96xC2a` retirement.
- `filebox_newbox_vm.sh`
  - `docs/development/current/main/phases/phase-96x/README.md` and
    `96x-91-task-board.md` record `96xC1c` as landed retirement with
    `apps/archive/phase29cc_plg04_filebox_pilot_vm.sh` as the explicit green
    anchor.

## Deleted Paths

- `tools/smokes/v2/profiles/archive/vm_hako_caps/atomic/atomic_fence_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/file/filebox_newbox_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/tls/tls_last_error_ported_vm.sh`

## Held Path

- `tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh`

Blocker evidence:

- `docs/development/current/main/phases/phase-291x/291x-253-birth-compat-deletion-criteria-card.md`
  still lists the archive script in `Related` and in `Acceptance`.
- `docs/development/current/main/phases/phase-291x/291x-254-birth-emit-kind-prune-card.md`
  still runs `bash tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh`
  in `Acceptance`.

That makes `args_vm.sh` non-clear for delete-last even though the inventory TSV
still reports `0/0/0`; the owner migration is real (`phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` via phase-96x), but the current phase-291x cards still treat this archive singleton as active acceptance evidence.

## Result

This lands only the coherent safe subset:

- no MapBox family files were touched
- no active suite-owned vm_hako owner was removed
- the args singleton stays until the phase-291x acceptance/docs stop depending
  on the archive script

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381hc_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/atomic/atomic_fence_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/file/filebox_newbox_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/tls/tls_last_error_ported_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
