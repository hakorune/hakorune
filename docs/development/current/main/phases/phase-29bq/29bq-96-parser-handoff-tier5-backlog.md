---
Status: Active
Scope: Phase 29bq parser handoff Tier-5（control-flow + local/fini 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-95-parser-handoff-tier4-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-5 Backlog (control-flow + local/fini)

目的:
- Tier-4 の制御構造交差に、`local ... fini {}` の寿命契約を重ねて 1件ずつ固定する。
- parser handoff を「構文受理だけ」で終わらせず、DropScope/LIFO を含めて gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`cleanup` と `fini` の交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-5 control + local/fini fixtures）: 1件

## Remaining Backlog (Tier-5 candidate pack)

### P0: minimal loop cross（最初に固める）

- [x] `T5-LOOP-IF-BREAK-CONTINUE-LOCAL-FINI-CLEANUP`:
  - `loop(cond)` + `if` + `break/continue` に `local ... fini {}` と postfix `cleanup` を重ねる
  - candidate fixture: `apps/tests/phase29bq_selfhost_control_loop_local_fini_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
