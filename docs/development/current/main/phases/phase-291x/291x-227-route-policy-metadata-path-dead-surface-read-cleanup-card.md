---
Status: Landed
Date: 2026-04-25
Scope: Remove the dead method-surface read from the MIR-call route-policy metadata path.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-226-generic-has-metadata-output-thinning-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_route_policy.inc
---

# 291x-227 Route Policy Metadata Path Dead Surface Read Cleanup Card

## Goal

Keep the MIR-call route-policy adapter metadata-first.

When `classify_mir_call_generic_method_route_kind_from_core_method_metadata(...)`
already returns a concrete route, `classify_generic_method_route(...)` still
classifies `mname` into `method_surface` and then discards the result.

That read is not a semantic guard; it is a dead compatibility surface read on
the metadata-success path.

## Boundary

- Do not change fallback behavior.
- Do not prune any tracked receiver/method classifier rows.
- Do not change `GenericMethodRouteState` fields.
- Only skip the unused method-surface classification after metadata already
  chose the route.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed.

- Removed the dead `classify_mir_call_method_surface(mname)` read from the
  metadata-success path in `classify_generic_method_route(...)`.
- Kept receiver/method surface classification inside the fallback branch only.
- No route-state fields or no-growth classifier rows changed; guard remains
  `classifiers=13 rows=13`.

Validated with:

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/archive/phase29ck_boundary/string/phase29ck_boundary_pure_string_length_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
