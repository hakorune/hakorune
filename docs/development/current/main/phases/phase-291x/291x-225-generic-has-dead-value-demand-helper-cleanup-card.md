---
Status: Landed
Date: 2026-04-25
Scope: Remove the unused generic-method `has` value-demand helper.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-224-generic-has-dead-array-route-cleanup-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-225 Generic Has Dead Value-Demand Helper Cleanup Card

## Goal

Remove the unused `classify_generic_method_has_value_demand_read_ref(...)`
helper.

The `has` metadata reader already validates:

```text
value_demand == read_ref
```

The helper later recomputes a broad route-is-not-none boolean, stores it in a
local, and immediately discards it. That creates a fake second decision point
without affecting lowering.

## Boundary

- Do not change `value_demand` metadata validation.
- Do not change helper selection or emitted calls.
- Do not prune any tracked string classifier rows.

## Acceptance

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

## Result

Landed.

- Removed the unused `classify_generic_method_has_value_demand_read_ref(...)`
  helper.
- Removed the discarded `value_demand_read_ref` local from `has` lowering.
- Kept metadata validation as the only value-demand contract check:
  `value_demand == read_ref`.
- No no-growth classifier rows changed; guard remains `classifiers=13 rows=13`.

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
