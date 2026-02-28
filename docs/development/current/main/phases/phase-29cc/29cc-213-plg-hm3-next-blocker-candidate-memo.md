---
Status: Proposed
Decision: provisional
Date: 2026-02-28
Scope: PLG-HM3 候補を docs-first で固定し、HM2 closeout 後は monitor-only のまま次ブロッカーを選べる状態にする。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cc/README.md
  - docs/development/current/main/phases/phase-29cc/29cc-212-plg-hm2-min3-route-policy-matrix-lock-ssot.md
---

# 29cc-213 PLG-HM3 Next Blocker Candidate Memo

## Purpose
HM2 完了後に「すぐ実装に飛ぶ」ことを避け、次ブロッカーを docs-first で固定する。

## Current State

1. plugin lane は `active next: none`（monitor-only）。
2. HM2-min1/min2/min3 の lock は完了済み。
3. reopen は failure-driven のみ。

## HM3 Candidate Set (not active)

1. Candidate-A: plugin route telemetry lock
   - 目的: `exec_mode x factory_policy` の実行時観測タグを固定し、drift を早期検出する。
   - 受け入れ基準: telemetry guard + plugin-module-core8 gate 緑。
2. Candidate-B: plugin fallback shrink lock
   - 目的: monitor-only 経路の到達面積を計測し、不要 fallback を縮退する。
   - 受け入れ基準: shrink inventory doc + fail-fast guard 緑。
3. Candidate-C: plugin route docs/runtime parity lock
   - 目的: docs 上の route matrix と runtime 実装の差分を自動検査する。
   - 受け入れ基準: parity guard 緑、差分ゼロ。

## Promotion Rule

1. HM3 を active にする場合は、候補を 1 件だけ選ぶ。
2. 1 blocker = 1 lock = 1 guard = 1 commit の原則で昇格する。
3. `CURRENT_TASK.md` と `phase-29cc/README.md` を同コミットで同期する。
