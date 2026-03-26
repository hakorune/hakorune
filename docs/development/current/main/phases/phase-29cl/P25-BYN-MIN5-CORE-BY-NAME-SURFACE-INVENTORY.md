---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: Python-side known-box direct-miss fallback の退役後に残る kernel core `by_name` surface を inventory し、次の execution/delete judgment を 1 本に固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P24-BYN-MIN5-KNOWN-BOX-DIRECT-MISS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/mod.rs
  - crates/nyash_kernel/src/tests.rs
---

# P25: BYN-min5 Core By-Name Surface Inventory

## Purpose

- inventory the remaining core `by_name` surface now that Python-side emitters are gone
- keep archive-only surrogate and hook/registry packs frozen
- choose the next narrow kernel-side execution or delete slice without reopening old caller lanes

## Current Truth

1. Python-side known-box direct-miss fallback is retired
2. `nyash.plugin.invoke_by_name_i64` remains only as a kernel compat/archive surface
3. the visible core owner set is now:
   - `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
   - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - `crates/nyash_kernel/src/plugin/mod.rs`
   - `crates/nyash_kernel/src/tests.rs`
4. `plugin/mod.rs` no longer owns a test-only `try_module_string_dispatch(...)` wrapper
5. the next exact front is `P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md`

## Next Exact Front

1. `P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md`
