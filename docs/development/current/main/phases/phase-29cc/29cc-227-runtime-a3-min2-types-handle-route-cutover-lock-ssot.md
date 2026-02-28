---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A3-min2 として、types の PluginHandleInner 呼び出しを route-aware（BoxInvoke優先）に固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-225-runtime-a2-min2-host-bridge-route-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-226-runtime-a3-min1-loader-metadata-route-hardening-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/types.rs
---

# 29cc-227 Runtime A3-min2 Types Handle Route Cutover Lock

## Purpose

`PluginHandleInner` の lifecycle/clone 呼び出しが shim 固定のままだと route drift の温床になるため、`BoxInvokeFn` 優先の route-aware 呼び出しへ固定する。

## Fixed Contract

1. `PluginHandleInner` は `invoke_box_fn: Option<BoxInvokeFn>` を保持する。
2. `Drop::drop` / `finalize_now` / `clone_box` は `invoke_alloc_with_route` を使う。
3. `PluginHandleInner` 構築時は `type_id` から `invoke_box_fn` を解決して保存する。
4. fail-fast 既定では unresolved route は compat fallback へ流れない（`invoke_alloc_with_route` 契約に従う）。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. `PluginBoxMetadata.invoke_fn` の型縮退（A3 次境界で扱う）
2. `globals.rs` / `errors.rs` / `extern_functions.rs` の route整理
3. Rust source 削除（no-delete-first を維持）
