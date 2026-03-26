---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P16` の次 bucket として、`build_surrogate.rs` が still-live proof owner か、archive-only proof residue かを単独で棚卸しする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P13-BYN-MIN5-COMPILED-STAGE1-PROOF-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/README.md
  - src/llvm_py/tests/test_boxcall_plugin_invoke_args.py
  - src/llvm_py/tests/test_method_call_stage1_module_alias.py
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P17: BYN-min5 Build Surrogate Readiness Inventory

## Purpose

- decide whether the `build_surrogate.rs` compiled-stage1 route is still a live proof owner under `P9`
- keep this as inventory/judgment first, not a delete wave
- isolate `BuildBox.emit_program_json_v0` proof questions from the broader `llvm_backend_surrogate.rs` and compat-keep questions

## Fixed Targets

1. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/README.md`
4. `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py`

## Current Truth

1. `module_string_dispatch.rs` still probes `build_surrogate.rs` through `try_dispatch(...)`
2. `build_surrogate.rs` still keeps the compiled-stage1 `BuildBox.emit_program_json_v0` route local to the surrogate cluster
3. `src/llvm_py/tests/test_boxcall_plugin_invoke_args.py` pins the direct BuildBox lowering path rather than a generic `by_name` path
4. the build-surrogate route is still exercised by `build_surrogate_route_contract_is_stable`
5. `P13` narrowed the broader compiled-stage1 question, and this bucket confirms the build surrogate is not the only green proof path
6. current evidence says `build_surrogate.rs` is archive-only proof residue, not a live proof owner

## Acceptance

1. `cargo test -p nyash_kernel build_surrogate_route_contract_is_stable -- --nocapture`
2. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_boxcall_plugin_invoke_args src.llvm_py.tests.test_method_call_stage1_module_alias`
3. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a fresh caller-proof shows `BuildBox.emit_program_json_v0` is no longer needed
2. a regression shows the build surrogate has become the only green path again
3. docs stop making it clear that this bucket is about build-surrogate proof readiness, not backend or compat residue

## Non-Goals

1. deleting `build_surrogate.rs`
2. touching `llvm_backend_surrogate.rs`
3. touching `hako_forward_bridge.rs`
4. touching `hako_forward_registry_shared_impl.inc`

## Next Exact Front

1. `P18-BYN-MIN5-LLVM-BACKEND-SURROGATE-READINESS-INVENTORY.md`
