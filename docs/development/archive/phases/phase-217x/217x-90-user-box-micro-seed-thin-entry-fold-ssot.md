# 217x-90 User-Box Micro Seed Thin-Entry Fold SSOT

Status: SSOT

## Goal

- move the current boundary pure-first user-box micro seed thin-entry read to folded `placement_effect_routes` first
- keep this cut narrow and behavior-preserving

## In Scope

- update the shared thin-entry metadata helper in `hako_llvmc_ffi_common.inc`
- read folded `thin_entry` rows from `placement_effect_routes`
- preserve `thin_entry_selections` as compatibility fallback
- add folded route rows to the user-box metadata-bearing prebuilt MIR fixtures that already own the boundary proof

## Fixed Decisions

- no new folded route schema field is needed in this cut
- the helper derives the thin-entry surface from `detail = manifest_row`
- field inline-scalar and method known-receiver proving stay in the same cut because they share the same helper
- this is still a consumer fold, not a generic transform

## Out of Scope

- deleting `thin_entry_selections`
- broad user-box lowering rewrites
- string or sum seam changes
- widening `placement_effect_routes` beyond current thin-entry transport

## Acceptance

- boundary user-box field and known-receiver smokes stay green when fixtures carry folded `placement_effect_routes`
- `thin_entry_selections` remains available as fallback
- `git diff --check`

