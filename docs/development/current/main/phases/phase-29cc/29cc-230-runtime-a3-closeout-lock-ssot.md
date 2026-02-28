---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A3 closeout として、loader/types/globals/errors/extern の route 契約と done criteria を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-226-runtime-a3-min1-loader-metadata-route-hardening-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-227-runtime-a3-min2-types-handle-route-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-228-runtime-a3-min3-globals-errors-extern-failfast-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-229-runtime-a3-min4-pluginboxmetadata-routeaware-lock-ssot.md
---

# 29cc-230 Runtime A3 Closeout Lock

## Purpose

runtime plugin loader の A3（loader/config boundary）を closeout し、B1 へ進む前に done 判定を SSOT 化する。

## A3 Done Criteria (fixed)

1. `loader/*` の type_id/invoke route は mainline fail-fast（compat fallback は `NYASH_FAIL_FAST=0` 限定）。
2. `types.rs` の handle lifecycle（drop/finalize/clone）は `invoke_alloc_with_route` 経由で BoxInvoke 優先。
3. `globals.rs` は lock 失敗で panic せず `BidError::PluginError` を返す。
4. `extern_functions.rs` は unknown interface/method を `BidError::PluginError` で fail-fast reject。
5. `PluginBoxMetadata` は shim関数ポインタ依存を持たず `invoke_box_fn` で route-aware。
6. `enabled/stub` の公開型は同期される。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. `B1-min1`（`crates/nyash_kernel/src/plugin/invoke_core.rs` + `crates/nyash_kernel/src/plugin/birth.rs`）
2. target: kernel 側 birth/invoke の mainline route cutover（legacy fallback 非依存）

## Not in this lock

1. source 削除（no-delete-first 維持）
2. B1-min2 以降（`runtime_data.rs` / `semantics.rs` / `instance.rs`）
