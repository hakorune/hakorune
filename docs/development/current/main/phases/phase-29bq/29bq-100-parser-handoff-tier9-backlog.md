---
Status: Active
Scope: Phase 29bq parser handoff Tier-9（Box member + local/fini + cleanup + blockexpr + compare/logic 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-99-parser-handoff-tier8-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-9 Backlog (Box member + local/fini + cleanup + blockexpr + compare/logic)

目的:
- Tier-8 の member+lifetime+blockexpr 交差に、比較/論理式（`==/!=/</>/&&/||`）を重ねて 1件ずつ固定する。
- parser handoff を受理確認で終わらせず、JSON v0 bridge 経由で compare/logic の評価まで崩れないことを gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`loop/if + local/fini + blockexpr + cleanup + compare/logic` と Box member の交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-9 member+lifetime+blockexpr+compare/logic fixtures）: 1件

## Remaining Backlog (Tier-9 candidate pack)

### P0: minimal compare/logic cross（最初に固める）

- [x] `T9-BOX-MEMBER-LOOP-IF-BREAK-CONTINUE-LOCAL-FINI-BLOCKEXPR-COMPARE-LOGIC-CLEANUP`:
  - Box method call（`counter.step(...)`） + `local ... fini {}` + blockexpr (`{ ... }`) + compare/logic (`==/!=/</>/&&/||`) + postfix `cleanup`
  - candidate fixture: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
