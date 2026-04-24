---
Status: Landed
Date: 2026-04-25
Scope: Pin the mutating `set` CoreMethod route-carrier boundary before implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-176-push-emit-kind-mirror-prune-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_facts.rs
  - src/mir/core_method_op.rs
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-177 Set Mutating Carrier Preflight Card

## Goal

Prepare direct `ArrayBox.set(index, value)` and `MapBox.set(key, value)`
CoreMethod metadata carriers without changing backend lowering:

```text
MethodCall(ArrayBox, "set", [index, value])
  -> generic_method_routes[].core_method.op = ArraySet
  -> effect = mutates_slot
  -> route_kind = array_store_any

MethodCall(MapBox, "set", [key, value])
  -> generic_method_routes[].core_method.op = MapSet
  -> effect = mutates_slot
  -> route_kind = map_store_any
```

This is a BoxShape-only preflight. `set` is wider than `push` because the
legacy `.inc` layer has two coupled decisions:

- emit-kind classification: `method == set`
- storage-route classification: MapBox / ArrayBox / RuntimeDataBox route kind

The implementation must not collapse those decisions into a new backend-local
policy mirror.

## Boundary

- Do not remove the generic `set` emit-kind mirror row.
- Do not remove `classify_generic_method_set_route(...)` box rows.
- Do not remove mir-call route-policy `set` rows.
- Do not change helper symbols:
  - `nyash.map.slot_store_hhh`
  - `nyash.runtime_data.set_hhh`
  - `nyash.array.slot_store_hii`
  - `nyash.array.slot_store_hih`
  - `nyash.array.kernel_slot_store_hi`
- Do not add hot inline lowering.
- Do not encode the stored value as key metadata.
- Do not promote unknown-origin `RuntimeDataBox.set` to `ArraySet` / `MapSet`
  in the first carrier slice.
- Do not let `.inc` rediscover mutating legality from method or box names in a
  new path.

## Required Implementation Shape

The next code card should be the smallest carrier slice:

```text
match_generic_set_route(...)
  accepts:
    direct ArrayBox.set(index, value)
    direct MapBox.set(key, value)
  emits:
    route_id = generic_method.set
    core_method.op = ArraySet | MapSet
    proof = SetSurfacePolicy
    lowering_tier = cold_fallback
    route_kind = array_store_any | map_store_any
    arity = 2
    key_route = i64_const | unknown_any
    key_value = first argument
    return_shape = scalar_i64
    value_demand = write_any
    publication_policy = no_publication
```

The stored value is the second argument. If a future lowering needs operand
metadata for it, add an explicit value/argument field. Do not overload
`key_value`.

`RuntimeDataBox.set(key, value)` is intentionally not part of the first carrier
unless the implementation can prove receiver origin and publication semantics
from MIR-owned metadata. Unknown facade calls must remain metadata-absent
fallback routes.

## Deletion Conditions

The legacy `set` mirror rows may not be pruned by `ArraySet` / `MapSet`
metadata alone. Deletion requires separate metadata-absent mutating boundary
coverage for:

- direct ArrayBox set carrier presence
- direct MapBox set carrier presence
- unknown RuntimeDataBox set fallback
- existing array string-store publication/source-preserve behavior
- existing array i64 vs handle store route behavior
- existing map store receipt behavior
- alias/residence invalidation remains equivalent

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
