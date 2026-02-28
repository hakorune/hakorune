---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の B1-min1 closeout として、invoke_core/birth の route cutover 完了条件を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-231-kernel-b1-min1-invoke-birth-route-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - crates/nyash_kernel/src/plugin/invoke_core.rs
  - crates/nyash_kernel/src/plugin/birth.rs
---

# 29cc-232 Kernel B1-min1 Closeout Lock

## Purpose

B1-min1 で導入した invoke/birth route 切替を closeout し、B1-min2 へ進む前提契約を SSOT 化する。

## B1-min1 Done Criteria (fixed)

1. `invoke_core` は plugin handle decode 時に `resolve_invoke_route_for_type` を通して route を決定する。
2. `birth` は metadata route を前提とし、`NYASH_FAIL_FAST=1` で route 未解決時に即停止する。
3. invoke の統一入口は `nyash_plugin_invoke_v2_shim` とし、route 実体判定は metadata（`invoke_box_fn`）で行う。
4. A3-min4 型同期の compile-safe 追従（`invoke.rs`/`future.rs`）は許可済みとする。

## Acceptance

1. `cargo check --bin hakorune` green
2. `cargo check -p nyash_kernel` green
3. `tools/checks/dev_gate.sh runtime-exec-zero` green
4. `phase29y_no_compat_mainline_vm.sh` green

## Next Boundary (fixed)

1. `B1-min2`: `runtime_data.rs` / `semantics.rs` / `instance.rs`
2. target: runtime state semantics の fail-fast route contract を固定（挙動不変）
