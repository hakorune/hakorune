---
Status: Active
Scope: docs-only（Phase 29an の closeout）
Related:
- docs/development/current/main/phases/phase-29an/README.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
---

# Phase 29an P15: Closeout（P0–P14まとめ・次フェーズ入口固定）

Date: 2025-12-29  
Status: Ready for execution  
Scope: docs-only（コード変更なし）

## Objective

- Phase 29an（Skeleton/Feature Facts）の成果と SSOT を 1枚にまとめて “完了” と言える状態にする
- 次フェーズ（CorePlan composition）への入口を固定し、Now/Backlog/CURRENT_TASK を迷子にしない

## Deliverables（docs-only）

### Step 1: phase-29an README を closeout 形式へ

Update:
- `docs/development/current/main/phases/phase-29an/README.md`

Add:
- P0–P14 の完了一覧（commit hash）
- “何が SSOT として揃ったか” の短いサマリー
  - SkeletonFacts / FeatureFacts（ExitUsage/ExitMap/Cleanup/ValueJoin）
  - Canonical projections（skeleton_kind/exit_usage/exit_kinds_present/cleanup_kinds_present/value_join_needed）
  - debug-only invariants（exit_usage↔plan / exit_usage↔exitmap / cleanup↔exitkind）
- Next phase link（Phase 29ao）

### Step 2: Now/Backlog/CURRENT_TASK の更新

Update:
- `docs/development/current/main/10-Now.md`
  - Current Focus を Phase 29ao に更新
  - Phase 29an P14/P15 の完了記録を追記
- `docs/development/current/main/30-Backlog.md`
  - Phase 29an を ✅ COMPLETE に更新（現状は “P0 Ready” 表記が残っているので修正）
  - Phase 29ao を active/candidate として追加
- `CURRENT_TASK.md`
  - Next implementation を Phase 29ao P0 へ差し替え

### Step 3: Commit（docs-only）

- `git add -A`
- `git commit -m "docs(phase29an): closeout p0-p14; handoff to phase29ao"`

## Verification（optional, docs-only）

- `./tools/smokes/v2/run.sh --profile quick`

