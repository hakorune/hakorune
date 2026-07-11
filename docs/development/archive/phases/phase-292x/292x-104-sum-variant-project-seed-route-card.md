---
Status: Landed
Date: 2026-04-22
Scope: Move Sum `variant_project` exact seeds from C-side MIR scanning to a
  function-level backend route tag.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-103-sum-variant-tag-seed-route-card.md
---

# 292x-104: Sum Variant Project Seed Route

## Problem

The remaining Sum exact seed matchers still scan raw MIR JSON instruction
windows in C:

- `hako_llvmc_match_variant_project_local_i64_seed`
- `hako_llvmc_match_variant_project_local_f64_seed`
- `hako_llvmc_match_variant_project_local_handle_seed`
- `hako_llvmc_match_variant_project_copy_local_i64_seed`
- `hako_llvmc_match_variant_project_copy_local_f64_seed`
- `hako_llvmc_match_variant_project_copy_local_handle_seed`

Sum placement metadata already proves the local aggregate route. The backend
boundary should validate a route payload and emit the selected helper.

## Decision

Add one MIR-owned exact seed route:

- owner: `FunctionMetadata.sum_variant_project_seed_route`
- backend tag: `metadata.exact_seed_backend_route.tag =
  "sum_variant_project_local"`
- selected source route: `sum_variant_project_seed_route`
- proof: `sum_variant_project_local_aggregate_seed`
- covered kinds:
  - `variant_project_local_i64`
  - `variant_project_local_f64`
  - `variant_project_local_handle`
  - `variant_project_copy_local_i64`
  - `variant_project_copy_local_f64`
  - `variant_project_copy_local_handle`

The route carries the literal payload needed by the temporary exact helper.
Route legality remains owned by `thin_entry_selections`,
`sum_placement_selections`, and `sum_placement_layouts`.

## Acceptance

```bash
cargo fmt --check
cargo test -q sum_variant_project_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_sum_metadata_keep_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Boundary traces for migrated fixtures must show:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata`
- `stage=sum_variant_project_local result=emit reason=exact_match`

## Result

- Added `FunctionMetadata.sum_variant_project_seed_route`.
- Added `ExactSeedBackendRouteKind::SumVariantProjectLocal` with backend tag
  `sum_variant_project_local`.
- Replaced the six raw C project matchers with
  `hako_llvmc_consume_sum_variant_project_seed_route`.
- Deleted the old project local/copy matcher include files and replaced them
  with `hako_llvmc_ffi_sum_local_seed_project_route.inc`.
- Updated project prebuilt MIR fixtures to carry
  `sum_variant_project_seed_route` and `exact_seed_backend_route`.
- `phase163x_boundary_sum_metadata_keep_min.sh` pins both exact-route hit and
  Sum project route emit traces for migrated fixtures.
- The analysis-debt baseline is now `21` files / `206` lines.
