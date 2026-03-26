---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P26` after module-string inventory; retire the remaining MirBuilder direct-miss compat proof into a dedicated helper so the direct route is explicit and isolated.
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P26-BYN-MIN5-MODULE-STRING-DISPATCH-SURFACE-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P24-BYN-MIN5-KNOWN-BOX-DIRECT-MISS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - src/llvm_py/instructions/direct_box_method.py
  - src/llvm_py/instructions/mir_builder_direct.py
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
  - src/llvm_py/tests/test_method_call_stage1_module_alias.py
---

# P27: BYN-min5 MirBuilder Direct-Miss Retire

## Purpose

- retire the remaining visible MirBuilder direct-miss compat proof from the generic known-box path
- keep `module_string_dispatch.rs` frozen as a live router / archive-only surrogate parent
- isolate the MirBuilder direct lowering ownership so it cannot drift back into the generic boxcall tail

## Current Truth

1. `module_string_dispatch.rs` remains a live router surface
2. `build_surrogate.rs` and `llvm_backend_surrogate.rs` remain archive-only proof residue
3. `MirBuilderBox.emit_from_program_json_v0` and `MirBuilderBox.emit_from_source_v0` now route through the dedicated direct helper in `src/llvm_py/instructions/mir_builder_direct.py`
4. `src/llvm_py/instructions/direct_box_method.py` now owns the MirBuilder direct-miss slice explicitly instead of relying on generic known-box fallback ordering
5. `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py` and `src/llvm_py/tests/test_method_call_stage1_module_alias.py` keep the direct-route proof green while the helper stays isolated

## Next Exact Front

1. `P28-BYN-MIN5-MODULE-STRING-DISPATCH-LIVE-ROUTER-INVENTORY.md`
