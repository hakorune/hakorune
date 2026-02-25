---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X33 `.hako` verifier 経路の最小導入と mismatch fail-fast 契約固定。
Related:
  - docs/development/current/main/phases/phase-29x/29x-58-derust-route-orchestrator-skeleton-ssot.md
  - src/runner/modes/common_util/verifier_gate.rs
  - lang/src/vm/verifier_gate_skeleton.hako
  - tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh
---

# Phase 29x X33: De-Rust Verifier Path SSOT

## 0. Goal

Rust verifier gate の結果を `.hako` 側でも受け取り可能な最小経路を追加し、
Rust/Hako の判定不一致を fail-fast で停止させる契約を固定する。

## 1. Skeleton Contract

`lang/src/vm/verifier_gate_skeleton.hako` は次の最小責務だけを持つ:

1. Rust verifier error count と `.hako` verifier error count を比較する
2. 不一致時は `rc=1` と freeze tag を返す
3. 一致時は `rc=0` と check tag を返す

安定タグ:

1. mismatch: `[freeze:contract][derust-verifier/mismatch] route=<...> function=<...> rust_errors=<...> hako_errors=<...>`
2. match: `[derust-verifier/check] route=<...> function=<...> errors=<...> source=hako-skeleton`

## 2. Smoke Contract

Canonical command:

1. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh`

この smoke は 2 ケースで契約を固定する:

1. mismatch (`rust_errors != hako_errors`) は `rc=1` + freeze tag
2. match (`rust_errors == hako_errors`) は `rc=0` + check tag

## 3. Acceptance

1. verifier 不一致時に fail-fast（`rc=1`）で停止する
2. mismatch/match の安定タグ語彙が smoke で固定される
3. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X33 完了状態に同期される
4. 次タスク `X34`（`.hako` safety 経路導入）へ進める

## 4. Evidence (X33)

1. `cargo check -q --bin hakorune`
2. `bash tools/smokes/v2/profiles/integration/apps/phase29x_derust_verifier_vm.sh`

## 5. Next Step

X34 で `.hako` safety 経路を導入し、lifecycle 境界の fail-fast を `.hako` 側へ移管する。
