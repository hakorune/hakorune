---
Status: Active
Date: 2026-04-23
Scope: Delete-probe `pure_compile_minimal_paths` path #3 and #4.
Related:
  - docs/development/current/main/phases/phase-292x/292x-111-pure-compile-minimal-paths-inventory-card.md
  - docs/development/current/main/phases/phase-292x/292x-112-pure-compile-minimal-ret-branch-deletion-card.md
  - lang/c-abi/shims/hako_llvmc_ffi_pure_compile_minimal_paths.inc
  - tools/checks/inc_codegen_thin_shim_guard.sh
---

# 292x-115: Pure Compile Minimal Map/Array Deletion

## Goal

Remove the next two raw MIR recognizers from
`hako_llvmc_ffi_pure_compile_minimal_paths.inc`:

- path #3: `MapBox` constructor, `set`, `size/len`, `ret`
- path #4: `ArrayBox` constructor, `push`, `len/length/size`, `ret`

These are CoreBox method shortcuts. The C boundary must not own constructor /
method route legality.

## Plan

1. Delete path #3 and #4 recognizer blocks.
2. Rebuild the C FFI.
3. Run pure keep, pure historical, and llvmlite monitor canaries.
4. If all pass, prune `hako_llvmc_ffi_pure_compile_minimal_paths.inc` in the
   allowlist to the new guard count.

If deletion exposes a missing real owner, add a MIR-owned route or fix generic
method lowering. Do not add another `.inc` shape recognizer.

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
