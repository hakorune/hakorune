---
Status: Landed
Date: 2026-04-25
Scope: Remove the dead generic-method `has` ArrayContainsAny route vocabulary.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-223-generic-has-route-no-growth-guard-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - lang/src/runtime/collections/method_policy_box.hako
---

# 291x-224 Generic Has Dead Array Route Cleanup Card

## Goal

Remove the unused `ArrayContainsAny` route vocabulary from the generic
`has` policy.

Current behavior already keeps array-origin `has` on the RuntimeData facade:

```text
ArrayBox / RuntimeDataBox(ArrayBox-origin) has
  -> runtime_data_contains_any
  -> nyash.runtime_data.has_hh
```

The C-side `HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY` enum
variant is never selected by `classify_generic_method_has_route(...)`; if it
were selected, the emitter would reject it because no helper symbol is defined.

## Boundary

- Do not add `ArrayHas`.
- Do not change array-origin `RuntimeDataBox.has` lowering.
- Do not prune any tracked box/method string classifier rows.
- Keep array-origin `has` on the runtime-data facade until a separate
  `ArrayHas` BoxCount card exists.

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

- Removed `HAKO_LLVMC_GENERIC_METHOD_HAS_ROUTE_ARRAY_CONTAINS_ANY` from the
  C-side generic `has` route enum and emitter switch.
- Removed the dead `.hako` `ArrayContainsAny` policy vocabulary.
- Kept `runtime_array_has` mapping to `RuntimeDataContainsAny`, preserving the
  existing `nyash.runtime_data.has_hh` facade route.
- No no-growth classifier rows changed; guard remains `classifiers=13 rows=13`.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
