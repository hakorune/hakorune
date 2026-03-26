---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P27` after MirBuilder direct-miss isolation; inventory the live `module_string_dispatch` parent router without reopening archive-only surrogate packs.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P26-BYN-MIN5-MODULE-STRING-DISPATCH-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P27-BYN-MIN5-MIRBUILDER-DIRECT-MISS-RETIRE.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs
---

# P28: BYN-min5 Module-String Dispatch Live Router Inventory

## Purpose

- inventory the live `module_string_dispatch.rs` parent router after MirBuilder direct-miss isolation
- keep `build_surrogate.rs` and `llvm_backend_surrogate.rs` frozen as archive-only proof residue
- decide whether the remaining parent router surface is still live proof owner or only thin routing support

## Current Truth

1. `module_string_dispatch.rs` remains the live parent router surface
2. `build_surrogate.rs` and `llvm_backend_surrogate.rs` remain archive-only proof residue
3. `MirBuilderBox.emit_from_program_json_v0` and `MirBuilderBox.emit_from_source_v0` now use the isolated direct helper path
4. direct helper ownership stays in `src/llvm_py/instructions/mir_builder_direct.py`
5. no new `by_name` widening is allowed while the router inventory is open

## Next Exact Front

1. inventory the router surface before any delete or widen move
