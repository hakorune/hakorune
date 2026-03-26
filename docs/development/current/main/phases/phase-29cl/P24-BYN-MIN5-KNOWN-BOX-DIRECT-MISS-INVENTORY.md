---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `FileBox` family と built-in `InstanceBox` keep の退役後に残る visible `by_name` residue として、known-box direct-miss fallback を inventory し、次の narrow execution slice を 1 本に固定する。
Related:
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - docs/development/current/main/phases/phase-29cl/P23-BYN-MIN5-INSTANCEBOX-BUILTIN-KEEP-RETIRE.md
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - src/llvm_py/instructions/direct_box_method.py
  - src/llvm_py/instructions/boxcall.py
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
  - src/llvm_py/tests/test_method_call_stage1_module_alias.py
---

# P24: BYN-min5 Known-Box Direct-Miss Inventory

## Purpose

- identify the narrowest visible compat residue after `FileBox` and built-in `InstanceBox` keep retirement
- keep compiled-stage1 surrogate and hook/registry residue frozen
- choose the next execution slice without reopening archive-only proof owners

## Current Truth

1. `FileBox` family no longer uses Python-side or kernel built-in `by_name` keep
2. built-in `InstanceBox.getField/setField` keep is retired from kernel `by_name`
3. `src/llvm_py/instructions/direct_box_method.py` still allows known-box direct-miss fallback under the legacy known-box policy
4. `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py` still pins `MirBuilderBox.emit_from_program_json_v0` direct-miss fallback to `nyash.plugin.invoke_by_name_i64`
5. this residue is narrower than reopening compiled-stage1 surrogate or hook/registry packs

## Inventory Focus

1. `direct_box_method.py`
   - policy owner for known-box direct call vs compat fallback
2. `boxcall.py`
   - visible caller lane that still exercises the direct-miss compat fallback
3. `test_boxcall_plugin_invoke_args.py`
   - current proof that the compat route remains intentionally visible
4. `test_method_call_stage1_module_alias.py`
   - direct-call proof that must stay green while the direct-miss residue is examined

## Next Exact Front

1. narrow the known-box direct-miss fallback inventory to one receiver family before any execution change
