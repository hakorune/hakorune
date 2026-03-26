---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P30` after confirming `emit_from_source_v0` stays live compat; inventory the remaining `MirBuilderBox.emit_from_program_json_v0` module-string seam without reopening source or surrogate buckets.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P30-BYN-MIN5-MIRBUILDER-SOURCE-SEAM-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P32-BYN-MIN5-PROGRAM-JSON-LIVE-CALLER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - lang/src/runner/stage1_cli.hako
  - lang/src/runner/stage1_cli_env.hako
  - lang/src/runner/launcher.hako
  - src/llvm_py/instructions/mir_builder_direct.py
---

# P31: BYN-min5 MirBuilder Program-JSON Seam Inventory

## Purpose

- inventory the remaining kernel/module-string `MirBuilderBox.emit_from_program_json_v0` seam inside `module_string_dispatch.rs`
- keep `emit_from_source_v0` judgment closed and separate
- decide whether the Program(JSON) seam is still a live compat owner or only thin residual support

## Current Truth

1. `module_string_dispatch.rs` still carries `handle_mir_builder_emit_from_program_json_v0(...)`
2. kernel tests still pin direct route proof and contract/freeze handling for `emit_from_program_json_v0`
3. language-side live/bootstrap callers still use `MirBuilderBox.emit_from_program_json_v0(...)` in runner owners
4. LLVM Python direct lowering for `MirBuilderBox.emit_from_program_json_v0` is already isolated from generic by-name fallback
5. current judgment: `emit_from_program_json_v0` remains a live compat owner, not frozen residue yet
6. this slice must not reopen `emit_from_source_v0`, `resolve_for_source`, `build_surrogate.rs`, or `llvm_backend_surrogate.rs`

## Next Exact Front

1. `P32-BYN-MIN5-PROGRAM-JSON-LIVE-CALLER-INVENTORY.md`
