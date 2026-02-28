---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A1-min1 として、method_resolver 依存を mainline 既定経路から切り離す。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-218-plugin-method-resolver-failfast-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
  - src/runtime/plugin_loader_unified.rs
---

# 29cc-222 Runtime A1-min1 Method Resolver Route Cutover Lock

## Purpose

`method_resolver.rs` の legacy/fallback 解決に mainline が流入しないよう、A1-min1 を route cutover で固定する。

## Fixed Contract

1. `ffi_bridge` の mainline 経路は `resolve_method_id()` へ依存しない。
2. method id 解決は selected library の config/spec/resolve_fn に限定する。
3. `plugin_loader_unified::resolve_method` の fallback は compat-only（`NYASH_FAIL_FAST=0` 時のみ）とする。
4. fail-fast 既定時の未解決は `BidError::InvalidMethod` で停止する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. mainline 既定で method resolver fallback への暗黙流入がない

## Not in this lock

1. `method_resolver.rs` の source 削除
2. `instance_manager` の route cutover（A1-min2 で扱う）
3. ABI 面の変更
