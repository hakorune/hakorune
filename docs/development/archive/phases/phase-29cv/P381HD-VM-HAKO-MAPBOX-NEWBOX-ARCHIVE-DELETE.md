# P381HD vm_hako MapBox newbox Archive Delete

Date: 2026-05-06
Scope: delete only the zero-ref archived vm_hako `newbox(MapBox)` witness while
leaving the held MapBox bad-key/missing-key/getField/setField archive witnesses
untouched.

## Context

Inventory and owner docs still support a single-file delete-last slice:

- `target/smoke_inventory/archive_inventory.tsv` currently classifies
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_newbox_ported_vm.sh`
  as `orphan_candidate` with `fullpath_ref_count = 0`,
  `basename_ref_count = 0`, and `suite_hit_count = 0`.
- `docs/development/current/main/phases/phase-96x/96x-92-execution-plan.md`
  records `mapbox_newbox_ported_vm.sh` in the non-live archive-only MapBox rows.
- `docs/development/current/main/phases/phase-29y/81-RUST-VM-TO-HAKO-VM-FEATURE-MATRIX.md`
  records RVP-C16 as ported with the live owner moved to
  `tools/smokes/v2/profiles/integration/apps/vm_hako_caps_mapbox_newbox_ported_vm.sh`.

The held MapBox archive witnesses remain separate runtime-contract evidence:

- RVP-C24 missing-key:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh`
- RVP-C25 bad-key get:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_bad_key_ported_vm.sh`
- RVP-C26 bad-key set:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_set_bad_key_ported_vm.sh`
- RVP-C27 bad-key getField:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_getfield_bad_key_ported_vm.sh`
- RVP-C28 bad-key setField:
  `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_setfield_bad_key_ported_vm.sh`

This slice does not reopen those witnesses.

## Deleted Path

- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_newbox_ported_vm.sh`

## Held Paths

- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_getfield_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_set_bad_key_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_setfield_bad_key_ported_vm.sh`

## Result

This lands only the `newbox(MapBox)` archive singleton removal:

- no active owner moved
- no bad-key/missing-key/getField/setField witness changed
- no non-MapBox archive family changed

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381hd_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_newbox_ported_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_bad_key_ported_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_get_missing_ported_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_getfield_bad_key_ported_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_set_bad_key_ported_vm.sh
test -e tools/smokes/v2/profiles/archive/vm_hako_caps/mapbox/mapbox_setfield_bad_key_ported_vm.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
