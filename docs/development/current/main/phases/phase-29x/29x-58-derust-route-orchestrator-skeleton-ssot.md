---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X32 `.hako` route orchestrator skeleton の導入と dual-run 契約固定。
Related:
  - docs/development/current/main/phases/phase-29x/29x-57-thin-rust-gate-pack-ssot.md
  - src/runner/route_orchestrator.rs
  - lang/src/vm/route_orchestrator_skeleton.hako
  - tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh
---

# Phase 29x X32: De-Rust Route Orchestrator Skeleton SSOT

## 0. Goal

Rust `route_orchestrator` が持つ VM lane 選択規則を `.hako` 側に最小スケルトンとして用意し、
両者の選択結果が一致することを dual-run smoke で固定する。

## 1. Skeleton Contract

`lang/src/vm/route_orchestrator_skeleton.hako` は次の最小責務だけを持つ:

1. route 選択規則を返す（実行しない）
2. stable tag を出す: `[derust-route/select] backend=<...> lane=<...> source=hako-skeleton`
3. 未知 backend は `lane=unknown` + `Main.main() -> rc=1` で fail-fast

規則の優先順位:

1. `backend=vm`: `force_fallback=1` なら `vm-compat-fallback`
2. `backend=vm`: それ以外で `prefer_vm_hako=1` なら `vm-hako-reference`
3. `backend=vm`: それ以外は `rust-vm-keep`
4. `backend=vm-hako`: 常に `vm-hako-reference`

## 2. Dual-Run Smoke

Canonical command:

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh`

この smoke は 4 ケースで Rust と `.hako` の lane を比較する:

1. `vm-default`
2. `vm-prefer-hako`
3. `vm-fallback-priority`
4. `vm-hako-explicit`

## 3. Acceptance

1. Rust `[vm-route/select]` と `.hako` `[derust-route/select]` の lane が 4 ケースすべて一致する
2. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X32 完了状態に同期される
3. 次タスク `X33`（`.hako` verifier 経路導入）へ進める

## 4. Evidence (X32)

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_route_dualrun_vm.sh`

## 5. Next Step

X33 で `.hako` verifier 経路を導入し、契約不一致時の fail-fast を `.hako` 側へ移管する。
