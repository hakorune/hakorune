# 293x-671 MIMAP-151A Segment Map Scalar Lookup Boundary Inventory

Status: selected current
Date: 2026-05-18

## Decision

Add a proof-only segment-map scalar lookup boundary inventory that uses explicit
segment/page/slice identities instead of raw pointer residence.

## Owner

```text
lang/src/hako_alloc/memory/
apps/hako-alloc-segment-map-scalar-lookup-boundary-inventory-proof/
```

## Scope

- Define one allocator-owned scalar inventory for segment/page membership lookup
  by explicit IDs.
- Prove accepted lookup for a known `(segment_id, page_id, slice)` row.
- Prove stable reject rows for unknown segment, wrong page, stale generation,
  out-of-range slice, and unsupported raw-pointer lookup request.
- Keep the route proof-only and executable in VM and pure-first EXE.

## Stop Lines

- No raw pointer residence or pointer-derived lookup.
- No real segment allocation/free.
- No arena backing allocation.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No worker scheduling, provider activation, host allocator replacement, hooks,
  or `#[global_allocator]`.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --only MIMAP-151A
bash tools/checks/k2_wide_hako_alloc_segment_map_scalar_lookup_boundary_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
