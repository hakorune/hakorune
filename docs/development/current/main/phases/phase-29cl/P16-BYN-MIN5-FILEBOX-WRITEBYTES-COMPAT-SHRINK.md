---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P15` の次 bucket として、`FileBox.writeBytes` を compat keep surface からさらに縮められるかを単独で扱う。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P15-BYN-MIN5-FILEBOX-BUILTIN-KEEP-INVENTORY.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py
  - src/backend/mir_interpreter/handlers/boxes_file.rs
  - src/llvm_py/tests/test_method_fallback_tail.py
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
---

# P16: BYN-min5 FileBox writeBytes Compat Shrink

## Purpose

- treat `FileBox.writeBytes` as the next exact shrink bucket under the negative `P9` judgment
- keep this bucket narrower than the full built-in `FileBox` family
- verify that removing `writeBytes` from compat-only by-name surfaces does not disturb the still-live `readBytes` binary route

## Fixed Targets

1. `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`
2. `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
3. `src/llvm_py/tests/test_method_fallback_tail.py`
4. `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py`
5. `src/backend/mir_interpreter/handlers/boxes_file.rs`

## Current Truth

1. `writeBytes` is still present in the explicit Python-side `FileBox` compat allowlist
2. `writeBytes` is still present in the kernel built-in `FileBox` by-name branch
3. `readBytes` still has live binary-route fixture proof, so it is not bundled into this bucket
4. `writeBytes` has no current caller-proof beyond keep implementations, making it the narrowest next shrink candidate

## Landed Truth

1. `writeBytes` is retired from `filebox_plugin_fallback.py`
2. `writeBytes` is retired from the built-in `FileBox` branch in `plugin/invoke/by_name.rs`
3. `readBytes` remains untouched and stays outside this bucket
4. this shrink does not make `P9` automatically positive because compiled-stage1 proof owners and broader compat keep owners still remain

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_boxcall_plugin_invoke_args`
2. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
3. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a live caller-proof appears for `FileBox.writeBytes`
2. a regression shows `writeBytes` is still required to keep the compat surface green
3. docs stop making it clear that `readBytes` and `writeBytes` are not the same bucket anymore

## Non-Goals

1. touching `readBytes`
2. touching `open`, `read`, or `close`
3. touching `module_string_dispatch.rs`
4. broad hook/registry retirement

## Next Exact Front

1. `P9-BYN-MIN5-READINESS-JUDGMENT.md`
