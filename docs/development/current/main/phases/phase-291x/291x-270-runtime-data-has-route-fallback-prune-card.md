---
Status: Landed
Date: 2026-04-26
Scope: Prune the RuntimeDataBox.has route fallback after metadata coverage.
Related:
  - docs/development/current/main/phases/phase-291x/291x-268-arrayhas-core-method-carrier-card.md
  - docs/development/current/main/phases/phase-291x/291x-269-arraybox-has-route-fallback-prune-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_has_policy.inc
  - tools/checks/core_method_contract_inc_no_growth_allowlist.tsv
---

# 291x-270 RuntimeData Has Route Fallback Prune Card

## Goal

Remove the direct `RuntimeDataBox` branch from
`classify_generic_method_has_route(...)`.

`RuntimeDataBox.has` is now represented by MIR-owned `generic_method.has`
metadata:

- Map-origin with proven i64 key: `core_method.op=MapHas`,
  `route_kind=map_contains_i64`
- Array-origin: `core_method.op=ArrayHas`, `route_kind=array_contains_any`
- Unknown/compat RuntimeData facade: `route_kind=runtime_data_contains_any`

The backend should consume that metadata instead of rediscovering the receiver
by `box_name`.

## Boundary

- Prune only `classify_generic_method_has_route box RuntimeDataBox`.
- Keep `plan->runtime_map_has` while MIR-call MapBox/has surface sentinels are
  still under separate review.
- Keep generic emit-kind `method has`.
- Keep MIR-call `MapBox + has` surface sentinels.
- Do not remove `runtime_data_contains_any` route metadata support; it remains
  the explicit compat facade route for metadata-present RuntimeData cases.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo test -q runtime_data_has
cargo check -q
git diff --check
```

## Result

`classify_generic_method_has_route box RuntimeDataBox` is removed from the
backend route classifier and the no-growth allowlist. Metadata-present
RuntimeData cases continue to compile through `generic_method.has` route
metadata, including the explicit `runtime_data_contains_any` facade route.

Remaining no-growth rows after this card:

- `classify_generic_method_emit_kind method has`
- `classify_mir_call_receiver_surface box MapBox`
- `classify_mir_call_method_surface method has`

## Validated

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/runtime_data/phase29ck_boundary_pure_runtime_data_map_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29ck_boundary/array/phase29ck_boundary_pure_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_array_has_min.sh
bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_has_min.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/checks/core_method_contract_manifest_guard.sh
cargo test -q runtime_data_has
cargo check -q
git diff --check
```

Guard result:

```text
[core-method-contract-inc-no-growth-guard] ok classifiers=3 rows=3
```

## Next

The method-specific `has` route classifier is now metadata-only. Remaining
cleanup belongs to generic emit-kind selection and MIR-call route surface
fallbacks; keep those as separate evidence cards.
