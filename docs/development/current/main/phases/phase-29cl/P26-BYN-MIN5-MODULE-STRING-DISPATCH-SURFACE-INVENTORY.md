---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P25` after the test-only wrapper removal; inventory the remaining kernel `module_string_dispatch` surface and decide whether it is still a live proof owner or just archive-only routing residue.
Related:
  - docs/development/current/main/phases/phase-29cl/P25-BYN-MIN5-CORE-BY-NAME-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/mod.rs
  - crates/nyash_kernel/src/tests.rs
---

# P26: BYN-min5 Module-String Dispatch Surface Inventory

## Purpose

- decide whether `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` is still a live proof owner
- keep build/llvm backend surrogates frozen while examining the parent router surface
- avoid widening to `by_name.rs` or reopening archive-only surrogate packs

## Current Truth

1. `plugin/mod.rs` no longer owns a test-only `try_module_string_dispatch(...)` wrapper
2. `tests.rs` now calls `module_string_dispatch::try_dispatch(...)` directly
3. `module_string_dispatch.rs` remains the remaining router surface in the core owner set
4. `build_surrogate.rs` and `llvm_backend_surrogate.rs` remain archive-only proof residue

## Next Exact Front

1. `P27-BYN-MIN5-MIRBUILDER-DIRECT-MISS-RETIRE.md`
