---
Status: Active
Scope: Phase 29bq parser handoff Tier-10（Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-100-parser-handoff-tier9-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-10 Backlog (Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call)

目的:
- Tier-9 の member+lifetime+blockexpr+compare/logic 交差に、unary（`!`/unary `-`）と method call 条件式を重ねて 1件ずつ固定する。
- parser handoff を受理確認で終わらせず、JSON v0 bridge 経由で unary/call を含む条件式でも安定して評価されることを gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`loop/if + local/fini + blockexpr + cleanup + compare/logic + unary/call` と Box member の交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-10 member+lifetime+blockexpr+compare/logic+unary/call fixtures）: 1件

## Remaining Backlog (Tier-10 candidate pack)

### P0: minimal unary/call cross（最初に固める）

- [x] `T10-BOX-MEMBER-LOOP-IF-BREAK-CONTINUE-LOCAL-FINI-BLOCKEXPR-COMPARE-LOGIC-UNARY-CALL-CLEANUP`:
  - Box method call（`counter.step(...)`） + `local ... fini {}` + blockexpr (`{ ... }`) + compare/logic + unary/call 条件式 + postfix `cleanup`
  - candidate fixture: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
