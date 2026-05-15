# 293x-369 METADATA-CATALOG-003 Promotion Matrix

Status: landed
Date: 2026-05-15

## Decision

`METADATA-CATALOG-003` records the current promotion matrix for MIR metadata
rows that should already be treated as CorePlan inputs, verifier contracts,
backend lowering routes, or metadata-only rows.

This is a BoxShape docs row only. It does not change MIR JSON shape, Rust
metadata structs, verifier behavior, backend lowering, or runtime behavior.

## Responsibility

The canonical matrix lives in:

```text
docs/reference/mir/metadata-facts-ssot.md
```

The matrix separates:

- rows that are active now as contracts, CorePlan inputs, or lowering routes;
- rows that should become contracts/routes only when a specific owner lands;
- rows that must stay metadata-only or experimental seed bridges.

## Current Active Promotion Set

Treat these as active contract / CorePlan / route surfaces when planning
follow-up work:

- `lowering_plan`
- `typed_object_plans`
- `static_data_plans`
- `effect_plans`
- `inline_plans` only for `request=required` and `verified=true`
- `string_kernel_plans`
- `placement_effect_routes`
- `exact_numeric_runtime_check_contracts`
- `hako_alloc_*_packed_store_pilot_plans` as verifier-active only, not
  CorePlan lowering

## Near-Term Promotion Queue

This queue is closed. The follow-up cards landed in this order:

1. `293x-370 METADATA-PROMOTE-001`: active promotion matrix guard.
2. `293x-371 METADATA-PROMOTE-002`: typed-object/static-data verifier
   hardening.
3. `293x-372 METADATA-PROMOTE-003`: exact numeric / effect /
   required-inline / string-kernel contract rows.
4. `293x-373 METADATA-PROMOTE-004`: `placement_effect_routes` consumer
   fold-up plan.
5. `293x-374 METADATA-PROMOTE-005`: PackedArray no-fallback contract before
   backend lowering activation.
6. `293x-375 METADATA-PROMOTE-006`: seed route retirement ledger.

Future metadata cleanup should use `metadata-facts-ssot.md` and a new owner
row. Do not reopen this original queue to add allocator behavior or backend
activation work.

## Stop Lines

- Do not combine metadata promotion cleanup with allocator behavior rows.
- Do not promote `*_seed_route`, `*_micro_seed_route`, or
  `exact_seed_backend_route` to CorePlan ownership.
- Do not flip packed-row `backend_lowering_enabled` without a proof-bearing
  direct-read route, backend capability gate, and `boxed_fallback=false`
  contract in the same behavior card.
- Do not make Stage0 own layout, legality, packed eligibility,
  materialization, optimizer routes, or backend routes.

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/mir_metadata_catalog_guard.sh
tools/checks/dev_gate.sh quick
```
