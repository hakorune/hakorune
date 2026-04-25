---
Status: Landed
Date: 2026-04-25
Scope: Remove dead demand helper functions from generic get/len/push method-specific policies.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-231-method-specific-route-no-growth-guard-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_len_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_push_policy.inc
---

# 291x-232 Method-Specific Dead Demand Helper Cleanup Card

## Goal

Remove method-specific demand helpers whose results are computed and then
discarded.

Affected helpers:

```text
classify_generic_method_get_value_demand_stable_object(...)
classify_generic_method_get_publish_demand_need_stable_object(...)
classify_generic_method_len_value_demand_read_ref(...)
classify_generic_method_push_value_demand_encode_any(...)
classify_generic_method_push_storage_demand_generic_residence(...)
classify_generic_method_push_mutation_demand_invalidate_aliases(...)
```

These helpers do not guard lowering. The real route contracts are already
encoded by route selection and, for metadata paths, by metadata validation.

## Boundary

- Do not change route selection.
- Do not change emitted helper calls.
- Do not prune tracked string classifier rows.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 cleanup slice.

- Removed dead get/len/push demand helper functions.
- Removed local demand variables whose values were only cast to `(void)`.
- Kept route selection, helper symbols, and no-growth tracked mirror rows unchanged.
- `core_method_contract_inc_no_growth_guard.sh` remains at `classifiers=20 rows=20`.
