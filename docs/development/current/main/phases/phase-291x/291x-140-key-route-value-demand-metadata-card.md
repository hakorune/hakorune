---
Status: Landed
Date: 2026-04-24
Scope: Expose key-route and value-demand metadata for generic-method has routes without changing lowering behavior.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/hotline-core-method-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-139-receiver-origin-proof-metadata-card.md
  - src/mir/generic_method_route_facts.rs
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/root.rs
---

# 291x-140 Key-Route / Value-Demand Metadata Card

## Goal

Make the next MapHas promotion decision evidence-backed by carrying key and
value-demand facts in `generic_method_routes`.

## Implementation

- Added `src/mir/generic_method_route_facts.rs` as the reusable fact owner.
- Moved receiver-origin lookup into that fact module.
- Added `GenericMethodKeyRoute`:

```text
i64_const
unknown_any
```

- Added `GenericMethodValueDemand`:

```text
read_ref
```

- `GenericMethodRoute` now carries `key_route` and `value_demand`.
- MIR JSON emits both fields.
- The `.inc` consumer validates known `key_route` vocabulary when present, but
  accepts missing `key_route` for older MIR JSON compatibility.

## Observed Metadata

For `bench_kilo_leaf_map_getset_has.hako`, the measured route now exposes:

```text
box_name = RuntimeDataBox
receiver_origin_box = MapBox
key_route = i64_const
core_method = null
route_kind = runtime_data_contains_any
helper_symbol = nyash.runtime_data.has_hh
value_demand = read_ref
```

## Boundary

- No helper route is promoted in this card.
- `core_method` remains `null` for the measured RuntimeData facade route.
- `route_kind` remains `runtime_data_contains_any`.
- `helper_symbol` remains `nyash.runtime_data.has_hh`.

## Next

- Do not promote this route to `MapHas` until the key-conversion seam is proven
  no-regress.
- Decide whether the next keeper is a key-specialized route or a separate
  value-demand proof for `MapGet`.
- Re-run HCM-7 perf/asm before any CoreMethod promotion.
