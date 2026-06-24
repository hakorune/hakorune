# 296x-1544: BOX-COMPILATION-CONTEXT-FACTS-PILOT-001

Status: landed

## Goal

Select the next bounded MirBuilder easy-tier family pilot and start the first
slice of real facts inventory.

## Slice

- Source: `crates/hakorune_mir_builder/src/context.rs`
- Family: `BoxCompilationContext`
- Bounded methods:
  - `new`
  - `is_empty`
- Excluded from the pilot:
  - `size_info`

## Notes

- This slice is intentionally smaller than the full `context` module.
- The first landing point is lightweight facts extraction, plan, and oracle
  fixtures for the bounded constructor + is_empty slice.
- No size-info claim is made here.
- The later typed IR / generated artifact work stays queued after this pilot
  inventory is green.

## Acceptance

- `BoxCompilationContext::new` facts are extracted as `Self::default()`
- `BoxCompilationContext::is_empty` facts are extracted as a three-field
  conjunction over the internal ordered maps
- The corresponding HakoLifecyclePlan and oracle vectors are pinned for the
  same bounded slice
- `BoxCompilationContext::size_info` remains excluded from the pilot
