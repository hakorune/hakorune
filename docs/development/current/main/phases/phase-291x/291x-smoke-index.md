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

This file is the smoke selection SSOT for phase-291x cards. Cards should name
the family they touched and point here instead of repeating long script lists.

## Daily Gate

Run this bundle after ordinary docs/code slices:

```bash
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
tools/checks/dev_gate.sh quick
```

Use `cargo check -q` when Rust code changed. Avoid full perf ladders unless a
perf card explicitly requires them.

## Boundary Smokes

| Family | Active smoke(s) | Purpose |
| --- | --- | --- |
| MapBox direct has / MapHas metadata | `tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh` | Pins direct `MapBox.has` through `MapHas` route metadata; the filename is historical. |
| RuntimeData has facade metadata | `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_has_facade_min.sh` | Pins `runtime_data_contains_any` metadata and demand-driven `nyash.runtime_data.has_hh`. |
| ArrayBox direct has / ArrayHas metadata | `tools/smokes/v2/profiles/integration/phase29ck_boundary/array/phase29ck_boundary_pure_array_has_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh` | Pins direct `ArrayBox.has` and Array-origin `RuntimeDataBox.has` through `ArrayHas` metadata. |
| RuntimeData Array get | `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh`<br>`tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` | Pins Array get route-policy prune and RuntimeData dispatch boundaries. |
| RuntimeData Map get | `tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh`<br>`tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh` | Pins direct MapBox route pruning and RuntimeData fallback contract. |
| ArrayBox string `indexOf` observers | `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_select_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min.sh` | Pins string-observer route-state and same-slot live-follow-up behavior. |
| ArrayBox string len / substring observers | `tools/smokes/v2/profiles/integration/phase29ck_boundary/string/phase29ck_boundary_pure_array_string_len_live_after_get_min.sh`<br>`tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_insert_mid_source_only_min.sh`<br>`tools/smokes/v2/profiles/integration/phase137x/phase137x_boundary_array_string_len_piecewise_concat3_source_only_min.sh` | Pins len-window source tracking and substring follow-up metadata. |
| Set / metadata-first consumer packs | `tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_length_min.sh`<br>`tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_string_length_min.sh`<br>`tools/smokes/v2/profiles/quick/core/map/map_len_size_vm.sh`<br>`tools/smokes/v2/profiles/quick/core/array/array_length_vm.sh` | Confirms metadata-first consumer reordering for len/push and pinned fallback contracts. |

## Archive / Historical References

These are useful for targeted archaeology, but they are not first-choice daily
smokes:

- `tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh`
- `tools/smokes/v2/profiles/integration/compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh`
- `tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh`
- `tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh`

## Operating Rules

- Add a smoke here first when it becomes a durable phase-291x boundary.
- Do not duplicate long smoke lists in individual cards.
- Prefer one family row over a new combined mega-smoke.
- Archive/historical smokes stay archive-only unless an active card reopens the
  exact boundary.
