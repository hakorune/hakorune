---
Status: Active
Date: 2026-04-22
Scope: `.inc` codegen analysis-debt ledger for Phase 292x.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md
  - tools/checks/inc_codegen_thin_shim_guard.sh
  - tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv
---

# 292x-92: `.inc` Codegen Analysis-Debt Ledger

## Baseline

Current no-growth baseline:

- `.inc` files: 76
- `.inc` lines: 19,521
- analysis-debt files: 27
- analysis-debt lines: 312

The baseline is not a permission slip to add more C analysis. It is a deletion
ledger. Reductions are expected as route families move to MIR-owned metadata.

## Debt Pattern

The guard counts lines matching:

- `analyze_*candidate`
- `hako_llvmc_match_*seed`
- `window_candidate`
- raw `yyjson_obj_get(... "blocks" | "instructions" | "op")` access

## Rule

- new debt files fail
- per-file debt growth fails
- reductions pass and should prune the allowlist
- deleted analyzers must reduce the baseline in the same commit

## First Reduction Target

`array_rmw_window`:

- debt file: `lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_window.inc`
- current C function: `analyze_array_rmw_window_candidate`
- target: route tag emitted by MIR, consumed by
  `hako_llvmc_ffi_generic_method_get_lowering.inc`

## Landed Reduction

`array_string_len_window`:

- deleted C function: `analyze_array_string_len_window_candidate`
- deleted trace fallback: `trace_array_string_len_window_candidate`
- lowered allowlist:
  - `hako_llvmc_ffi_generic_method_get_lowering.inc`: `4 -> 2`
  - `hako_llvmc_ffi_generic_method_get_window.inc`: `6 -> 3`

`array_rmw_window`:

- deleted C function: `analyze_array_rmw_window_candidate`
- deleted trace fallback: `trace_array_rmw_window_candidate`
- removed now-clean allowlist rows:
  - `hako_llvmc_ffi_generic_method_get_lowering.inc`
  - `hako_llvmc_ffi_generic_method_get_window.inc`

`string_direct_set_window_routes`:

- deleted hidden C matcher:
  - `ArrayStringPiecewiseDirectSetSourceReuseMatch`
  - `match_array_string_piecewise_concat3_direct_set_source_reuse`
- note: this matcher was not counted by the no-growth regex baseline, so the
  analysis-debt line count stays `314`; the `.inc` total shrank to `19,234`.

`generic_method.has`:

- moved the first generic method route-policy leaf to MIR metadata
  (`GenericMethodRoute`)
- note: this adds metadata reader / validation glue, not raw MIR analysis; the
  analysis-debt baseline stayed `314` for that card.

`array_string_store_micro` exact seed backend route:

- added function-level `exact_seed_backend_route` metadata for the already
  MIR-owned `array_string_store_micro_seed_route`
- renamed the backend consumer away from the legacy `hako_llvmc_match_*seed`
  debt pattern
- lowered allowlist:
  - removed now-clean `hako_llvmc_ffi_array_string_store_seed.inc`
  - `hako_llvmc_ffi_pure_compile.inc`: `34 -> 33`
  - current analysis-debt baseline is `312`
