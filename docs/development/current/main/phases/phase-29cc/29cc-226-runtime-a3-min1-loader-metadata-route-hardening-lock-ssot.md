---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A3-min1 として、loader metadata の type_id 解決経路を fail-fast mainline 契約へ固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-225-runtime-a2-min2-host-bridge-route-cutover-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/loader/metadata.rs
---

# 29cc-226 Runtime A3-min1 Loader Metadata Route Hardening Lock

## Purpose

`loader/metadata.rs` の per-Box invoke 解決で残っていた generic scan を compat-only に閉じ、mainline fail-fast で route drift を防ぐ。

## Fixed Contract

1. `box_invoke_fn_for_type_id` は config mapping がある場合のみ mainline で解決する。
2. config mapping が取れない場合の `box_specs` 全走査は `NYASH_FAIL_FAST=0` の compat-only とする。
3. fail-fast 既定（`NYASH_FAIL_FAST=1`）では unresolved route を `None` で返し、呼び出し側が fail-fast 停止する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. `types.rs` の `InvokeFn` 型縮退（A3-min2で扱う）
2. loader source 削除（no-delete-first を維持）
