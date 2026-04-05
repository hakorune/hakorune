# Phase 137x-90: main kilo reopen selection SSOT

## Goal

Reopen `main kilo` on the landed `nyash_kernel` split without re-expanding `.hako` migration or reintroducing `vm` cleanup work.

## Current Focus

- Rebaseline `kilo_kernel_small_hk`
- Recheck `kilo_micro_substring_concat`
- Recheck `kilo_micro_array_getset`
- Keep the fixed perf order:
  1. `leaf-proof micro`
  2. `micro kilo`
  3. `main kilo`

## Constraints

- Do not broaden `.hako` migration.
- Keep `vm-hako` parked as reference/conformance only.
- Keep the `keep / thin keep / compat glue / substrate candidate` split intact.

## Success Condition

- `main kilo` is reopened on top of the split `nyash_kernel`.
- Perf baselines are re-recorded on the new substrate boundary.
- The next hot leaf is fixed from the rebaseline, not from a new structural split.
