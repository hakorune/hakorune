---
Status: Active
Date: 2026-04-26
Scope: Canonical smoke index for the phase-291x CoreBox / CoreMethodContract cleanup lane.
Related:
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/CURRENT_STATE.toml
---

# Phase 291x Smoke Index

This index keeps the active confirmation smokes in one place so cards can stay
small. It is intentionally grouped by family instead of listing every exact
boundary variant.

## Canonical Smokes

| Family | Canonical smoke(s) | Typical cards | Purpose | Notes |
| --- | --- | --- | --- | --- |
| MapBox direct has / metadata-absent fallback | `tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh` | `291x-271` | Pins direct `MapBox.has` with no `generic_method.has` metadata through the MIR-call surface fallback pair. | This is the active proof that the remaining `MapBox` receiver-surface and `has` method-surface rows are paired fallback debt, not removable independently. |
| RuntimeData has facade metadata | `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_has_facade_min.sh` | `291x-273` | Pins `generic_method.has route_kind=runtime_data_contains_any` without a CoreMethod carrier. | This is the declaration-demand proof for `nyash.runtime_data.has_hh` when the helper is selected by route metadata. |
| ArrayBox direct has / ArrayHas metadata | `tools/smokes/v2/profiles/integration/phase29ck_boundary/array/phase29ck_boundary_pure_array_has_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh` | `291x-268` `291x-269` `291x-270` | Pins direct `ArrayBox.has` and Array-origin `RuntimeDataBox.has` through `ArrayHas` metadata. | This row confirms the direct ArrayBox and RuntimeDataBox has fallback prunes; remaining has cleanup is generic emit-kind / MIR-surface only. |
| ArrayBox direct get / route-policy prune | `tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh`<br>`tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh`<br>`tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` | `291x-199` `291x-200` `291x-235` `291x-240` `291x-247` | Pins direct ArrayBox get metadata, runtime-data dispatch, and route-policy prune behavior. | `pure-historical` is still useful as a representative canary, but the phase29ck boundary smoke is the active daily confirmation. |
| MapBox direct get / has / len route-policy prune | `tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh`<br>`tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh`<br>`tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` | `291x-240` `291x-241` `291x-242` `291x-244` `291x-245` | Confirms the direct MapBox route pruning and the remaining RuntimeData fallback contract. | `MapBox.has` and `MapBox.len/size` already have their own consumer smokes; this row only covers the route-policy side. |
| ArrayBox string-observer boundary repairs | `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_select_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min.sh` | `291x-238` | Pins the string-observer `indexOf` route-state and same-slot live-follow-up behavior. | These six scripts are one family; keep them together instead of scattering them across cards. |
| ArrayBox string len / substring boundary repairs | `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh`<br>`tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`<br>`tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh` | `291x-239` | Confirms len-window source tracking and substring follow-up metadata after same-slot set / concat patterns. | The phase137x pair is still a useful regression pair even though the lane is otherwise observe-only. |
| Set / metadata-first consumer batches | `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_string_length_min.sh`<br>`tools/smokes/v2/profiles/quick/core/map/map_len_size_vm.sh`<br>`tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh` | `291x-242` `291x-243` `291x-245` `291x-246` | Confirms the metadata-first consumer reordering for len/push and the pinned fallback contracts that prevent over-pruning. | These scripts are mostly confirmation packs, not separate product behaviors. Group them by family instead of duplicating them in each card. |
| Contract guard bundle | `bash tools/checks/current_state_pointer_guard.sh`<br>`bash tools/checks/core_method_contract_inc_no_growth_guard.sh`<br>`bash tools/checks/dev_gate.sh quick` | all active 291x cards | Keeps the current state, `.inc` mirror growth, and quick gate contract pinned. | This is the lightest stable bundle to run after any docs/code slice. |

## Redundancy Notes

- The current smoke set is already separated enough by family.
- The main consolidation opportunity is documentation, not removal:
  keep one canonical smoke row per family and stop repeating the same script list in every card.
- Archive/historical smokes should stay archive-only unless an active card reopens the exact boundary.
- The active lane does not need a new “combined mega-smoke”; it needs a stable index that points to the right family.

## Suggested Follow-Up

If a future card needs a new smoke, add it to this index first and keep the card itself narrow.
