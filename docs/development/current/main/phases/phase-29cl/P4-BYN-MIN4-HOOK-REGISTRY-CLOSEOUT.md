---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min4` hook/registry keep cluster (`hako_forward_bridge.rs`, `hako_forward.rs`, `hako_forward_registry.c`, `hako_forward_registry_shared_impl.inc`, `hako_kernel.c`) を explicit compat-only owner として closeout し、fresh live caller proof が出るまで code widening/rewrite を止める。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - docs/development/current/main/phases/phase-29cl/P3-BYN-MIN3-COMPILED-STAGE1-SURROGATE-CLOSEOUT.md
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/hako_forward.rs
  - crates/nyash_kernel/src/hako_forward_registry.c
  - lang/c-abi/shims/hako_forward_registry_shared_impl.inc
  - lang/c-abi/shims/hako_kernel.c
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
  - tools/checks/phase29cl_by_name_mainline_guard.sh
---

# P4: BYN-min4 Hook/Registry Closeout

## Purpose

- `hako_forward_bridge.rs` / `hako_forward.rs` / shared C registry surface を new mainline owner に戻さない。
- current hook/registry keep cluster を explicit compat-only owner として docs/inventory で閉じる。
- duplicate registry behavior や silent widening を防ぎ、fresh live caller proof が出るまで code churn を止める。

## Frozen Owners

1. `crates/nyash_kernel/src/hako_forward_bridge.rs`
   - Rust-side compat keep bridge for hook registration / try-call / fallback contract only
2. `crates/nyash_kernel/src/hako_forward.rs`
   - exported registration entry shim only
3. `crates/nyash_kernel/src/hako_forward_registry.c`
   - nyash-kernel-side C include owner only
4. `lang/c-abi/shims/hako_forward_registry_shared_impl.inc`
   - shared compat-only C registry body for future/string hook storage, register, and try-call
5. `lang/c-abi/shims/hako_kernel.c`
   - libc shim include owner only; no duplicate hook registry behavior inline

## Current Truth

1. `hako_forward_registry_shared_impl.inc` is the single shared C owner for hook registry storage and try-call behavior.
2. `hako_forward_registry.c` and `hako_kernel.c` are include owners only; they do not each keep duplicate registry logic inline.
3. `hako_forward_bridge.rs` is already at thin floor:
   - FFI declarations
   - compat keep call/register wrappers
   - fallback policy / freeze contract
   - owner-local regression proof
4. the forward bridge cluster is explicit compat-only residue, not a new daily by-name owner.
5. acceptance proof is green, so current move is closed docs/inventory closeout only.
6. next exact front is `BYN-min5` hard-retire readiness only when no daily caller and no compiled-stage1 proof still require the keep cluster.

## Acceptance

1. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
2. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
3. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
5. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Reopen Rule

Reopen `BYN-min4` code only when one of these is true.

1. a fresh live caller still requires new hook/registry behavior beyond the frozen keep surface
2. duplicate registry ownership reappears across `hako_forward_registry.c` and `hako_kernel.c`
3. `phase29cl_by_name_lock_vm.sh` or `phase29cl_by_name_mainline_guard.sh` regresses and the hook/registry keep becomes ambiguous
4. docs stop making it clear that the hook/registry cluster is compat-only residue

## Non-Goals

1. deleting `hako_forward_bridge.rs`
2. deleting `hako_forward_registry_shared_impl.inc`
3. widening hook registration or fallback policy
4. mixing `BYN-min4` closeout with `BYN-min5` hard-retire readiness
