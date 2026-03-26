---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P17` の次 bucket として、`llvm_backend_surrogate.rs` が still-live proof owner か、archive-ready かを route-level で棚卸しする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P17-BYN-MIN5-BUILD-SURROGATE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P13-BYN-MIN5-COMPILED-STAGE1-PROOF-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/README.md
  - src/llvm_py/tests/test_method_call_stage1_module_alias.py
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh
---

# P18: BYN-min5 LLVM Backend Surrogate Readiness Inventory

## Purpose

- decide whether the `llvm_backend_surrogate.rs` compiled-stage1 route is still a live proof owner under `P9`
- keep this as inventory/judgment first, not a delete wave
- isolate `selfhost.shared.backend.llvm_backend::{compile_obj,link_exe}` proof questions from the broader compat-keep questions
- treat `compile_obj` and `link_exe` as one route-level owner for this bucket; do not split them yet

## Fixed Targets

1. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/README.md`
4. `src/llvm_py/tests/test_method_call_stage1_module_alias.py`
5. `tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Current Truth

1. `module_string_dispatch.rs` still probes `llvm_backend_surrogate.rs` through `try_dispatch(...)`
2. `llvm_backend_surrogate.rs` still owns the compiled-stage1 `compile_obj` and `link_exe` proof routes
3. `src/llvm_py/tests/test_method_call_stage1_module_alias.py` still pins the direct backend-surrogate lowering path
4. the backend-surrogate route is still exercised by `llvm_backend_route_contract_is_stable`
5. `phase29ck_vmhako_llvm_backend_runtime_proof.sh` still keeps `compile_obj` and `link_exe` paired in one proof path
6. current evidence says `llvm_backend_surrogate.rs` is still a live proof owner, not archive-ready
7. `P17` already narrowed away from `build_surrogate.rs`, and this bucket narrows the backend surrogate only

## Acceptance

1. `cargo test -p nyash_kernel llvm_backend_route_contract_is_stable -- --nocapture`
2. `cargo test -p nyash_kernel llvm_backend_compile_obj_missing_arg_returns_zero_handle -- --nocapture`
3. `cargo test -p nyash_kernel llvm_backend_link_exe_missing_arg_returns_zero_flag -- --nocapture`
4. `cargo test -p nyash_kernel llvm_backend_surrogate_ -- --nocapture`
5. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_call_stage1_module_alias src.llvm_py.tests.test_boxcall_plugin_invoke_args`
6. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
7. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
8. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a fresh caller-proof shows `llvm_backend_surrogate.rs` is no longer needed
2. a regression shows the backend surrogate has become the only green path again
3. docs stop making it clear that this bucket is about backend-surrogate proof readiness, not build-surrogate or compat residue

## Non-Goals

1. deleting `llvm_backend_surrogate.rs`
2. touching `build_surrogate.rs`
3. touching `hako_forward_bridge.rs`
4. touching `hako_forward_registry_shared_impl.inc`

## Next Exact Front

1. `P9-BYN-MIN5-READINESS-JUDGMENT.md`
