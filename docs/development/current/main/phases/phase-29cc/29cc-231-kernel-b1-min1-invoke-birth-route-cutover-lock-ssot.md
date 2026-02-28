---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B1-min1 として、kernel plugin の invoke_core/birth を fail-fast mainline route へ固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-230-runtime-a3-closeout-lock-ssot.md
  - crates/nyash_kernel/src/plugin/invoke_core.rs
  - crates/nyash_kernel/src/plugin/birth.rs
---

# 29cc-231 Kernel B1-min1 Invoke/Birth Route Cutover Lock

## Purpose

kernel plugin 側の birth/invoke コアで残る shim/legacy 依存を mainline fail-fast 契約に寄せ、A3 closeout 後の次境界を固定する。

## Fixed Contract

1. `invoke_core` の Plugin handle decode は `metadata_for_type_id` を使い、mainline fail-fast 既定で unresolved route を拒否する。
2. `birth` は `metadata_for_type_id` の route 情報を前提にし、`NYASH_FAIL_FAST=1` では route 未解決時に即停止する。
3. 呼び出し経路は `nyash_plugin_invoke_v2_shim` を統一入口とし、route 実体は metadata 側（`invoke_box_fn`）で判定する。
4. compat fallback は `NYASH_FAIL_FAST=0` のみ許可する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. `runtime_data.rs` / `semantics.rs` / `instance.rs`（B1-min2+）
2. `future.rs` / `invoke.rs` の本格 route cutover（B3 line）
   - ただし A3-min4 型同期に伴う compile-safe 追従（`invoke_fn` → shim+`invoke_box_fn` 判定）は許可する
3. source 削除（no-delete-first 維持）
