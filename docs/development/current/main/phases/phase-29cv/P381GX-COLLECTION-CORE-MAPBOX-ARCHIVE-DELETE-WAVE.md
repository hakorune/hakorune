# P381GX Collection-Core MapBox Archive Delete Wave

Date: 2026-05-06
Scope: delete the zero-ref `collection_core` MapBox archive smokes after the
compiler cleanup wave reached optional polish.

## Context

Post-P381GI, broad T6 smoke deletion is closed. The allowed follow-up is a
per-script delete-last slice only when the inventory proves:

- full-path refs = 0
- basename refs = 0
- suite hits = 0
- no wrapper-manifest owner remains
- an owner/local README says the live owner moved elsewhere

The refreshed inventory after P381GW showed that the archived collection-core
MapBox ported scripts satisfy that rule.

At the same time, the active owner moved away from this archive family:

- `tools/smokes/v2/profiles/integration/vm_hako_caps/README.md` says the 7 live
  `MapBox.*` owner rows now live under
  `tools/smokes/v2/profiles/integration/phase29y/hako/emit_mir/`
- the same README says `collection_core/mapbox_*` is no longer part of the live
  phase29y vm-hako acceptance gate
- archive evidence that still remains is the separate
  `profiles/archive/vm_hako_caps/mapbox/` family, not `archive/collection_core`

## Deleted Paths

- `tools/smokes/v2/profiles/archive/collection_core/mapbox_clear_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/collection_core/mapbox_delete_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/collection_core/mapbox_get_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/collection_core/mapbox_has_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/collection_core/mapbox_keys_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/collection_core/mapbox_set_ported_vm.sh`
- `tools/smokes/v2/profiles/archive/collection_core/mapbox_size_ported_vm.sh`

## Result

This is a narrow delete-last smoke cleanup slice:

- no compiler behavior changed
- no suite-protected archive bucket was touched
- no manual archive bucket was touched
- no active `tools/dev` or `tools/selfhost` keeper was touched

## Validation

```bash
SMOKE_INVENTORY_DIR=tools/smokes/v2/profiles/archive \
  SMOKE_INVENTORY_LABEL=post_p381gx_profiles_archive \
  SMOKE_INVENTORY_INCLUDE_ARCHIVE=1 \
  bash tools/checks/smoke_inventory_report.sh

test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_clear_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_delete_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_get_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_has_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_keys_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_set_ported_vm.sh
test ! -e tools/smokes/v2/profiles/archive/collection_core/mapbox_size_ported_vm.sh

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
