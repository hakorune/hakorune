---
Status: Active
Date: 2026-04-25
Scope: Prune the `mir_call_route_policy` RuntimeDataBox receiver-surface mirror after exact metadata-absent review.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-255-post-birth-cleanup-task-order-card.md
  - docs/development/current/main/phases/phase-291x/291x-257-mir-call-mapbox-receiver-surface-review-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-258 MIR-Call RuntimeDataBox Receiver-Surface Prune Card

## Goal

Decide whether the remaining `RuntimeDataBox` receiver-surface classifier in
`mir_call_route_policy` is still needed after the direct ArrayBox receiver row
was pruned and the MapBox receiver row was reviewed.

## Change

Pruned the `mir_call` receiver-surface mirror for `RuntimeDataBox`:

- removed `HAKO_LLVMC_MIR_CALL_RECV_SURFACE_RUNTIME_DATA_BOX`
- removed `classify_mir_call_receiver_surface(... "RuntimeDataBox" ...)`
- removed the `RuntimeDataBox + has + MapBox origin -> RUNTIME_MAP_HAS` route arm
- removed the matching `RuntimeDataBox + has + MapBox origin -> NEED_MAP_HAS` arm
- removed the no-growth allowlist row

## Probe

The initial exact metadata-absent boundary confirmed why this row was suspicious:

```text
RuntimeDataBox.has(MapBox origin)
generic_method_routes = []
NYASH_NY_LLVM_COMPILER=/__missing__/ny-llvmc
ny-llvmc --in /tmp/runtime_data_map_has_no_metadata.*.json --out /tmp/*.o
```

Before pruning, route trace showed:

```text
bname=RuntimeDataBox mname=has ... recv_org=2 ... map_has:1
object written
```

After pruning, the same metadata-absent boundary still compiled:

```text
bname=RuntimeDataBox mname=has ... recv_org=2 ... map_has:0
object written
```

Existing metadata-carrying boundaries also stayed green for both map and array
origins.

## Decision

Safe prune.

`RuntimeDataBox` does not need to be a `mir_call_route_policy` receiver-surface
classifier anymore. The metadata-absent semantic fallback is still owned by the
method-specific `generic_method_has_policy` RuntimeDataBox compatibility row:

```text
RuntimeDataBox.has(...) -> nyash.runtime_data.has_hh
```

This card only removes the redundant `mir_call` map-probe promotion. It does
not delete the generic `RuntimeDataBox` has-route fallback.

## Boundary

- Keep `MapBox` receiver-surface classification; 291x-257 proved it is still
  required for direct metadata-absent `MapBox.has`.
- Keep `classify_generic_method_has_route(... "RuntimeDataBox" ...)` until the
  has-family cleanup proves a narrower replacement.
- Do not add hot lowering or helper substitution in this cleanup card.
- Do not infer RuntimeDataBox set/get/push cleanup from this result.

## Next Work

Proceed to the next task-order family:

```text
has family cleanup
```

The next slice should review generic `mname == "has"`, direct ArrayBox
has-route fallback, RuntimeDataBox has-route fallback, and the remaining
`mir_call` method-surface `has` row as one family, with one decision per row.

## Acceptance

```bash
cargo check -q
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
