---
Status: Active
Scope: Phase 29bq parser handoff Tier-3（`local + fini` 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-93-parser-handoff-tier2-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-3 Backlog (`local + fini`)

目的:
- Tier-2 で固めた式網羅を、`local ... fini {}` と交差させて 1件ずつ固定する。
- `fini` の宣言スロット捕捉/LIFO を壊さずに、式パーサ handoff の網羅を拡張する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `fini` 本文で非ローカル脱出（`return`/`throw`/`break`/`continue`）は扱わない（既存 contract を維持）。

## Coverage Snapshot (2026-02-08)

Done（Tier-3 local+fini fixtures）: 5件
- local-expr-compare-fini-cleanup, local-expr-call-fini-cleanup, local-expr-blockexpr-fini-cleanup, local-fini-multi-lifo-cleanup, local-expr-null-fini-cleanup まで完了。

## Remaining Backlog (Tier-3 candidate pack)

### P0: expression init + fini capture（最初に固める）

- [x] `T3-LF-EXPR-CMP`: `local expr init + compare + fini + cleanup`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_compare_fini_cleanup_min.hako`
- [x] `T3-LF-EXPR-CALL`: `local call/new init + fini`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_call_fini_cleanup_min.hako`
- [x] `T3-LF-EXPR-BLOCK`: `local blockexpr init + fini`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_blockexpr_fini_cleanup_min.hako`

### P1: interaction cross（fini/cleanup/LIFO）

- [x] `T3-LF-MULTI-LIFO`: multi local-fini + reassignment + cleanup
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_fini_multi_lifo_cleanup_min.hako`
- [x] `T3-LF-NULL-LOGIC`: null compare/logic init + fini
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_null_fini_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. P0 完了後に P1 へ進む。
3. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
