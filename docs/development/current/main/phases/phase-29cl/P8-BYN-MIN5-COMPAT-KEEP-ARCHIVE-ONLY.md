---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `BYN-min5` readiness runway の third blocker bucket を compat keep owners に固定し、hook / registry / fallback policy を explicit archive-only closeout として閉じる。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P6-BYN-MIN5-DAILY-CALLER-SHRINK.md
  - docs/development/current/main/phases/phase-29cl/P7-BYN-MIN5-COMPILED-STAGE1-PROOF-FREEZE.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P0-BY-NAME-OWNER-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P2-BY-NAME-ACCEPTANCE-AND-REOPEN-RULE.md
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/hako_forward.rs
  - crates/nyash_kernel/src/hako_forward_registry.c
  - lang/c-abi/shims/hako_forward_registry_shared_impl.inc
  - lang/c-abi/shims/hako_kernel.c
  - crates/nyash_kernel/src/tests.rs
---

# P8: BYN-min5 Compat Keep Archive-Only Judgment

## Purpose

- Decide whether the compat keep cluster can be treated as archive-only.
- Keep hook / registry / fallback policy explicit and avoid duplicate C registry owners.
- This is the final blocker bucket before `BYN-min5` readiness judgment can start.

## Frozen Owners

1. `crates/nyash_kernel/src/hako_forward_bridge.rs`
   - Rust-side compat keep bridge for hook registration / try-call / fallback contract only
2. `crates/nyash_kernel/src/hako_forward.rs`
   - exported registration entry shim only
3. `crates/nyash_kernel/src/hako_forward_registry.c`
   - nyash-kernel-side C include owner only
4. `lang/c-abi/shims/hako_forward_registry_shared_impl.inc`
   - shared compat-only C registry body for hook storage, register, and try-call
5. `lang/c-abi/shims/hako_kernel.c`
   - libc shim include owner only; no duplicate hook registry behavior inline

## Current Truth

1. `hako_forward_registry_shared_impl.inc` is the single shared C owner for hook registry storage and try-call behavior.
2. `hako_forward_registry.c` and `hako_kernel.c` are include owners only; they do not each keep duplicate registry logic inline.
3. `hako_forward_bridge.rs` is already at thin floor and remains explicit compat-only residue.
4. `BYN-min5` readiness judgment can start now that this compat bucket is closed.

## Acceptance

1. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
2. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
3. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`
4. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
5. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`

## Reopen Rule

Reopen this wave only when one of these is true.

1. a fresh live caller still requires new hook/registry behavior beyond the frozen keep surface
2. duplicate registry ownership reappears across `hako_forward_registry.c` and `hako_kernel.c`
3. `phase29cl_by_name_lock_vm.sh` or `phase29cl_by_name_mainline_guard.sh` regresses and the hook/registry keep becomes ambiguous
4. docs stop making it clear that the hook/registry cluster is compat-only residue

## Non-Goals

1. deleting `hako_forward_bridge.rs`
2. deleting `hako_forward_registry_shared_impl.inc`
3. widening hook registration or fallback policy
4. mixing this wave with `BYN-min5` hard-retire judgment

## Next Exact Front

1. `P9-BYN-MIN5-READINESS-JUDGMENT.md`
