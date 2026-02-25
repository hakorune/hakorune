---
Status: Active
Scope: Phase 29bq parser handoff Tier-4（制御構造交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-93-parser-handoff-tier2-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-94-parser-handoff-tier3-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-4 Backlog (control-flow cross)

目的:
- Tier-2/3 の式交差を土台に、`if`/`loop` の交差形を 1件ずつ固定する。
- 受理拡張を急がず、`PROBE→PROMOTE` の最小ループを維持する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`catch/cleanup` と DropScope で検証する。

## Coverage Snapshot (2026-02-08)

Done（Tier-4 control fixtures）: 1件

## Remaining Backlog (Tier-4 candidate pack)

### P0: loop + if basic cross（最初に固める）

- [x] `T4-LOOP-IF-BREAK-CONTINUE-CLEANUP`: `loop(cond)` 内 `if` + `break/continue` + postfix `cleanup`
  - candidate fixture: `apps/tests/phase29bq_selfhost_control_loop_if_break_continue_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
