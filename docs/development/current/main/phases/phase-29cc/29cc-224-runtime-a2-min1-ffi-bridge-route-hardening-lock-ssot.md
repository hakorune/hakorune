---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A2-min1 として、ffi_bridge の type/method 解決を fail-fast mainline 契約へ固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-222-runtime-a1-min1-method-resolver-route-cutover-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
---

# 29cc-224 Runtime A2-min1 FFI Bridge Route Hardening Lock

## Purpose

`ffi_bridge` の解決経路を selected-library deterministic route に固定し、mainline での generic fallback を排除する。

## Fixed Contract

1. `resolve_type_info` は config/select key を優先し、mainline fail-fast では generic box scan を行わない。
2. config 不在時の generic scan は compat-only（`NYASH_FAIL_FAST=0`）に限定する。
3. compat fallback は lexical deterministic selection を使う（再現性確保）。
4. mainline fail-fast 既定の未解決は `BidError::InvalidType` で停止する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. `host_bridge` route cutover（A2-min2で扱う）
2. `ffi_bridge.rs` source 削除
3. ABI 面の変更
