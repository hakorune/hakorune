---
Status: Active
Decision: accepted
Date: 2026-02-13
Scope: Phase 29x X39 Rust lane を tools/compat 専用へ隔離し、明示 opt-in のみ許可する。
Related:
  - docs/development/current/main/phases/phase-29x/29x-64-llvm-only-daily-default-ssot.md
  - tools/compat/README.md
  - tools/compat/phase29x_rust_lane_gate.sh
  - tools/smokes/v2/profiles/integration/apps/archive/phase29x_rust_lane_optin_only.sh
---

# Phase 29x X39: Rust Lane Opt-In Isolation SSOT

## 0. Goal

通常運用（daily/milestone）から Rust lane を分離し、
`tools/compat` 配下の明示コマンドでのみ実行可能にする。

## 1. Contract

1. daily/milestone 既定は `phase29x_llvm_only_daily_gate.sh`
2. Rust lane 入口は `tools/compat/phase29x_rust_lane_gate.sh` のみ
3. Rust lane 実行には `PHASE29X_ALLOW_RUST_LANE=1` を必須化
4. opt-in なし実行は `[compat/optin-required]` で fail-fast

## 2. Acceptance

1. `phase29x_rust_lane_optin_only.sh` が PASS
2. `tools/compat/README.md` に入口/原則が記載されている
3. `README / 29x-90 / 29x-91 / CURRENT_TASK` が X39 完了状態に同期
4. 次タスク `X40`（llvm-only build done 同期）へ進める

## 3. Evidence (X39)

1. `bash tools/smokes/v2/profiles/integration/apps/archive/phase29x_rust_lane_optin_only.sh`

## 4. Next Step

X40 で llvm-only build done 判定、
rollback 条件、残存 Rust 依存一覧を docs に最終同期する。
