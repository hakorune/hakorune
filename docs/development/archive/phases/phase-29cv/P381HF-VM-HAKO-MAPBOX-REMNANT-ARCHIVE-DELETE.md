# P381HF vm_hako MapBox Remnant Archive Delete

Date: 2026-05-06
Scope: reconcile the stale doc pointers for the five held vm_hako MapBox
bad-key/missing-key witnesses, then delete those archive-only remnants without
touching `selfhost_stageb_oob_vm.sh` or any non-MapBox archive family.

## Context

`target/smoke_inventory/t6_profiles_archive_candidate_paths.tsv` classifies all
five paths below as `orphan_candidate`, matching the recent delete-ready
inventory conclusion once doc-fix-first holds are cleared:

- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_getfield_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_set_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_setfield_bad_key_ported_vm.sh`

The doc-fix-first hold came from stale owner pointers in current docs:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md`
- `docs/development/current/main/phases/phase-29y/60-NEXT-TASK-PLAN.md`
- `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
- `docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md`
- `docs/development/current/main/phases/phase-291x/291x-101-mapbox-get-missing-key-contract-card.md`
- `docs/development/current/main/investigations/current_task_archive_2026-03-22.active-state.md`

## Live Owner / Replacement Evidence

- `MapBox.get(missing-key)`
  - source-route witness:
    `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_get_missing_vm.sh`
  - quick-core guard:
    `tools/smokes/v2/profiles/quick/core/map/map_missing_key_tag_vm.sh`
- `MapBox.get(non-string key)` and `MapBox.set(non-string key, value)`
  - source-route witness:
    `tools/smokes/v2/profiles/integration/apps/phase291x_mapbox_hako_bad_key_vm.sh`
- `MapBox.getField/setField(non-string key, ...)`
  - contract card:
    `docs/development/current/main/phases/phase-291x/291x-100-mapbox-bad-key-contract-card.md`
  - live owner frontier:
    `lang/src/runtime/collections/map_state_core_box.hako`
  - dispatch entry:
    `lang/src/vm/boxes/mir_vm_s0_boxcall_builtin.hako`
  - owner-frontier closeout note:
    `docs/development/current/main/phases/phase-29cm/README.md`

## Deleted Paths

- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_getfield_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_set_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_setfield_bad_key_ported_vm.sh`

## Result

This slice lands the full held MapBox remnant delete wave after pointer cleanup:

- current docs now point at the live phase-291x or owner-frontier evidence
- the five archive-only MapBox remnants are removed
- `selfhost_stageb_oob_vm.sh` stays untouched
- no non-MapBox archive family changed

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381hf_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_bad_key_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_getfield_bad_key_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_set_bad_key_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_setfield_bad_key_ported_vm.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
