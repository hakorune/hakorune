---
Status: Active
Scope: Phase 29bq parser handoff Tier-8（Box member + local/fini + cleanup + blockexpr 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-98-parser-handoff-tier7-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-8 Backlog (Box member + local/fini + cleanup + blockexpr)

目的:
- Tier-7 の member+lifetime 交差に、`{ ... }` blockexpr を重ねて 1件ずつ固定する。
- parser handoff を「受理のみ」で終わらせず、blockexpr が `local ... fini {}` と postfix `cleanup` に同居しても崩れないことを gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`loop/if + local/fini + blockexpr + cleanup` と Box member の交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-8 member+lifetime+blockexpr fixtures）: 1件

## Remaining Backlog (Tier-8 candidate pack)

### P0: minimal cross with blockexpr（最初に固める）

- [x] `T8-BOX-MEMBER-LOOP-IF-BREAK-CONTINUE-LOCAL-FINI-BLOCKEXPR-CLEANUP`:
  - Box method call（`counter.step(...)`） + `local ... fini {}` + blockexpr (`{ ... }`) + postfix `cleanup` を同一fixtureで交差
  - candidate fixture: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
