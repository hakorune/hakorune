---
Status: Active
Scope: `.hako` parser handoff を「mirbuilder安定化後」に最短で進める運用チェックリスト。
Related:
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-93-parser-handoff-tier2-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-94-parser-handoff-tier3-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-95-parser-handoff-tier4-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-96-parser-handoff-tier5-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-97-parser-handoff-tier6-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-98-parser-handoff-tier7-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-99-parser-handoff-tier8-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-100-parser-handoff-tier9-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-101-parser-handoff-tier10-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-102-parser-handoff-tier11-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-103-parser-handoff-tier12-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-104-parser-handoff-tier13-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-105-parser-handoff-tier14-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-106-parser-handoff-tier15-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-107-parser-handoff-tier16-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-108-parser-handoff-tier17-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-110-parser-handoff-tier18-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-112-parser-handoff-tier19-backlog.md
  - docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
  - docs/development/current/main/design/selfhost-language-v1-freeze-ssot.md
  - docs/reference/language/README.md
---

# Phase 29bq — Parser Handoff Checklist (SSOT)

目的: `mirbuilder` 側の土台を壊さず、`.hako parser` の移植を **1ブロッカーずつ** 前進させる。

## 0.x 開始時点スナップショット（2026-02-06）

- [x] `phase29bq*selfhost*parse*.hako` 系の既存 blocker は subset へ PROMOTE 済み（remaining=0）。
- [x] 次の parser handoff は「新規 fixture を 1本作って PROBE」から開始する（`try` なし形を先に受理してから）。
  - evidence: `apps/tests/phase29bq_selfhost_cleanup_only_min.hako`

## 0) 前提（着手条件）

- [x] `29bq-91` の Route readiness が緑（release build / fast gate / selfhost canary）。
- [x] `29bq-91` の JSON v0 bridge coverage が緑（Known gap を含む）。
- [x] language 仕様変更を同時に混ぜない（必要なら docs/reference 側で Decision を先に確定）。

## 1) 日常コマンド（parser handoff）

- [x] quick baseline: `cargo check --bin hakorune`
- [x] fast gate: `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- [x] parser probe(1件): `tools/selfhost/run.sh --gate --planner-required 1 --filter <parser_case_substring> --max-cases 1`
  - evidence: `tools/selfhost/run.sh --gate --planner-required 1 --filter parse_local_fini_no_init_min --max-cases 1` PASS
- [x] legacy literal readiness flow（撤去判断前）:
  - unified helper: `bash tools/selfhost/legacy_main_readiness.sh`
  - strict gate: `bash tools/selfhost/legacy_main_readiness.sh --strict`
- [x] parser/mirbuilder の `.hako` 修正を含むコミットでは、internal-only emit を先に通す: `bash tools/hakorune_emit_mir_mainline.sh apps/tests/phase29bq_selfhost_cleanup_only_min.hako /tmp/phase29bq_cleanup_only_internal.mir.json`（latest evidence: 2026-02-25 PASS）

### 1.1 subset intentional exclusions（2026-02-24 snapshot）

- `planner_required_selfhost_subset.tsv` からは次の 5 fixture を intentional に除外する。
  - funcscanner 3件（専用 smoke で監視）:
    - `apps/tests/phase29bq_selfhost_funcscanner_box_from_min.hako`
    - `apps/tests/phase29bq_selfhost_funcscanner_method_boundary_from_birth_min.hako`
    - `apps/tests/phase29bq_selfhost_funcscanner_lambda_literal_min.hako`
  - throw 2件（surface 言語契約で parser fail-fast）:
    - `apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako`
    - `apps/tests/phase29bq_selfhost_try_loop_throw_catch_min.hako`
- 監視コマンド:
  - `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_box_from_min_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_funcscanner_method_boundary_min_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/selfhost/phase29bq_selfhost_stageb_lambda_literal_pair_min_vm.sh`
  - `./target/release/hakorune --backend vm apps/tests/phase29bq_selfhost_try_throw_catch_cleanup_min.hako`（`[freeze:contract][parser/throw_reserved]`）
  - `./target/release/hakorune --backend vm apps/tests/phase29bq_selfhost_try_loop_throw_catch_min.hako`（`[freeze:contract][parser/throw_reserved]`）

## 2) 進め方（PROBE→FIX→PROMOTE）

### 2.1 PROBE（ローカル）

- [x] parser 形を 1件だけ選び、`/tmp/selfhost_probe.tsv` で局所実行する（コミットしない）。
- [x] Tier-2 は `29bq-93` の在庫表から未着手を 1件だけ選ぶ（P0→P1→P2 の順）。
- [x] Tier-3 は `29bq-94` の在庫表から未着手を 1件だけ選ぶ（P0→P1 の順）。
- [x] Tier-4 は `29bq-95` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-5 は `29bq-96` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-6 は `29bq-97` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-7 は `29bq-98` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-8 は `29bq-99` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-9 は `29bq-100` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-10 は `29bq-101` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-11 は `29bq-102` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-12 は `29bq-103` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-13 は `29bq-104` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-14 は `29bq-105` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-15 は `29bq-106` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-16 は `29bq-107` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-17 は `29bq-108` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-18 は `29bq-110` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] Tier-19 は `29bq-112` の在庫表から未着手を 1件だけ選ぶ（P0 の順）。
- [x] PASS なら PROMOTE へ、FAIL なら FIX へ進む。
- [x] Tier-20以降は failure-driven で追加する（green維持中の先回り追加は禁止）。

### 2.1.1 Tier-20+ fixture 追加条件（failure-driven）

- [x] 新規fixtureは次の3条件のどれかを満たす時だけ追加する:
  - 新しい `first_freeze_or_reject` が出た
  - 既存green fixture が回帰した
  - language/contract SSOT の Decision 変更が入った
- [x] 条件外では `planner_required_selfhost_subset.tsv` を増やさない（quick gate運用を優先）。
- [x] 追加時は 1件だけ `PROBE→PROMOTE` し、複数件バッチ追加をしない。

### 2.2 FIX（1ブロッカー=1コミット）

- [ ] `phase29bq_collect_planner_required_blocker_vm.sh` で `first_freeze_or_reject` を採取する。
- [x] 既知FAIL: block-postfix `cleanup` が `Var("cleanup")` として解釈され `undefined variable: cleanup` になる点を先に修正する。
- [x] parser 層の最小修正で unblock する（bridge/runner のついで変更を混ぜない）。
- [ ] 仕様境界（v1 freeze）外の受理拡張はしない。

### 2.3 PROMOTE（subset更新のみ）

- [x] `planner_required_selfhost_subset.tsv` に 1行だけ追加する（コード変更禁止）。
- [x] `29bq-90` の頻度ルールに従って gate を実行する（quick/probe標準、fullは節目のみ）。
- [x] legacy literals 撤去コミットの pre-promote は機械判定で実施する:
  - `bash tools/selfhost/pre_promote_legacy_main_removal.sh`
- [x] PASS 証拠を `29bq-91` の snapshot に反映する。
  - evidence: `apps/tests/phase29bq_selfhost_blocker_parse_local_fini_no_init_min.hako` を PROMOTE（expected=`BA`）
  - evidence: `apps/tests/phase29bq_selfhost_local_multibind_cleanup_min.hako` を PROMOTE（expected=`13`）
  - evidence: `apps/tests/phase29bq_selfhost_local_multibind_init_cleanup_min.hako` を PROMOTE（expected=`27`）
  - evidence: `apps/tests/phase29bq_selfhost_local_multibind_mixed_init_cleanup_min.hako` を PROMOTE（expected=`36`）
  - evidence: `apps/tests/phase29bq_selfhost_local_triple_noinit_cleanup_min.hako` を PROMOTE（expected=`46`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_compare_cleanup_min.hako` を PROMOTE（expected=`55`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_logic_cleanup_min.hako` を PROMOTE（expected=`100`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_unary_not_cleanup_min.hako` を PROMOTE（expected=`91`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_unary_minus_cleanup_min.hako` を PROMOTE（expected=`78`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_unary_mix_cleanup_min.hako` を PROMOTE（expected=`61`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_call_new_cleanup_min.hako` を PROMOTE（expected=`132`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_array_map_literal_cleanup_min.hako` を PROMOTE（expected=`142`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_string_concat_len_cleanup_min.hako` を PROMOTE（expected=`233`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_string_subcmp_cleanup_min.hako` を PROMOTE（expected=`344`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_null_logic_cleanup_min.hako` を PROMOTE（expected=`455`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_string_trim_chain_cleanup_min.hako` を PROMOTE（expected=`666`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_bool_combo_cleanup_min.hako` を PROMOTE（expected=`703`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_logic_precedence_cleanup_min.hako` を PROMOTE（expected=`803`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_logic_parenthesized_cleanup_min.hako` を PROMOTE（expected=`911`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_double_not_compare_cleanup_min.hako` を PROMOTE（expected=`1003`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_compare_chain_cleanup_min.hako` を PROMOTE（expected=`1113`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_compare_or_cleanup_min.hako` を PROMOTE（expected=`1214`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_compare_mixed_logic_cleanup_min.hako` を PROMOTE（expected=`1315`）
  - evidence: `apps/tests/phase29bq_selfhost_local_expr_null_compare_and_cleanup_min.hako` を PROMOTE（expected=`1416`）
  - evidence: `apps/tests/phase29bq_selfhost_control_loop_if_break_continue_cleanup_min.hako` を PROMOTE（expected=`171`）
  - evidence: `apps/tests/phase29bq_selfhost_control_loop_local_fini_cleanup_min.hako` を PROMOTE（expected=`156`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_loop_cleanup_min.hako` を PROMOTE（expected=`47`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_cleanup_min.hako` を PROMOTE（expected=`29`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_cleanup_min.hako` を PROMOTE（expected=`115`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_cleanup_min.hako` を PROMOTE（expected=`118`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_cleanup_min.hako` を PROMOTE（expected=`113`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_cleanup_min.hako` を PROMOTE（expected=`111`）
  - evidence: `apps/tests/phase29bq_selfhost_box_member_local_fini_blockexpr_compare_logic_unary_call_literals_nested_tail_nested_loop_branch_method_chain_tail_side_effect_tail_nested_join_tail_dual_tail_sync_guard_sync_tail_cleanup_min.hako` を PROMOTE（expected=`111`）

## 3) 受け入れ条件（1ユニット完了）

- [x] parser probe が PASS。
- [x] `phase29bq_fast_gate_vm.sh --only bq` が PASS。
- [x] subset を更新した場合のみ full selfhost canary が PASS。
- [x] commit は「1ブロッカー=1受理形=1コミット」を守る。

## 4) ガードレール

- [ ] AST rewrite 禁止（analysis-only 観測）。
- [ ] fallback で通さない（strict/dev + planner_required で fail-fast）。
- [ ] parser の変更で Stage-B JSON v0 契約を壊さない。
- [ ] BoxCount と BoxShape を同コミットで混ぜない。
- [ ] 新規 fixture は `try` を使わず、postfix `cleanup` / DropScope 構文で検証する。
- [ ] `main(args)` 移行は migration-order SSOT の entry-signature 契約に従う（outer entry先行、payload stringは後段。fixture上の `main(args)` は移行済みで、残件は legacy 検出 literals のみ。保持/撤去は legacy literals decision に従う）。
- [ ] legacy producer 撤去順は SSOT固定（tests-first -> compiler-literals second）。順序を逆転しない。
- [ ] `stage1_cli.hako` internal-only emit が FAIL のまま parser handoff を PROMOTE しない（milestone check は `29bq-90` / `29bq-91` に従う）。
