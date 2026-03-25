---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` readiness runway の second blocker bucket を compiled-stage1 proof owners に固定し、`module_string_dispatch` 群を frozen exact owners として閉じる前提を文書化する。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/README.md
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh
  - tools/checks/phase29cl_by_name_mainline_guard.sh
---

# P7: BYN-min5 Compiled-Stage1 Proof Freeze

## Purpose

- Keep the compiled-stage1 surrogate cluster frozen while caller-proof is pending.
- Do not treat the surrogates as permanent owners, but also do not reopen or delete them while proof is still needed.
- This is a freeze/closeout runway, not a new dispatch design.

## Frozen Owners

1. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - thin parent router plus shared decode/gate helpers
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
   - compiled-stage1 `BuildBox.emit_program_json_v0` surrogate only
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
   - compiled-stage1 `selfhost.shared.backend.llvm_backend.{compile_obj,link_exe}` surrogate only

## Current Truth

1. launcher and compiled-stage1 visible callers are already off explicit `nyash.plugin.invoke_by_name_i64`.
2. launcher build-exe source lane now uses direct `LlvmBackendBox.{compile_obj,link_exe}` instead of a quoted module-string backend literal.
3. the surrogate cluster is at thin floor and still serves as temporary proof owner only.
4. `BYN-min5` is still not open because this proof bucket is not yet ready to be demoted further.

## Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
3. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
4. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_call_stage1_module_alias src.llvm_py.tests.test_method_fallback_tail`

## Reopen Rule

Reopen this wave only when one of these is true.

1. a fresh live caller still requires `build_surrogate.rs` or `llvm_backend_surrogate.rs`
2. `phase29cl_by_name_lock_vm.sh` regresses and the surrogate becomes the only green path
3. docs or caller inventory become ambiguous about whether the surrogate cluster is still temporary

## Non-Goals

1. deleting `build_surrogate.rs`
2. deleting `llvm_backend_surrogate.rs`
3. widening module-string dispatch responsibility
4. mixing this wave with hook/registry demotion

## Next Exact Front

1. `P9-BYN-MIN5-READINESS-JUDGMENT.md`
