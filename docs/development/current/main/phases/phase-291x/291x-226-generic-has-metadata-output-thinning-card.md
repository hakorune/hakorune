---
Status: Landed
Date: 2026-04-25
Scope: Thin the generic-method `has` metadata reader output struct.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-225-generic-has-dead-value-demand-helper-cleanup-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-226 Generic Has Metadata Output Thinning Card

## Goal

Remove unused fields from `GenericMethodHasRouteMetadata`.

The metadata reader validates `receiver_value` and `key_value` against the
current call site and still uses local variables for trace output. The caller
only consumes:

```text
metadata.invalid
metadata.route
```

The `matched`, `receiver_reg`, and `key_reg` fields are unused output surface
and make the adapter look wider than it is.

## Boundary

- Do not change metadata validation.
- Do not change route trace output.
- Do not change helper selection or emitted calls.
- Do not prune tracked string classifier rows.

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

- Removed the unused `matched`, `receiver_reg`, and `key_reg` fields from
  `GenericMethodHasRouteMetadata`.
- Kept receiver/key validation as locals inside the metadata reader.
- Kept route trace output unchanged.
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
