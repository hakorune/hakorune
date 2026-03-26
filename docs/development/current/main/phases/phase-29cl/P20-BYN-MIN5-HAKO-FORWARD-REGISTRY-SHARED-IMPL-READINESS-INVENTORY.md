---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P19` の次 bucket として、`hako_forward_registry_shared_impl.inc` が still-live keep body か、frozen exact keep body かを file-level で棚卸しする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P5-BYN-MIN5-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P9-BYN-MIN5-READINESS-JUDGMENT.md
  - docs/development/current/main/phases/phase-29cl/P14-BYN-MIN5-COMPAT-KEEP-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P19-BYN-MIN5-HAKO-FORWARD-BRIDGE-READINESS-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P8-BYN-MIN5-COMPAT-KEEP-ARCHIVE-ONLY.md
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/hako_forward.rs
  - crates/nyash_kernel/src/hako_forward_registry.c
  - lang/c-abi/shims/hako_forward_registry_shared_impl.inc
  - lang/c-abi/shims/hako_kernel.c
  - crates/nyash_kernel/src/tests.rs
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P20: BYN-min5 Hako Forward Registry Shared Impl Readiness Inventory

## Purpose

- decide whether `hako_forward_registry_shared_impl.inc` is still a live keep body under `P9`
- keep this as inventory/judgment first, not a delete wave
- isolate the single shared C registry body from the Rust-side bridge and exported shim wrappers

## Fixed Targets

1. `lang/c-abi/shims/hako_forward_registry_shared_impl.inc`
2. `crates/nyash_kernel/src/hako_forward_bridge.rs`
3. `crates/nyash_kernel/src/hako_forward.rs`
4. `crates/nyash_kernel/src/hako_forward_registry.c`
5. `lang/c-abi/shims/hako_kernel.c`
6. `crates/nyash_kernel/src/tests.rs`

## Current Truth

1. `hako_forward_registry_shared_impl.inc` still owns the shared C registry body for hook storage, register, and try-call behavior
2. `hako_forward_bridge.rs` still owns the Rust-side keep bridge that calls into that shared body
3. `hako_forward.rs` is only the exported registration shim
4. `hako_forward_registry.c` and `hako_kernel.c` remain include owners only
5. `crates/nyash_kernel/src/tests.rs` still pins the bridge contract through `hako_forward_registration_and_call_contract`
6. current evidence says the shared registry body is a frozen exact keep body, not a live readiness blocker
7. this bucket is narrower than P19, but it remains explicit rather than ambiguous

## Acceptance

1. `cargo test -p nyash_kernel hako_forward_registration_and_call_contract -- --nocapture`
2. `cargo test -p nyash_kernel string_exports_disable_rust_fallback_when_policy_is_off -- --nocapture`
3. `cargo test -p nyash_kernel future_spawn_instance_disable_rust_fallback_when_policy_is_off -- --nocapture`
4. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`

## Reopen Rule

Reopen this bucket only when one of these is true.

1. a fresh live caller-proof shows the shared registry body can shrink further
2. a regression shows the shared registry body is the only green hook/fallback path again
3. docs stop making it clear that this bucket is about the shared C registry body, not the Rust bridge

## Non-Goals

1. deleting `hako_forward_registry_shared_impl.inc`
2. deleting `hako_forward_bridge.rs`
3. touching `module_string_dispatch.rs`
4. touching `build_surrogate.rs`

## Next Exact Front

1. `P9-BYN-MIN5-READINESS-JUDGMENT.md`
