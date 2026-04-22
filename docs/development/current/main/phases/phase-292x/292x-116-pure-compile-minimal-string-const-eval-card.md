---
Status: Landed
Date: 2026-04-23
Scope: Decide and retire `pure_compile_minimal_paths` paths #5 and #6.
Related:
  - docs/development/current/main/phases/phase-292x/292x-111-pure-compile-minimal-paths-inventory-card.md
  - docs/development/current/main/phases/phase-292x/292x-115-pure-compile-minimal-map-array-deletion-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_minimal_paths.inc
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-116: Pure Compile Minimal String Const-Eval Decision

## Goal

Retire the remaining two raw MIR recognizers in
`hako_llvmc_ffi_pure_compile_minimal_paths.inc`:

- path #5: const ASCII string handle, `StringBox.length/size`, folded ret
- path #6: const ASCII haystack/needle, `StringBox.indexOf`, folded ret

Both paths are compile-time constant evaluation shortcuts. The `.inc` boundary
must not own string method legality or rediscover constant-foldable shapes.

## Decision Options

1. Delete both paths and rely on generic StringBox runtime lowering.
2. If pure mode still needs the fold, add MIR-owned const-eval metadata carrying
   the proof and folded result, then make `.inc` consume only that metadata.

Do not add another C-side raw MIR recognizer.

## Current Result

- Deleted `hako_llvmc_ffi_pure_compile_minimal_paths.inc` and removed its
  include from `hako_llvmc_ffi_pure_compile.inc`.
- Deleted paths #5 and #6 instead of adding MIR-owned const-eval metadata.
- Kept the generic pure lowering owner by materializing skipped StringBox
  constants at the `newbox StringBox` boundary when a later method needs a
  handle.
- Updated the boundary smokes from legacy seed wording to generic boundary
  wording and moved the hand-authored fixtures from `callee.name` to
  `callee.method`.
- Guard moved from 5 files / 21 analysis-debt lines to 4 files / 7
  analysis-debt lines.

## Plan

1. Inventory the smokes that still exercise path #5 / #6.
2. Delete-probe both paths together.
3. If a smoke fails because it depends on legacy dialect, update the smoke to
   canonical MIR first.
4. If a smoke fails because no runtime/generic owner exists, add the smallest
   MIR-owned route or const-eval contract before deleting.
5. Prune the allowlist after the guard reports the reduced count.

## Follow-Up

The remaining guard debt is no longer `pure_compile_minimal_paths`; it is the
generic pure walker / copy-graph residual bucket. Continue with
`292x-117-generic-pure-walker-residual-debt-card.md`.

## Acceptance

```bash
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
bash tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
