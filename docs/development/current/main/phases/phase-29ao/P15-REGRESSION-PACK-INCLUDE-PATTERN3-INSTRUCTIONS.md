---
Status: Ready
Scope: docs+scripts（回帰ゲートの強化、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
---

# Phase 29ao P15: JoinIR 回帰パックに Pattern3(If‑Phi, VM) を追加

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変。回帰ゲート（SSOT）を “P13 の実経路” を含む形に強化する。

## 目的

- P13 で移行した `Pattern3 If‑Phi`（merge join の block_params 化）が、JoinIR 回帰 SSOT のゲートで必ず実行されるようにする。
- 既存の回帰パック設計（VM backend / filter で小さく）を維持する。

## 非目的

- LLVM EXE 系テストを回帰ゲートに含める（時間/環境依存が大きいので対象外）
- 新しい fixture/smoke の新設（既存 `phase118_pattern3_if_sum_vm` を流用）
- 新 env var 追加

## 実装

### Step 1: regression pack に Pattern3 VM を追加

- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
  - 追加: `run_filter "pattern3_ifphi_vm" "phase118_pattern3_if_sum_vm"`
  - 位置: pattern2 の後 / pattern6 の前（順序は SSOT として固定）

### Step 2: SSOT ドキュメントに追記

- `docs/development/current/main/phases/phase-29ae/README.md`
  - Regression pack に `phase118_pattern3_if_sum_vm` を追記

### Step 3: Phase 29ao の進捗と Next を更新

- `docs/development/current/main/phases/phase-29ao/README.md`
- `docs/development/current/main/10-Now.md`
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## 検証（必須）

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`
- `./tools/smokes/v2/run.sh --profile quick`

## コミット

- `git add -A`
- `git commit -m "docs(phase29ao): add pattern3 vm to joinir regression pack"`

