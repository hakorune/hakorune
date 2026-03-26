---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P27` after MirBuilder direct-miss isolation; inventory the live `module_string_dispatch` parent router without reopening archive-only surrogate packs.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P26-BYN-MIN5-MODULE-STRING-DISPATCH-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P27-BYN-MIN5-MIRBUILDER-DIRECT-MISS-RETIRE.md
  - docs/development/current/main/phases/phase-29cl/P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md
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
5. `crates/nyash_kernel/src/plugin/invoke/by_name.rs` still probes `module_string_dispatch::try_dispatch(...)` before generic named-receiver fallback, so the parent router remains live
6. kernel tests still pin `lang.compiler.entry.using_resolver(_box).resolve_for_source` and `lang.mir.builder.MirBuilderBox.emit_from_source_v0` through the module-string dispatch surface
7. no Python-side `by_name` compat proof remains for MirBuilder direct lowering; `test_boxcall_plugin_invoke_args.py`, `test_method_call_stage1_module_alias.py`, and `test_method_fallback_tail.py` now pin direct-route only
8. no new `by_name` widening is allowed while the router inventory is open

## Next Exact Front

1. `P29-BYN-MIN5-USING-RESOLVER-STUB-INVENTORY.md`
