---
Status: Landed
Date: 2026-04-25
Scope: Remove the unused `matched` output field from MapLookup fusion metadata.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - lang/c-abi/shims/hako_llvmc_ffi_map_lookup_fusion_metadata.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
---

# 291x-234 MapLookup Fusion Metadata Matched Field Cleanup Card

## Goal

Remove the unused `matched` field from `MapLookupFusionRouteMetadata`.

The match function already communicates success with its integer return value.
Callers consume the route payload fields for trace and const-fold emission, but
do not read `fusion.matched`.

## Boundary

- Do not change MapLookup fusion validation.
- Do not change get/has trace output.
- Do not change const-fold lowering.
- Do not prune tracked string classifier rows.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_get_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed in 2026-04-25 cleanup slice.

- Removed the unused `matched` field from `MapLookupFusionRouteMetadata`.
- Kept receiver/key/result payload fields for caller trace output.
- Kept stored-value const-fold payload fields unchanged.
- No no-growth classifier rows changed; guard remains `classifiers=20 rows=20`.

Note: `phase291x_maplookup_fusion_const_fold_contract_vm.sh` currently stops at
`unsupported pure shape for current backend recipe` before this field contract
is exercised, so this card uses the get/has daily caller smokes as the active
acceptance gate.
