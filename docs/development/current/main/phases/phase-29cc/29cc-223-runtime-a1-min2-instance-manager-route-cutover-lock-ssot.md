---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A1-min2 として、instance_manager の route を mainline fail-fast 契約へ固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-219-instance-manager-boundary-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
---

# 29cc-223 Runtime A1-min2 Instance Manager Route Cutover Lock

## Purpose

`instance_manager` の birth contract 解決で、mainline が cross-library/generic fallback に流れないように固定する。

## Fixed Contract

1. config 経路がある場合、selected library の contract のみを採用する。
2. spec fallback は selected library key に限定する。
3. selected library が不明な状態での generic scan は compat-only（`NYASH_FAIL_FAST=0`）に限定する。
4. fail-fast 既定時の未解決は `BidError::InvalidType/InvalidMethod` で停止する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `create_box` 経路で generic scan への暗黙流入がない

## Not in this lock

1. `instance_manager.rs` source 削除
2. `ffi_bridge` route hardening（A2-min1で扱う）
3. ABI 面の変更
