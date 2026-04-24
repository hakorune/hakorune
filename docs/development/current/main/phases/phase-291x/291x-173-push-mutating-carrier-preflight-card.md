---
Status: Landed
Date: 2026-04-25
Scope: Pin the mutating `push` CoreMethod route-carrier boundary before implementation.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-172-metadata-absent-substring-fallback-contract-card.md
  - src/mir/generic_method_route_plan.rs
  - src/mir/generic_method_route_facts.rs
  - src/mir/core_method_op.rs
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-173 Push Mutating Carrier Preflight Card

## Goal

Prepare the `ArrayBox.push(value)` CoreMethod metadata carrier without changing
backend lowering:

```text
MethodCall(ArrayBox, "push", [value])
  -> generic_method_routes[].core_method.op = ArrayPush
  -> effect = mutates_shape
  -> arity = 1
  -> key metadata = null
  -> .inc still uses the legacy push fallback until the consumer card lands
```

This is a BoxShape-only preflight. `push` is the first mutating carrier in this
sequence, so it must not inherit read-only assumptions from `get` / `has` /
`len` / `substring`.

## Boundary

- Do not remove the generic `push` emit-kind mirror row.
- Do not remove the mir-call route-policy `push` mirror row.
- Do not change helper symbols:
  - `nyash.array.slot_append_hh`
  - `nyash.runtime_data.push_hh`
- Do not add hot inline lowering.
- Do not encode the pushed value as key metadata.
- Do not promote unknown-origin `RuntimeDataBox.push` to `ArrayPush`.
- Do not let `.inc` rediscover mutating legality from method or box names in a
  new path.

## Inventory

Current SSOT pieces already exist:

- `CoreMethodContract` has `ArrayBox.push/1` with:
  - `core_op = ArrayPush`
  - `effect = mutates_shape`
  - `lowering_tier = cold_fallback`
  - `cold_lowering = nyash.array.slot_append_hh`
- MIR carrier vocabulary already knows `CoreMethodOp::ArrayPush`.
- Legacy `.inc` push lowering still routes by compatibility policy:
  - direct `ArrayBox` and runtime-array plan flags use
    `nyash.array.slot_append_hh`
  - unknown `RuntimeDataBox` uses `nyash.runtime_data.push_hh`

## Required Implementation Shape

The next code card should be the smallest carrier slice:

```text
match_generic_push_route(...)
  accepts:
    direct ArrayBox.push(value)
  emits:
    route_id = generic_method.push
    core_method.op = ArrayPush
    proof = PushSurfacePolicy
    lowering_tier = cold_fallback
    route_kind = ArrayAppendAny
    arity = 1
    key_route = null
    key_value = null
    return_shape = scalar_i64
    value_demand = write_any
    publication_policy = no_publication
```

`RuntimeDataBox.push(value)` is intentionally not part of the first carrier
unless the implementation can prove `receiver_origin_box=ArrayBox` from
MIR-owned metadata. Unknown facade calls must remain metadata-absent fallback
routes.

If the carrier needs to publish the pushed operand, add a value/argument
metadata slot explicitly. Do not reuse `key_value` for a non-key argument.

## Deletion Condition

The legacy `push` mirror row may not be pruned by `ArrayPush` metadata alone.
Deletion requires a separate metadata-absent mutating boundary contract that
covers:

- direct ArrayBox push carrier presence
- unknown RuntimeDataBox push fallback
- existing runtime-array plan flag behavior
- return value remains the append receipt / resulting length shape expected by
  current lowering
- alias/residence invalidation remains equivalent

## Acceptance

```bash
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
