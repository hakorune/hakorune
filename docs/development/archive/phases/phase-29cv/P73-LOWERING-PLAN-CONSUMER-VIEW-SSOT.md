---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: centralize LoweringPlan v0 consumer field reads without adding a shape.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# P73 LoweringPlan Consumer View SSOT

## Goal

Keep ny-llvmc moving toward an emit-only LoweringPlan consumer by removing the
first duplicated field-reading shape from `.inc` consumers.

P70 and P72 intentionally proved plan-first behavior with narrow local readers.
The next step is structural: create one common view over a `LoweringPlan` entry
and let route, need, get, and has consumers validate against that view.

## Decision

- Add a common `LoweringPlanGenericMethodView` reader in
  `hako_llvmc_ffi_lowering_plan_metadata.inc`.
- The common reader only validates the generic-method source family, tier
  spelling, contract proof, and required identity fields.
- Family-specific consumers still own family-specific legality:
  - `get` owns get route proof, return shape, and helper match.
  - `has` owns has route proof, key route, effect, and helper match.
  - route/need policy only maps a valid plan view to existing route/need enums.
- No new CoreOp or accepted route is added in this card.

## Non-goals

- no `MapLen` / `ArrayLen` plan-only fixture
- no removal of legacy `generic_method_routes`
- no route widening
- no helper-symbol inference from raw MIR

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_runtime_data_map_get_min_v1.mir.json \
  --out /tmp/p73_lowering_plan_runtime_data_map_get.o
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_has_directabi_min_v1.mir.json \
  --out /tmp/p73_lowering_plan_map_has_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29ck_boundary/map/phase29ck_boundary_pure_map_has_no_metadata_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
