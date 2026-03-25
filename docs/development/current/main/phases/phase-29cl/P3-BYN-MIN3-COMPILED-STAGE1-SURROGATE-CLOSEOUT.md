---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min3` compiled-stage1 surrogate cluster (`module_string_dispatch.rs`, `build_surrogate.rs`, `llvm_backend_surrogate.rs`) を docs/inventory closeout で固定し、caller-proof が出るまで code removal/reopen を止める。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/README.md
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh
---

# P3: BYN-min3 Compiled-Stage1 Surrogate Closeout

## Purpose

- `build_surrogate.rs` / `llvm_backend_surrogate.rs` を new active code target にしない。
- current compiled-stage1 surrogate cluster を frozen exact owners として docs/inventory で閉じる。
- caller-proof が無い段階で surrogate code removal や refactor-only churn を防ぐ。

## Frozen Owners

1. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - thin parent router + shared decode/gate helpers
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
   - compiled-stage1 `BuildBox.emit_program_json_v0` surrogate only
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
   - compiled-stage1 `selfhost.shared.backend.llvm_backend.{compile_obj,link_exe}` surrogate only

## Current Truth

1. visible launcher and compiled-stage1 callers are already off explicit `nyash.plugin.invoke_by_name_i64`.
2. visible launcher build-exe source lane now also uses direct `LlvmBackendBox.{compile_obj,link_exe}` instead of a quoted module-string backend literal.
3. direct-known-box lowering prefers `BuildBox` / `MirBuilderBox` / `LlvmBackendBox` before compat fallback.
4. `module_string_dispatch` cluster is at thin floor and remains temporary proof owner only.
5. acceptance proof is green, so current move is closed docs/inventory closeout only.
6. next exact front is `BYN-min4` hook/registry closeout; do not mix surrogate reopen with hook/registry demotion.

## Acceptance

1. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
3. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`

## Reopen Rule

Reopen `BYN-min3` code only when one of these is true.

1. a fresh live caller still requires `build_surrogate.rs` or `llvm_backend_surrogate.rs`
2. `phase29cl_by_name_lock_vm.sh` regresses and the surrogate becomes the only green path
3. docs or caller inventory become ambiguous about whether the surrogate cluster is still temporary

## Non-Goals

1. deleting `build_surrogate.rs`
2. deleting `llvm_backend_surrogate.rs`
3. widening module-string dispatch responsibility
4. mixing `BYN-min3` closeout with `BYN-min4` hook/registry demotion
