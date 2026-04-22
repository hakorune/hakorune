---
Status: Landed
Date: 2026-04-22
Scope: Move Sum `variant_tag` exact seeds from C-side MIR scanning to a
  function-level backend route tag.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md
---

# 292x-103: Sum Variant Tag Seed Route

## Problem

`hako_llvmc_ffi_sum_local_seed_matchers_tag.inc` still owns raw MIR shape
analysis for five exact seed helpers even though MIR already carries Sum
placement metadata:

- `hako_llvmc_match_variant_tag_local_i64_seed`
- `hako_llvmc_match_variant_tag_local_tag_only_seed`
- `hako_llvmc_match_variant_tag_local_f64_seed`
- `hako_llvmc_match_variant_tag_local_handle_seed`
- `hako_llvmc_match_variant_tag_copy_local_i64_seed`

That keeps `.inc` as a route-legality owner. It should only validate a
pre-decided MIR route and emit the selected helper.

## Decision

Add one MIR-owned exact seed route:

- owner: `FunctionMetadata.sum_variant_tag_seed_route`
- backend tag: `metadata.exact_seed_backend_route.tag =
  "sum_variant_tag_local"`
- selected source route: `sum_variant_tag_seed_route`
- proof: `sum_variant_tag_local_aggregate_seed`
- covered kinds:
  - `variant_tag_local_i64`
  - `variant_tag_local_tag_only`
  - `variant_tag_local_f64`
  - `variant_tag_local_handle`
  - `variant_tag_copy_local_i64`

The route may use existing `thin_entry_selections`,
`sum_placement_facts`, `sum_placement_selections`, and
`sum_placement_layouts` as its proof substrate. It must not add new accepted
language surface.

## Non-Goals

- Do not migrate `variant_project_*` matchers in this slice.
- Do not add benchmark-name or helper-name ownership to C.
- Do not make `.inc` rediscover Sum legality by scanning instruction windows.

## Acceptance

Pin the slice with:

```bash
cargo fmt --check
cargo test -q sum_variant_tag_seed --lib
cargo test -q exact_seed_backend_route --lib
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/phase163x/phase163x_boundary_sum_metadata_keep_min.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Boundary traces for migrated fixtures must show:

- `stage=exact_seed_backend_route result=hit reason=mir_route_metadata`
- `stage=sum_variant_tag_local result=emit reason=exact_match`

The `.inc` debt guard should drop by the five removed matcher definitions.

## Result

- Added `FunctionMetadata.sum_variant_tag_seed_route`.
- Added `ExactSeedBackendRouteKind::SumVariantTagLocal` with backend tag
  `sum_variant_tag_local`.
- Replaced the five raw C matchers in
  `hako_llvmc_ffi_sum_local_seed_matchers_tag.inc` with
  `hako_llvmc_consume_sum_variant_tag_seed_route`.
- Updated the tag-only, i64, f64, handle, and copy-i64 prebuilt MIR fixtures to
  carry `sum_variant_tag_seed_route` and `exact_seed_backend_route`.
- `phase163x_boundary_sum_metadata_keep_min.sh` now pins both exact-route hit
  and Sum route emit traces for the migrated fixtures.
- The analysis-debt baseline is now `23` files / `257` lines.
