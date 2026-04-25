---
Status: Landed
Date: 2026-04-26
Scope: Prune the RuntimeDataBox push route fallback and retire the RuntimeDataAppendAny route vocabulary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-203-runtime-data-array-push-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-263-push-arraybox-route-fallback-prune-card.md
  - apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json
  - lang/src/runtime/collections/method_policy_box.hako
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
  - src/mir/generic_method_route_plan.rs
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-267 RuntimeData Push Route Fallback Prune Card

## Goal

Remove the remaining `RuntimeDataBox` push route fallback:

```c
if (bname && !strcmp(bname, "RuntimeDataBox")) {
  return HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_RUNTIME_DATA_APPEND_ANY;
}
```

`RuntimeDataBox.push(ArrayBox origin)` already has a CoreMethod carrier from
`291x-203`:

```text
generic_method.push
core_method.op = ArrayPush
route_kind = array_append_any
lowering_tier = cold_fallback
```

## Boundary

- Reuse existing `ArrayPush` metadata.
- Do not add a new CoreMethod op.
- Do not change public RuntimeData push semantics.
- Keep older pure-compile runtime declarations out of scope.
- Keep `has` family cleanup separate.

## Implementation

- Added missing `generic_method.push` / `ArrayPush` metadata to
  `runtime_data_array_push_min_v1.mir.json`.
- Removed the `RuntimeDataBox` branch from
  `classify_generic_method_push_route(...)`.
- Removed `HAKO_LLVMC_GENERIC_METHOD_PUSH_ROUTE_RUNTIME_DATA_APPEND_ANY` and its
  lowering case.
- Removed `RuntimeDataAppendAny` from `CollectionMethodPolicyBox`.
- Updated the stale route-plan unit test: copy-chain receiver origins now also
  produce the `RuntimeDataBox.push(ArrayBox origin)` metadata route.
- Removed the RuntimeData push row from the no-growth allowlist.

## Result

The no-growth guard baseline is now:

```text
classifiers=5 rows=5
```

Remaining rows are now `has` / MIR-surface only:

```text
classify_generic_method_emit_kind method has
classify_generic_method_has_route box ArrayBox
classify_generic_method_has_route box RuntimeDataBox
classify_mir_call_receiver_surface box MapBox
classify_mir_call_method_surface method has
```

## Verification

```bash
python3 -m json.tool apps/tests/mir_shape_guard/runtime_data_array_push_min_v1.mir.json >/dev/null
cargo test -q records_runtime_data_arraybox_push
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_push_min.sh
bash tools/smokes/v2/profiles/integration/apps/phase29x_runtime_data_dispatch_llvm_e2e_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh
bash tools/smokes/v2/profiles/archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh
cargo check -q
```

Observed:

```text
PASS phase29x_backend_owner_daily_runtime_data_array_push_min
PASS phase29x_runtime_data_dispatch_llvm_e2e_vm
PASS phase29cc_runtime_v0_adapter_fixtures_vm
PASS s3_link_run_llvmcapi_pure_array_get_ret_canary_vm
core-method-contract-inc-no-growth-guard ok classifiers=5 rows=5
```

## Next

Do not revisit push without a new owner-path change. The remaining cleanup work
is the `has` family plus the MIR-call `MapBox` receiver surface sentinel.
