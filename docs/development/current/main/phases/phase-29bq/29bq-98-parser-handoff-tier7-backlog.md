---
Status: Active
Scope: Phase 29bq parser handoff Tier-7（Box member + local/fini + cleanup 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-97-parser-handoff-tier6-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-7 Backlog (Box member + local/fini + cleanup)

目的:
- Tier-6 の Box member + control-flow 交差に、`local ... fini {}` を重ねて 1件ずつ固定する。
- parser handoff を構文受理だけで止めず、DropScope と Box method 呼び出しが同時に載る形を gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`loop/if + local/fini + cleanup` と Box member の交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-7 box member + local/fini fixtures）: 1件

## Remaining Backlog (Tier-7 candidate pack)

### P0: minimal member+lifetime cross（最初に固める）

- [x] `T7-BOX-MEMBER-LOOP-IF-BREAK-CONTINUE-LOCAL-FINI-CLEANUP`:
  - Box method call（`counter.step(...)` / `counter.cleanup_bonus()`）を `loop(cond)` + `if` + `break/continue` + `local ... fini {}` + postfix `cleanup` と交差
  - candidate fixture: `apps/tests/phase29bq_selfhost_box_member_local_fini_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
