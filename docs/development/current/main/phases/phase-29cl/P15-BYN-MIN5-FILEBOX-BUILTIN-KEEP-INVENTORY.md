---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P14` の次 bucket として、`plugin/invoke/by_name.rs` の built-in `FileBox` compat surface が still-live keep owner か、さらに shrink できる narrow bucket かを棚卸しする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P14-BYN-MIN5-COMPAT-KEEP-READINESS-INVENTORY.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py
  - src/backend/mir_interpreter/handlers/boxes_file.rs
  - src/llvm_py/tests/test_method_fallback_tail.py
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P15: BYN-min5 FileBox Builtin Keep Inventory

## Purpose

- decide whether the built-in `FileBox` branch in `plugin/invoke/by_name.rs` is still a live keep owner under `P9`
- keep this as inventory/judgment first, not a delete wave
- isolate `FileBox` built-in keep questions from the wider hook/registry keep cluster

## Fixed Targets

1. `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
2. `src/llvm_py/instructions/mir_call/filebox_plugin_fallback.py`
3. `src/backend/mir_interpreter/handlers/boxes_file.rs`
4. `src/llvm_py/tests/test_method_fallback_tail.py`
5. `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py`

## Current Truth

1. `filebox_plugin_fallback.py` still owns the explicit Python-side `by_name` emission for `FileBox`
2. the current Python-side allowlist is `open`, `read`, `readBytes`, `writeBytes`, and `close`
3. `plugin/invoke/by_name.rs` still carries a built-in `FileBox` branch for `open`, `read`, `readBytes`, `write`, `writeBytes`, and `close`
4. `boxes_file.rs` already owns direct runtime behavior for `open`, `read`, `readBytes`, `writeBytes`, and `close`
5. `FileBox.write` is already retired from the Python-side compat leaf, but still exists in the kernel built-in keep branch
6. current evidence says this built-in `FileBox` keep surface is the next narrowest compat bucket after `P14`
7. `readBytes` still has active binary-route fixture proof, so it is not the smallest leaf
8. `writeBytes` has no current caller-proof beyond keep implementations, so it is the narrowest next shrink bucket

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_boxcall_plugin_invoke_args`
2. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
3. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
4. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`
5. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a fresh caller-proof shows one `FileBox` method is still needed on built-in `by_name`
2. a regression shows the built-in `FileBox` branch is again the only green compat route
3. docs stop making it clear that this bucket is about built-in `FileBox` keep residue, not the whole compat cluster

## Non-Goals

1. deleting `hako_forward_bridge.rs`
2. deleting `hako_forward_registry_shared_impl.inc`
3. touching `module_string_dispatch.rs`
4. widening `FileBox` compat fallback again

## Next Exact Front

1. `P16-BYN-MIN5-FILEBOX-WRITEBYTES-COMPAT-SHRINK.md`
