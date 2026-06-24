# 296x-905 LOCAL-I64-MAP-STORAGE-REALIZATION-DESIGN-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-i64-map-storage-realization-design-v0
source_evidence=296x-904
row_kind=design
target_front=kilo_leaf_map_get_dynamic_covered_i64

selected_owner=exact_aot_local_i64_map_storage_realization
selected_plan_owner=src/mir/map_repr_plan.rs
selected_backend_owner=src/llvm_py/instructions/mir_call/collection_method_call.py
selected_runtime_boundary=crates/nyash_kernel/src/plugin/map_slot_load.rs

before_publication_representation=local_i64_key_map
publication_materialization_required=1
after_publication_representation=product_mapbox
product_mapbox_storage_changed=0
product_hasher_swap=0
sidecar_storage=0
mirbuilder_map_storage_ownership=0

first_allowed_slice=passive_plan_and_guard_surface
backend_lowering_enabled=0
runtime_helper_enabled=0
winner_claim=0
next_task=LOCAL-I64-MAP-STORAGE-REALIZATION-GUARD-SURFACE-001
summary=ok
```

## Decision

The remaining hot owner after `LocalFastPathFact` reachability is the product
MapBox hash lookup boundary.  The next implementation family must therefore
change the representation used before publication, not mutate product
`MapBox`.

The selected model is:

```text
exact-AOT / local-first:
  LocalI64KeyMap storage before publication

publication site:
  materialize Product MapBox semantics

product route:
  HashMap<MapKeyDomain, Box<dyn NyashBox>> remains the public truth
```

This is the same publication model used by object storage:

```text
published object is Box-compatible
unpublished local object may use a faster internal representation
```

## First Slice

The first implementation slice is not runtime lowering.  It should add a guard
surface for a passive plan:

```text
MapReprPlan / LocalMapStoragePlan:
  identifies local_i64_key_map candidates
  records publication/materialization requirement
  exports metadata for backend inspection
  does not lower differently yet
```

The actual lowering row can only open after that guard surface proves:

```text
local_i64_map_storage_candidate_count > 0
publication_materialization_sites_known=1
backend_lowering_enabled=0
product_mapbox_storage_changed=0
```

## Stop Lines

- no product `HashMap` hasher swap
- no product `MapBox` i64-only storage
- no sidecar storage inside product `MapBox`
- no MIRBuilder map storage ownership
- no helper-name / benchmark-name branch
- no direct runtime helper enablement in this design row
- no winner claim

## Validation

```bash
bash tools/checks/k2_wide_phase296x_local_i64_map_storage_realization_design_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
