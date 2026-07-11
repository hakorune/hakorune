# Phase 137x-90: main kilo reopen selection SSOT

## Goal

Reopen `main kilo` on the landed `nyash_kernel` split after `phase-138x` fixes the final semantic-owner graph.

## Current Focus

- Keep this lane as the perf successor
- Wait for `phase-138x` to finish semantic owner cutover
- Then rebaseline `kilo_kernel_small_hk`
- Then recheck `kilo_micro_substring_concat`
- Then recheck `kilo_micro_array_getset`
- Keep the fixed perf order:
  1. `leaf-proof micro`
  2. `micro kilo`
  3. `main kilo`

## Constraints

- Do not broaden `.hako` migration from this lane.
- Do not pre-empt `phase-138x`.
- Keep `vm-hako` parked as reference/conformance only.
- Keep the `keep / thin keep / compat glue / substrate candidate` split intact.

## Success Condition

- `main kilo` is reopened on top of the split `nyash_kernel`.
- Perf baselines are re-recorded on the new substrate boundary.
- The next hot leaf is fixed from the rebaseline, not from a new structural split.
