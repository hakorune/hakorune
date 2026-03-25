---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P13` の次 bucket として、compat keep owner cluster (`hako_forward_bridge.rs`, `hako_forward.rs`, `hako_forward_registry.c`, `hako_forward_registry_shared_impl.inc`, `hako_kernel.c`) が still-live keep か、archive-ready かを棚卸しする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P4-BYN-MIN4-HOOK-REGISTRY-CLOSEOUT.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/hako_forward.rs
  - crates/nyash_kernel/src/hako_forward_registry.c
  - lang/c-abi/shims/hako_forward_registry_shared_impl.inc
  - lang/c-abi/shims/hako_kernel.c
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P14: BYN-min5 Compat Keep Readiness Inventory

## Purpose

- decide whether the compat keep cluster is still a live keep surface under `P9`
- keep this as inventory/judgment first, not a delete wave
- isolate hook/registry/fallback policy questions from compiled-stage1 proof questions

## Fixed Targets

1. `crates/nyash_kernel/src/hako_forward_bridge.rs`
2. `crates/nyash_kernel/src/hako_forward.rs`
3. `crates/nyash_kernel/src/hako_forward_registry.c`
4. `lang/c-abi/shims/hako_forward_registry_shared_impl.inc`
5. `lang/c-abi/shims/hako_kernel.c`

## Current Truth

1. `hako_forward_bridge.rs` still owns the Rust-side keep bridge for register/try-call/fallback contract
2. `hako_forward_registry_shared_impl.inc` still owns the single shared C registry body
3. `hako_forward_registry.c` and `hako_kernel.c` remain include owners only
4. forward-hook acceptance still depends on the explicit keep contract tests
5. this bucket exists because `P9` remains negative even after `P13` confirmed the surrogate proof cluster is still live
6. current evidence says the compat keep cluster is still a live keep owner, not archive-ready
7. `plugin/invoke/by_name.rs` still carries a built-in `FileBox` compat surface that keeps this cluster live

## Acceptance

1. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
2. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
3. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`
4. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a fresh live caller-proof says the keep cluster can shrink further
2. a regression shows the keep cluster is the only green hook/fallback path again
3. docs stop making it clear that this bucket is about compat keep readiness, not surrogate proof

## Non-Goals

1. deleting `hako_forward_bridge.rs`
2. deleting `hako_forward_registry_shared_impl.inc`
3. touching `module_string_dispatch.rs`
4. touching `build_surrogate.rs`

## Next Exact Front

1. `P15-BYN-MIN5-FILEBOX-BUILTIN-KEEP-INVENTORY.md`
