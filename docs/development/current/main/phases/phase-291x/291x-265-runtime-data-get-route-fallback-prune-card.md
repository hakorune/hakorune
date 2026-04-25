---
Status: Landed
Date: 2026-04-26
Scope: Prune the RuntimeDataBox get route-policy fallback after repairing the dispatch E2E metadata boundary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-248-runtime-data-get-route-policy-review-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-265 RuntimeData Get Route Fallback Prune Card

## Goal

Remove the remaining `RuntimeDataBox` receiver-name fallback from
`classify_generic_method_get_route(...)`.

The `get` route should now be selected from MIR-owned
`generic_method.get` CoreMethod metadata:

```text
RuntimeDataBox.get(ArrayBox origin)
  -> core_method.op=ArrayGet
  -> route_kind=array_slot_load_any

RuntimeDataBox.get(MapBox origin)
  -> core_method.op=MapGet
  -> route_kind=runtime_data_load_any
```

## Boundary

- Do not change `MapGet` lowering semantics.
- Keep RuntimeData `MapGet` on `nyash.runtime_data.get_hh`.
- Do not add a new `.inc` string classifier.
- Keep unrelated RuntimeData facade sentinels pinned until their own cards.
- Treat the dispatch E2E metadata gap as a fixture boundary repair, not a new
  lowering feature.

## Implementation

- Remove the `RuntimeDataBox` branch from
  `classify_generic_method_get_route(...)`.
- Remove the corresponding no-growth allowlist row.
- Add the missing `generic_method.set` / `MapSet` route metadata to
  `phase29x_runtime_data_dispatch_e2e_min_v1.mir.json`.

The E2E fixture already had `generic_method.get` metadata for both ArrayBox and
MapBox origins. Once the get fallback was removed, the next uncovered boundary
was the same fixture's `RuntimeDataBox.set(MapBox)` instruction. Adding that
metadata lets the existing set metadata consumer select `map_store_any` without
reintroducing a get fallback.

## Result

The no-growth guard baseline shrank from:

```text
classifiers=8 rows=8
```

to:

```text
classifiers=7 rows=7
```

Remaining rows are now explicit facade/compat sentinels for `has`, `push`,
`set`, and MIR-call route surface cleanup. RuntimeData `get` is no longer one
of them.

## Verification

```bash
python3 -m json.tool apps/tests/phase29x_runtime_data_dispatch_e2e_min_v1.mir.json >/dev/null
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_get_min.sh
bash tools/build_hako_llvmc_ffi.sh
cargo check -q
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/generic_method_set_policy_mirror_guard.sh
git diff --check
```

Observed:

```text
PASS phase29x_runtime_data_dispatch_llvm_e2e_vm
PASS phase29ck_boundary_pure_runtime_data_map_get_min
PASS phase29ck_boundary_pure_runtime_data_array_get_min
PASS phase29x_backend_owner_daily_runtime_data_map_get_min
PASS phase29x_backend_owner_daily_runtime_data_array_get_min
core-method-contract-inc-no-growth-guard ok classifiers=7 rows=7
current-state-pointer-guard ok
generic-method-set-policy-mirror-guard ok routes=5 demands=3
```

## Next

Inventory the remaining seven no-growth rows before the next prune attempt:

```text
classify_generic_method_emit_kind method has
classify_generic_method_set_route box RuntimeDataBox
classify_generic_method_has_route box ArrayBox
classify_generic_method_has_route box RuntimeDataBox
classify_generic_method_push_route box RuntimeDataBox
classify_mir_call_receiver_surface box MapBox
classify_mir_call_method_surface method has
```
