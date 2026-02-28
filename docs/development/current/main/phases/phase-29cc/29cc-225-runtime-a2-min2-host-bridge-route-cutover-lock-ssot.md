---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A2-min2 として、host_bridge 呼び出し経路を per-Box invoke 優先の mainline 契約へ固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-224-runtime-a2-min1-ffi-bridge-route-hardening-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/host_bridge.rs
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
  - src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
  - src/runtime/plugin_loader_v2/enabled/loader/singletons.rs
---

# 29cc-225 Runtime A2-min2 Host Bridge Route Cutover Lock

## Purpose

`host_bridge` の呼び出し境界を `BoxInvokeFn` 優先に固定し、mainline fail-fast で shim fallback へ流れない契約を明示する。

## Fixed Contract

1. `host_bridge` は `invoke_alloc_box` を公開し、per-Box invoke を直接呼べる。
2. `host_bridge::invoke_alloc_with_route` は `BoxInvokeFn` があれば必ずそれを使う。
3. `BoxInvokeFn` が未解決で `NYASH_FAIL_FAST=1` の場合は `E_PLUGIN(-5)` を返して即停止する（shim fallback なし）。
4. shim fallback は `NYASH_FAIL_FAST=0` の compat-only 経路に限定する。
5. `instance_manager` / `ffi_bridge` / `loader::singletons` は `invoke_alloc_with_route` を通す。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. `PluginHandleInner.invoke_fn` の型切替（`InvokeFn -> BoxInvokeFn`）
2. `types.rs` / `loader/metadata.rs` の構造整理（A3で扱う）
3. Rust source 削除（no-delete-first を維持）
