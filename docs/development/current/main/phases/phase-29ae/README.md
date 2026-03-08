# Phase 29ae: JoinIR Regression Pack (docs-first)

Goal: JoinIR の最小回帰セットを SSOT として固定する。

## Regression pack (SSOT)

Numbered route labels below are historical labels only. Current entry names are the semantic wrapper stems shown first.
Exact compat/archive stem mapping is tracked in
`joinir-smoke-legacy-stem-retirement-ssot.md`; legacy fixture key/pin details are tracked in
`joinir-legacy-fixture-pin-inventory-ssot.md`.

- loop_break (body-local route, VM): `loop_break_body_local_vm`（coverage-only semantic-body wrapper）
- loop_break (body-local seg route, VM): `loop_break_body_local_seg_vm`（coverage-only semantic-body wrapper）
- loop_break (real-world route, VM): `loop_break_realworld_vm`（regression-pack semantic-body wrapper）
- loop_break (subset, strict shadow, VM): `loop_break_plan_subset_vm`（regression-pack semantic-body wrapper）
- loop_break (release adopt, VM): `loop_break_release_adopt_vm`
- if_phi_join (VM): `if_phi_join_vm`（regression-pack semantic-body wrapper）
- if_phi_join (release adopt, VM): `if_phi_join_release_adopt_vm`
- loop_continue_only (continue min, VM): `loop_continue_only_vm`
- loop_simple_while (strict shadow, VM): `loop_simple_while_strict_shadow_vm`
- loop_simple_while (subset reject, VM): `loop_simple_while_subset_reject_extra_stmt_vm`
- loop_simple_while (stdlib to_lower, VM): `phase29ap_stringutils_tolower_vm`
- loop_simple_while (stdlib join, VM): `phase29ap_stringutils_join_vm`
- scan_with_init (stdlib index_of, VM): `phase29aq_string_index_of_min_vm`
- scan_with_init (stdlib last_index_of, VM): `phase29aq_string_last_index_of_min_vm`
- scan_with_init (stdlib index_of_string, VM): `phase29aq_string_index_of_string_min_vm`
- loop_break (stdlib parse_integer, VM): `phase29aq_string_parse_integer_min_vm`
- loop_break (stdlib parse_integer sign, VM): `phase29aq_string_parse_integer_sign_min_vm`
- loop_break (stdlib parse_integer leading whitespace, VM): `phase29aq_string_parse_integer_ws_min_vm`
- loop_break (stdlib parse_integer leading zero, VM): `phase29aq_string_parse_integer_leading_zero_min_vm`
- split_scan (stdlib split, VM): `phase29aq_string_split_min_vm`
- split_scan (stdlib split char, VM): `phase29aq_string_split_char_min_vm`
- split_scan (stdlib split string, VM): `phase29aq_string_split_string_min_vm`
- loop_break (stdlib trim_start, VM): `phase29aq_string_trim_start_min_vm`
- loop_break (stdlib trim_end, VM): `phase29aq_string_trim_end_min_vm`
- Derived (stdlib contains, VM): `phase29aq_string_contains_min_vm`
- Derived (stdlib starts_with, VM): `phase29aq_string_starts_with_min_vm`
- Derived (stdlib ends_with, VM): `phase29aq_string_ends_with_min_vm`
- Derived (stdlib trim, VM): `phase29aq_string_trim_min_vm`
- loop_simple_while (stdlib to_upper, VM): `phase29aq_string_to_upper_min_vm`
- Purity gate (strict fallback visibility, VM): `phase29as_purity_gate_vm`
- Return-in-loop (stdlib is_integer, strict fail-fast reject, VM): `phase29ar_string_is_integer_min_vm`
- Return-in-loop (stdlib is_integer, release adopt, VM): `phase29ar_string_is_integer_release_adopt_vm`
- Generic loop (continue, strict shadow, VM): `phase29ca_generic_loop_continue_strict_shadow_vm`
- Generic loop (continue, release adopt, VM): `phase29ca_generic_loop_continue_release_adopt_vm`
- Generic loop (in-body step, strict shadow, VM): `phase29cb_generic_loop_in_body_step_strict_shadow_vm`
- Generic loop (in-body step, release adopt, VM): `phase29cb_generic_loop_in_body_step_release_adopt_vm`
- BranchN (match return-only, strict shadow, VM): `phase29at_match_return_strict_shadow_vm`
- BranchN (match return-only, release adopt, VM): `phase29at_match_return_release_adopt_vm`
- FlowBox tags gate (strict/non-strict, VM): `phase29av_flowbox_tags_gate_vm`
- FlowBox tag coverage gate (strict/non-strict, VM): `phase29aw_flowbox_tag_coverage_gate_vm`
- loop_true_early_exit (VM): `loop_true_early_exit_vm`（regression-pack semantic-body wrapper）
- loop_true_early_exit (strict shadow, VM): `loop_true_early_exit_strict_shadow_vm`
- loop_true_early_exit (release adopt, VM): `loop_true_early_exit_release_adopt_vm`
- scan_with_init (strict shadow, VM): `scan_with_init_strict_shadow_vm`
- scan_with_init (release adopt, VM): `scan_with_init_release_adopt_vm`
- scan_with_init supplemental pack (VM): `scan_with_init_regression_pack_vm`
- nested_loop_minimal (release adopt, VM): `nested_loop_minimal_release_adopt_vm`
- nested_loop_minimal (strict shadow, VM): `nested_loop_minimal_strict_shadow_vm`
- split_scan (strict shadow, VM): `split_scan_strict_shadow_vm`
- split_scan (release adopt, VM): `split_scan_release_adopt_vm`
- split_scan supplemental pack (VM): `split_scan_regression_pack_vm`
- この pack が JoinIR 回帰の唯一の integration gate（phase143_* は対象外）
- JoinIR routing is plan/composer SSOT only (legacy loop table removed in Phase 29ap P12)
- phase143_* は LoopBuilder 撤去 / plugin disable 固定 / LLVM exe 期待が古いので除外
- legacy pack stems for the historical accum-const-loop lane are SKIP on the plugins-disabled path and stay outside this regression pack（exact stems are tracked in `joinir-smoke-legacy-stem-retirement-ssot.md`）
- legacy fixture family / key の詳細は `docs/development/current/main/design/joinir-legacy-fixture-pin-inventory-ssot.md` を正本とする
- archive-backed 6本の retire/collapse 判定は `joinir-smoke-legacy-stem-retirement-ssot.md` の `Archived replay forwarder resolution conditions` を正本とし、caller 0 単独では判定しない。
- archive-backed 6本の keep authority / demotion readiness は `joinir-smoke-legacy-stem-retirement-ssot.md` の `Archive-backed six-route keep authority` を正本とする。
- FlowBox schema tag（`[flowbox/*]`）は `filter_noise` で除去される
- タグ検証が必要な smoke は raw output（filter 前）を参照する
- タグ coverage SSOT: `docs/development/current/main/design/flowbox-tag-coverage-map-ssot.md`

## Header PHI Entry/Latch Contract (SSOT)

- Entry preds: `entry_incoming` のブロック + host entry block のみ
- Latch preds: header の preds から entry preds を引いた残り
- PHI inputs: entry preds は entry 値、latch preds は latch 値を流す
- 根拠: `src/mir/builder/control_flow/joinir/merge/README.md`（Phase 29ae セクション）
- 修正コミット: `11adec0ab`

## Commands

- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Status

- phase1883: PASS（RC=9 を成功扱い）
- loop_break: PASS（JoinIR main param remap を carrier_order に揃える; historical numbered route label 2 は inventory-only） `cf95afbd8`
