---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X31 thin-rust gate pack 固定（X24-X30 contract evidence を1コマンド化）。
Related:
  - docs/development/current/main/phases/phase-29x/29x-50-thin-rust-boundary-lock-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-56-thin-rust-core-cabi-min-surface-ssot.md
  - tools/smokes/v2/profiles/integration/apps/archive/phase29x_thin_rust_gate_vm.sh
  - tools/checks/vm_route_bypass_guard.sh
  - tools/checks/vm_verifier_gate_guard.sh
  - tools/checks/vm_safety_gate_guard.sh
  - tools/checks/nyrt_core_cabi_surface_guard.sh
---

# Phase 29x X31: Thin-Rust Gate Pack SSOT

## 0. Goal

X24-X30 の thin-rust 契約を個別実行のまま残さず、
1つの gate pack smoke で再現可能に固定する。

## 1. Gate Pack

Canonical command:

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_thin_rust_gate_vm.sh`

この gate は次を順に実行する:

1. route guard（bypass）
2. verifier guard
3. safety guard
4. Core C ABI surface guard
5. route/verifier/safety/cabi の各 smoke

## 2. Acceptance

1. `phase29x_thin_rust_gate_vm.sh` が PASS する
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X31 完了状態に同期される
3. 次タスク `X32`（`.hako` route orchestrator skeleton）へ進める

## 3. Evidence (X31)

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_thin_rust_gate_vm.sh`

## 4. Next Step

X32 で `.hako` route orchestrator skeleton を導入し、Rust orchestrator との dual-run 検証へ進む。
