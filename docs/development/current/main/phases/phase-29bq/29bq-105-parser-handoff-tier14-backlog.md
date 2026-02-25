---
Status: Active
Scope: Phase 29bq parser handoff Tier-14（Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail 交差）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-104-parser-handoff-tier13-backlog.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-14 Backlog (Box member + local/fini + cleanup + blockexpr + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail)

目的:
- Tier-13 の member+lifetime+blockexpr+compare/logic+unary/call+literal+nested-tail+nested-loop-branch 交差に、method-chain tail（tail 式が method chain を返す形）を重ねて 1件ずつ固定する。
- parser handoff を受理確認で終わらせず、JSON v0 bridge 経由で method-chain tail 形が崩れないことを gate で確認する。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- 実行証跡は `CURRENT_TASK.md` に残し、本書は在庫表だけを持つ。
- `try` は使わない。`loop/if + local/fini + blockexpr + cleanup + compare/logic + unary/call + null/array-map literal + nested if/else tail + nested loop/branch mix + method-chain tail` と Box member の交差だけを観測する。

## Coverage Snapshot (2026-02-08)

Done（Tier-14 member+lifetime+blockexpr+compare/logic+unary/call+literal+nested tail+nested loop/branch+method-chain tail fixtures）: 1件

## Remaining Backlog (Tier-14 candidate pack)

### P0: minimal method-chain-tail cross（最初に固める）

- [x] `T14-BOX-MEMBER-LOOP-IF-BREAK-CONTINUE-LOCAL-FINI-BLOCKEXPR-COMPARE-LOGIC-UNARY-CALL-LITERALS-NESTEDTAIL-NESTEDLOOPBRANCH-METHODCHAINTAIL-CLEANUP`:
  - Box method call（`counter.step(...)`） + `local ... fini {}` + blockexpr (`{ ... }`) + compare/logic + unary/call + `null` + `[]` + `%{}` + nested if/else tail + nested loop/branch mix + method-chain tail + postfix `cleanup`
  - candidate fixture: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. 選んだ 1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
