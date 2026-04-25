---
Status: Landed
Date: 2026-04-25
Scope: Remove the dead `runtime_array_has` route-state flag.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-229-dead-array-has-need-flag-cleanup-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - lang/src/runtime/meta/mir_call_route_policy_box.hako
  - lang/src/runtime/collections/method_policy_box.hako
---

# 291x-230 Dead Runtime Array Has Route Flag Cleanup Card

## Goal

Remove the route-state flag that used to mark Array-origin `has`:

```text
runtime_array_has
```

After `291x-224` and `291x-229`, Array-origin `has` no longer has a separate
array route or need flag. It stays on the RuntimeData facade selected by
`box_name == ArrayBox` / `box_name == RuntimeDataBox`.

## Boundary

- Do not add `ArrayHas`.
- Do not change `nyash.runtime_data.has_hh` behavior.
- Do not prune tracked string classifier rows.
- Keep Map-origin `has` route-state behavior unchanged.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed `HAKO_LLVMC_MIR_CALL_ROUTE_RUNTIME_ARRAY_HAS`.
- Removed `runtime_array_has` from `GenericMethodRouteState` and
  `GenericMethodEmitPlan`.
- Removed route trace output for the dead array-has route-state flag.
- Removed mirrored `.hako` route-policy and collection method-policy
  vocabulary.
- Array-origin `has` still routes through `nyash.runtime_data.has_hh`; Map-origin
  `has` behavior is unchanged.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
