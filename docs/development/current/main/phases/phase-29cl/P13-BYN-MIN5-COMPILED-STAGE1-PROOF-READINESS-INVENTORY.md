---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P9` re-check の次 bucket として、compiled-stage1 proof owner cluster (`module_string_dispatch.rs`, `build_surrogate.rs`, `llvm_backend_surrogate.rs`) がまだ live proof owner か、archive-only residue かを棚卸しする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch/README.md
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
  - tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh
---

# P13: BYN-min5 Compiled-Stage1 Proof Readiness Inventory

## Purpose

- decide whether the compiled-stage1 surrogate cluster is still a live proof owner under `P9`
- keep this as inventory/judgment first, not a delete wave
- avoid mixing surrogate proof questions with hook/registry compat keep questions

## Fixed Targets

1. `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
3. `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs`
4. `crates/nyash_kernel/src/plugin/module_string_dispatch/README.md`

## Current Truth

1. `module_string_dispatch.rs` still probes both surrogates through `try_dispatch(...)`
2. `build_surrogate.rs` and `llvm_backend_surrogate.rs` still keep the compiled-stage1 route bodies local to the surrogate cluster
3. current direct caller proof lives in launcher/stage1/backend routes, not in a generic module-string `by_name` path
4. `phase29cl_by_name_lock_vm.sh` and `phase29ck_vmhako_llvm_backend_runtime_proof.sh` stay green while direct BuildBox / LlvmBackendBox callers remain visible
5. `P3` and `P7` already froze the cluster, and this bucket exists to decide whether that frozen cluster still blocks readiness
6. current evidence says the cluster is archive-only proof residue, not a live proof owner

## Acceptance

1. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
3. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
4. `cargo test -p nyash_kernel build_surrogate_route_contract_is_stable -- --nocapture`
5. `cargo test -p nyash_kernel llvm_backend_surrogate_ -- --nocapture`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a fresh caller-proof shows one surrogate is no longer needed
2. a fresh regression shows a surrogate has become the only green path again
3. docs stop making it clear that this bucket is about proof readiness, not hook/registry residue

## Non-Goals

1. deleting `build_surrogate.rs`
2. deleting `llvm_backend_surrogate.rs`
3. touching `hako_forward_bridge.rs`
4. touching `hako_forward_registry_shared_impl.inc`

## Next Exact Front

1. `P14-BYN-MIN5-COMPAT-KEEP-READINESS-INVENTORY.md`
