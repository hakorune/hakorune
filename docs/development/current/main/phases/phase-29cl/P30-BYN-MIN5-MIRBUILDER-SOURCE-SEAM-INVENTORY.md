---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P29` after confirming `resolve_for_source` stays live keep; inventory the remaining `MirBuilderBox.emit_from_source_v0` module-string seam without reopening surrogate or resolver buckets.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/tests.rs
  - lang/src/runner/stage1_cli_env.hako
  - src/llvm_py/instructions/mir_builder_direct.py
---

# P30: BYN-min5 MirBuilder Source Seam Inventory

## Purpose

- inventory the remaining kernel/module-string `MirBuilderBox.emit_from_source_v0` seam inside `module_string_dispatch.rs`
- keep `resolve_for_source` stub judgment closed and separate
- decide whether the source seam is still a live compat owner or only thin residual support

## Current Truth

1. `module_string_dispatch.rs` still carries `handle_mir_builder_emit_from_source_v0(...)`
2. kernel tests still pin both direct `dispatch_stage1_module(...)` proof and exported `nyash_plugin_invoke_by_name_i64(...)` proof for `emit_from_source_v0`
3. `lang/src/runner/stage1_cli_env.hako` still calls `MirBuilderBox.emit_from_source_v0(...)` on the language side
4. LLVM Python direct lowering for `MirBuilderBox.emit_from_source_v0` is already isolated in `src/llvm_py/instructions/mir_builder_direct.py`
5. this slice must not reopen `build_surrogate.rs`, `llvm_backend_surrogate.rs`, or `resolve_for_source`

## Next Exact Front

1. inventory `emit_from_source_v0` caller-proof and decide whether the seam stays live compat or can move to frozen residue
