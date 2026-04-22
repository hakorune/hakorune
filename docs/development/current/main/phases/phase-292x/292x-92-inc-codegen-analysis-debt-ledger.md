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
- `.inc` lines: 19,264
- analysis-debt files: 28
- analysis-debt lines: 314

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
