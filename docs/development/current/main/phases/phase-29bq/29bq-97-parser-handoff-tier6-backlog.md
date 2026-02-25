---
Status: Active
Scope: Phase 29bq parser handoff Tier-6（Box member + control-flow/cleanup 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-96-parser-handoff-tier5-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-6 Backlog (Box member + control-flow/cleanup)

目的:
- Tier-5 の `control-flow + local/fini` 交差の次段として、Box member（method/field）アクセスを 1件ずつ固定する。
- parser handoff を式/文の受理で止めず、`me.*` / member access が DropScope cleanup と同居しても崩れないことを gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`loop/if + cleanup` と Box member 交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-6 box member fixtures）: 1件

## Remaining Backlog (Tier-6 candidate pack)

### P0: minimal member cross（最初に固める）

- [x] `T6-BOX-MEMBER-LOOP-IF-BREAK-CONTINUE-CLEANUP`:
  - Box method call（`counter.step(...)` / `counter.cleanup_bonus()`）を `loop(cond)` + `if` + `break/continue` + postfix `cleanup` と交差
  - candidate fixture: `apps/tests/phase29bq_selfhost_box_member_loop_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
