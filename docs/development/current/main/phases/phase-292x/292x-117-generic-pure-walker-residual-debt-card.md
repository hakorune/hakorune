---
Status: Active
Date: 2026-04-23
Scope: Inventory and shrink the remaining `.inc` analysis-debt guard rows after `pure_compile_minimal_paths` removal.
Related:
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md
  - docs/development/current/main/phases/phase-292x/292x-116-pure-compile-minimal-string-const-eval-card.md
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-117: Generic Pure Walker Residual Debt

## Current Debt

`tools/checks/inc_codegen_thin_shim_guard.sh` now reports 4 files / 7
analysis-debt lines:

- `hako_llvmc_ffi_compiler_state.inc`: 1 line
- `hako_llvmc_ffi_pure_compile.inc`: 1 line
- `hako_llvmc_ffi_pure_compile_generic_lowering.inc`: 3 lines
- `hako_llvmc_ffi_string_loop_seed_copy_graph.inc`: 2 lines

This is no longer a pile of route-specific raw recognizers. The remaining
question is whether these lines are generic walker substrate, copy graph
helpers, or still route-legality analysis hidden in boundary glue.

## Goal

Classify each remaining guard hit and remove only the lines that still encode
route-specific analysis. If a line is necessary walker substrate, either move it
behind a smaller helper API or explicitly document why it remains boundary
substrate.

## Plan

1. Print the guard hit list and classify each line.
2. Start with `hako_llvmc_ffi_string_loop_seed_copy_graph.inc`, because it still
   reads raw `op` values in a helper-specific file.
3. Keep `pure_compile_generic_lowering` changes behavior-preserving unless a
   fixture proves a route-specific owner is still hidden there.
4. Lower the allowlist only after the guard reports a reduction.

## Acceptance

```bash
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
