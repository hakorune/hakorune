---
Status: Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` readiness judgment negative の直後に進める最小 blocker bucket として、FileBox compat leaf とその direct-miss caller residue をさらに縮める。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - src/llvm_py/instructions/direct_box_method.py
  - src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py
  - src/llvm_py/tests/test_method_fallback_tail.py
  - src/llvm_py/tests/test_method_call_stage1_module_alias.py
  - src/llvm_py/tests/test_mir_call_hot_fallback.py
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P10: BYN-min5 FileBox Compat Leaf Shrink

## Purpose

- Shrink the narrowest remaining live blocker after the negative `P9` judgment.
- Keep the explicit FileBox compat helper from acting like a generic daily caller path.
- Do not mix this wave with compiled-stage1 surrogate or hook/registry demotion.

## Fixed Targets

1. `src/llvm_py/instructions/direct_box_method.py`
2. `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`
3. focused caller-proof tests
   - `src/llvm_py/tests/test_method_fallback_tail.py`
   - `src/llvm_py/tests/test_method_call_stage1_module_alias.py`
   - `src/llvm_py/tests/test_mir_call_hot_fallback.py`

## Current Truth

1. `direct_box_method.py` is thinner than before, but miss handling still allows the explicit FileBox compat leaf.
2. `filebox_plugin_fallback.py` is the remaining Python-side emitter of `nyash.plugin.invoke_by_name_i64`.
3. this is the smallest live blocker bucket under the negative `P9` judgment.
4. `method.rs`, `type_registry.rs`, and `unified_dispatch.rs` remain larger migration targets and are not the first next slice.

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_method_call_stage1_module_alias src.llvm_py.tests.test_mir_call_hot_fallback`
2. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this wave only when one of these is true.

1. a new direct-miss caller begins emitting `nyash.plugin.invoke_by_name_i64`
2. FileBox compat handling becomes the only green path for another daily caller family
3. docs stop making it clear that this is the narrowest blocker under the negative `P9` judgment

## Non-Goals

1. modifying `module_string_dispatch.rs`
2. modifying `hako_forward_bridge.rs`
3. deleting `by_name.rs`
4. mixing this wave with hard-retire execution
