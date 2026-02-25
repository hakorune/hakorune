---
Status: Active
Scope: Phase 29bq parser handoff Tier-2（式網羅）の在庫表 SSOT。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md
  - CURRENT_TASK.md
---

# Phase 29bq — Parser Handoff Tier-2 Backlog

目的:
- Tier-2 を「1件ずつ PROBE→FIX→PROMOTE」する運用は維持しつつ、残り在庫を先に固定する。
- どのカテゴリが未着手かを可視化し、探索コストを下げる。

運用ルール（固定）:
- 1コミット=1 fixture=1 PROMOTE（複数件まとめない）。
- この在庫表は planning のみ。実行証跡は `CURRENT_TASK.md` / `29bq-92` に残す。
- 受理境界を広げる変更（BoxCount）と、構造整理（BoxShape）は同コミットに混ぜない。

## Coverage Snapshot (2026-02-07)

Done（Tier-2 local-expr fixtures）: 37件
- compare/basic, logic, unary, call/new, array/map, string chain/subcmp, null-logic,
  logic precedence/parenthesized, double-not, compare-chain, compare-or, compare-mixed-logic,
  null-compare-and/null-compare-or/null-compare-mix/null-not/null-parenthesized,
  arith-compare-and/arith-compare-or, local-expr-compare-rel-mixed-logic, local-expr-unary-compare-mixed, local-expr-double-not-mixed-logic, local-expr-parenthesized-compare-mixed, local-expr-not-parenthesized-compare, local-expr-call-compare-and, local-expr-call-compare-or, local-expr-new-compare-mixed, local-expr-string-compare-logic, local-expr-array-len-compare, local-expr-map-value-compare, local-expr-literal-bool-mixed, local-expr-blockexpr-compare まで完了。

## Remaining Backlog (Tier-2 candidate pack)

### P0: null + logic matrix（先に固める）

- [x] `T2-NULL-AND`: `local + (== null) + (!= null) + &&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_null_compare_and_cleanup_min.hako`
- [x] `T2-NULL-OR`: `local + (== null) + (!= null) + ||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_null_compare_or_cleanup_min.hako`
- [x] `T2-NULL-MIX`: `local + (== null/!= null) + && + ||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_null_compare_mixed_logic_cleanup_min.hako`
- [x] `T2-NULL-NOT`: `local + !(== null) + &&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_null_not_compare_cleanup_min.hako`
- [x] `T2-NULL-PAREN`: `local + parenthesized null-logic`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_null_parenthesized_logic_cleanup_min.hako`

### P1: operator cross（算術/比較/論理の交差）

- [x] `T2-ARITH-CMP-AND`: `(+,-,*,/) + (==/!=) + &&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_arith_compare_and_cleanup_min.hako`
- [x] `T2-ARITH-CMP-OR`: `(+,-,*,/) + (==/!=) + ||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_arith_compare_or_cleanup_min.hako`
- [x] `T2-CMP-REL-MIX`: `(==/!=/< />) + && + ||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_compare_rel_mixed_logic_cleanup_min.hako`
- [x] `T2-UNARY-CMP-MIX`: `! + unary - + 比較 + &&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_unary_compare_mixed_cleanup_min.hako`
- [x] `T2-DOUBLE-NOT-MIX`: `!! + (==/!=) + ||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_double_not_mixed_logic_cleanup_min.hako`
- [x] `T2-PAREN-REL-MIX`: parenthesized `((cmp && cmp) || cmp)`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_parenthesized_compare_mixed_cleanup_min.hako`
- [x] `T2-NOT-PAREN-REL`: `!(cmp || cmp) && cmp`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_not_parenthesized_compare_cleanup_min.hako`

### P2: call/literal cross（呼び出し/リテラルとの交差）

- [x] `T2-CALL-CMP-AND`: call return 値 + 比較 + `&&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_call_compare_and_cleanup_min.hako`
- [x] `T2-CALL-CMP-OR`: call return 値 + 比較 + `||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_call_compare_or_cleanup_min.hako`
- [x] `T2-NEW-CMP-MIX`: `new` object method return + 比較 + &&/||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_new_compare_mixed_cleanup_min.hako`
- [x] `T2-STRING-CMP-LOGIC`: string method chain + 比較 + &&/||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_string_compare_logic_cleanup_min.hako`
- [x] `T2-ARRAY-LEN-CMP`: array length compare + &&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_array_len_compare_cleanup_min.hako`
- [x] `T2-MAP-VALUE-CMP`: map lookup-ish path + 比較 + ||`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_map_value_compare_cleanup_min.hako`
- [x] `T2-LITERAL-BOOL-MIX`: bool literal + 比較 + mixed logic
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_literal_bool_mixed_cleanup_min.hako`
- [x] `T2-BLOCKEXPR-CMP`: blockexpr return + 比較 + &&`
  - candidate fixture: `apps/tests/phase29bq_selfhost_local_expr_blockexpr_compare_cleanup_min.hako`

Total remaining candidate pack: 0件

## Next Selection Rule

毎回この順で選ぶ:
1. P0 から未着手を 1件選ぶ。
2. P0 完了後に P1、最後に P2。
3. 選んだ1件だけ `CURRENT_TASK.md` の `next` に転記して実行する。
