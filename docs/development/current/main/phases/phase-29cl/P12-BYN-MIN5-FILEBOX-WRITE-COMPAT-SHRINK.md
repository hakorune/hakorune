---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P9` negative re-check の直後に進める最小 Python-side blocker bucket として、explicit FileBox compat leaf から `write` だけを外す。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P10-BYN-MIN5-FILEBOX-COMPAT-LEAF-SHRINK.md
  - src/llvm_py/instructions/direct_box_method.py
  - src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py
  - src/llvm_py/tests/test_method_fallback_tail.py
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P12: BYN-min5 FileBox Write Compat Shrink

## Purpose

- keep `P9` negative after `P11`, but continue shrinking the smallest remaining Python-side compat leaf
- remove only `FileBox.write` from the explicit `nyash.plugin.invoke_by_name_i64` allowlist
- keep the rest of the FileBox compat leaf visible and unchanged

## Fixed Targets

1. `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`
2. focused caller-proof tests
   - `src/llvm_py/tests/test_method_fallback_tail.py`
   - `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py`

## Current Truth

1. `direct_box_method.py` is already only a policy gate and should stay unchanged in this slice
2. `filebox_plugin_fallback.py` remains the only Python-side emitter of `nyash.plugin.invoke_by_name_i64`
3. `write` is the narrowest removable method in that FileBox compat allowlist
4. `open`, `read`, `readBytes`, `writeBytes`, and `close` remain explicit compat methods in this slice
5. `type_registry.rs`, `method.rs`, and `unified_dispatch.rs` stay untouched
6. `FileBox.write` no longer emits `nyash.plugin.invoke_by_name_i64` on the method-call direct-miss path

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_boxcall_plugin_invoke_args`
2. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this wave only when one of these is true.

1. `FileBox.write` must remain the only green method-call path for a daily caller
2. a new FileBox compat method is added to the allowlist
3. docs stop making it clear that this slice removes only `write`

## Non-Goals

1. changing `direct_box_method.py`
2. changing `boxcall.py`
3. changing `type_registry.rs`
4. changing `unified_dispatch.rs`

## Next Exact Front

1. return to `P9-BYN-MIN5-READINESS-JUDGMENT.md` and re-check readiness with the landed `P12` evidence
