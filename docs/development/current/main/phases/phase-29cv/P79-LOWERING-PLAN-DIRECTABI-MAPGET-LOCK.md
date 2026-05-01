---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: lock the next LoweringPlan v0 direct-ABI MapGet slice before implementation.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P78-LOWERING-PLAN-ARRAYGET-DIRECTABI-CONSUME.md
  - src/mir/generic_method_route_plan.rs
  - src/runner/mir_json_emit/tests/generic_method_routes.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
---

# P79 LoweringPlan DirectAbi MapGet Lock

## Goal

Lock the `MapGet` direct-ABI shape before adding the plan-only fixture.

P70 already proved `MapGet` as `ColdRuntime` through
`nyash.runtime_data.get_hh`. The next `MapGet` slice is not another cold runtime
path. It is the direct `MapBox.get` path that uses `nyash.map.slot_load_hh`.

## Decision

The P80 implementation slice may accept exactly this shape:

- `source_route_id = generic_method.get`
- `core_op = MapGet`
- `tier = DirectAbi`
- `emit_kind = direct_abi_call`
- `route_kind = map_load_any`
- `symbol = nyash.map.slot_load_hh`
- `route_proof = get_surface_policy`
- `receiver_origin_box = MapBox`
- `key_route = unknown_any`
- `arity = 1`
- `return_shape = null`
- `value_demand = read_ref`
- `publication_policy = null`
- `effects = ["read.key"]`

The fixture should call `MapBox.get` directly, not `RuntimeDataBox.get`, so this
slice does not blur cold runtime and direct ABI ownership.

## Non-goals

- no `RuntimeDataBox.get` behavior change
- no map set / stored-value proof promotion
- no `MapGet` scalar proof or perf keeper claim
- no route widening beyond the existing `map_load_any` row

## Acceptance For P80

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_get_directabi_min_v1.mir.json \
  --out /tmp/p80_lowering_plan_map_get_directabi.o
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
