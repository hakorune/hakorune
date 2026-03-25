---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` readiness runway の first blocker bucket を daily caller / caller-shrink residue に固定し、`invoke_by_name_i64` を残す最後の evidence leaf を縮める。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - src/llvm_py/instructions/direct_box_method.py
  - src/backend/mir_interpreter/handlers/calls/method.rs
  - src/runtime/type_registry.rs
  - src/backend/wasm_v2/unified_dispatch.rs
  - src/llvm_py/tests/test_method_fallback_tail.py
  - src/llvm_py/tests/test_method_call_stage1_module_alias.py
  - src/llvm_py/tests/test_mir_call_hot_fallback.py
  - src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py
---

# P6: BYN-min5 Daily Caller Shrink

## Purpose

- Keep the last daily by-name evidence leaf from growing back.
- Shrink the direct-miss fallback leaf and the name-resolution dependent migration targets as one blocker bucket.
- Do not mix this wave with surrogate freeze or hook/registry archive decisions.

## Fixed Inputs

1. target residue
   - `src/llvm_py/instructions/direct_box_method.py`
2. migration targets
   - `src/backend/mir_interpreter/handlers/calls/method.rs`
   - `src/runtime/type_registry.rs`
   - `src/backend/wasm_v2/unified_dispatch.rs`
3. evidence tests
   - `src/llvm_py/tests/test_method_fallback_tail.py`
   - `src/llvm_py/tests/test_method_call_stage1_module_alias.py`
   - `src/llvm_py/tests/test_mir_call_hot_fallback.py`

## Current Truth

1. `direct_box_method.py` now delegates the last FileBox compat leaf into `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`.
2. the FileBox compat leaf is still present, but the direct-route helper itself is now direct-route focused.
3. `method.rs`, `type_registry.rs`, and `unified_dispatch.rs` remain name-resolution dependent migration targets.
4. `BYN-min5` is still not open because this caller bucket is not fully shrunk yet.
5. the compat-only by-name residue is narrower than before, but the daily caller inventory is not empty.

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
3. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_method_call_stage1_module_alias src.llvm_py.tests.test_mir_call_hot_fallback`

## Reopen Rule

Reopen this wave only when one of these is true.

1. a new daily caller appears on `nyash.plugin.invoke_by_name_i64`
2. the direct-miss fallback leaf becomes the only remaining green path for a known caller again
3. docs stop making it clear that these are caller-shrink migration targets

## Non-Goals

1. widening module-string dispatch
2. modifying hook/registry compat keeps
3. deleting `by_name.rs`
4. mixing with compiled-stage1 proof closeout

## Next Exact Front

1. `P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md`
