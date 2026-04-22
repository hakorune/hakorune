---
Status: Active
Date: 2026-04-23
Scope: Inventory and cleanup order for `hako_llvmc_ffi_pure_compile_minimal_paths.inc`.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_minimal_paths.inc
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-111: Pure Compile Minimal Paths Inventory

## Decision

`hako_llvmc_ffi_pure_compile_minimal_paths.inc` is not thin glue today. It
contains six C-side raw MIR recognizers that read `blocks`, `instructions`, and
`op` to decide whether a function can be lowered by a handwritten shortcut.

This file is the next primary cleanup target after the exact-seed ladder work:

- current no-growth baseline: 5 `.inc` files / 47 analysis-debt lines
- this bucket: 40 analysis-debt lines
- rule: no new path may be added here
- cleanup mode: prove each path has a non-C route owner, then delete or replace
  it with MIR-owned metadata consumption

## Inventory

| Path | Current C Recognizer | Classification | Next Action |
| --- | --- | --- | --- |
| #1 | single-block `const* -> ret const` | route legality owner | delete-only probe; generic pure lowering and Hako LL daily owner already cover the shape |
| #2 | const compare branch with two const arms and merge ret | route legality owner plus fallback hook | delete-only probe after pure keep / historical ternary / llvmlite compare canaries pass |
| #3 | `MapBox` constructor, `set`, `size/len`, `ret` | CoreBox method shortcut | prove generic `mir_call` lowering owns the path, otherwise add MIR metadata route |
| #4 | `ArrayBox` constructor, `push`, `len/length/size`, `ret` | CoreBox method shortcut | prove generic `mir_call` lowering owns the path, otherwise add MIR metadata route |
| #5 | const ASCII string, `StringBox.length/size`, folded ret | string const-eval shortcut | choose delete vs MIR-owned const-eval route; C must not own the fold |
| #6 | const ASCII haystack/needle, `StringBox.indexOf`, folded ret | string const-eval shortcut | choose delete vs MIR-owned const-eval route; C must not own the fold |

## Cleanup Order

1. Delete-probe paths #1 and #2.
   - These are language/control-flow shapes, not backend helper special cases.
   - Acceptance must cover ret const, compare branch, pure keep, and historical
     ternary collection.
2. Retire paths #3 and #4.
   - First try deletion with generic constructor/method lowering canaries.
   - If generic lowering is missing a real contract, add MIR-owned route
     metadata. Do not add another `.inc` shape recognizer.
3. Decide paths #5 and #6 as a pair.
   - If the constant fold is required, add a MIR-owned const-eval route carrying
     the proof and result.
   - If the fold is not required, delete and rely on generic runtime method
     lowering.
4. Prune the allowlist after each successful deletion slice.

## Acceptance

Minimum for this inventory card:

```bash
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Minimum for any deletion slice:

```bash
cargo fmt --check
bash tools/build_hako_llvmc_ffi.sh
bash tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh
bash tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh
bash tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh
bash tools/checks/inc_codegen_thin_shim_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard must report a reduced debt count before the allowlist is pruned.
