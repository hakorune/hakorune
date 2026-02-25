# Phase 29ae: JoinIR Regression Pack (docs-first)

Goal: JoinIR の最小回帰セットを SSOT として固定する。

## Regression pack (SSOT)

- Pattern2: `phase29ab_pattern2_*`
- Pattern2 (real-world): `phase263_pattern2_*`
- Pattern2 (subset, strict shadow, VM): `phase29ai_pattern2_break_plan_subset_ok_min_vm`
- Pattern2 (release adopt, VM): `phase29ao_pattern2_release_adopt_vm`
- Pattern3 (If‑Phi, VM): `phase118_pattern3_if_sum_vm`
- Pattern3 (release adopt, VM): `phase29ao_pattern3_release_adopt_vm`
- Pattern4 (continue min, VM): `phase29ap_pattern4_continue_min_vm`
- Pattern1 (strict shadow, VM): `phase29ao_pattern1_strict_shadow_vm`
- Pattern1 (subset reject, VM): `phase29ao_pattern1_subset_reject_extra_stmt_vm`
- Pattern1 (stdlib to_lower, VM): `phase29ap_stringutils_tolower_vm`
- Pattern1 (stdlib join, VM): `phase29ap_stringutils_join_vm`
- ScanWithInit (stdlib index_of, VM): `phase29aq_string_index_of_min_vm`
- ScanWithInit (stdlib last_index_of, VM): `phase29aq_string_last_index_of_min_vm`
- ScanWithInit (stdlib index_of_string, VM): `phase29aq_string_index_of_string_min_vm`
- Pattern2 (stdlib parse_integer, VM): `phase29aq_string_parse_integer_min_vm`
- Pattern2 (stdlib parse_integer sign, VM): `phase29aq_string_parse_integer_sign_min_vm`
- Pattern2 (stdlib parse_integer leading whitespace, VM): `phase29aq_string_parse_integer_ws_min_vm`
- Pattern2 (stdlib parse_integer leading zero, VM): `phase29aq_string_parse_integer_leading_zero_min_vm`
- SplitScan (stdlib split, VM): `phase29aq_string_split_min_vm`
- SplitScan (stdlib split char, VM): `phase29aq_string_split_char_min_vm`
- SplitScan (stdlib split string, VM): `phase29aq_string_split_string_min_vm`
- Pattern2 (stdlib trim_start, VM): `phase29aq_string_trim_start_min_vm`
- Pattern2 (stdlib trim_end, VM): `phase29aq_string_trim_end_min_vm`
- Derived (stdlib contains, VM): `phase29aq_string_contains_min_vm`
- Derived (stdlib starts_with, VM): `phase29aq_string_starts_with_min_vm`
- Derived (stdlib ends_with, VM): `phase29aq_string_ends_with_min_vm`
- Derived (stdlib trim, VM): `phase29aq_string_trim_min_vm`
- Pattern1 (stdlib to_upper, VM): `phase29aq_string_to_upper_min_vm`
- Purity gate (strict fallback visibility, VM): `phase29as_purity_gate_vm`
- Return-in-loop (stdlib is_integer, strict shadow, VM): `phase29ar_string_is_integer_min_vm`
- Return-in-loop (stdlib is_integer, release adopt, VM): `phase29ar_string_is_integer_release_adopt_vm`
- Generic loop (continue, strict shadow, VM): `phase29ca_generic_loop_continue_strict_shadow_vm`
- Generic loop (continue, release adopt, VM): `phase29ca_generic_loop_continue_release_adopt_vm`
- Generic loop (in-body step, strict shadow, VM): `phase29cb_generic_loop_in_body_step_strict_shadow_vm`
- Generic loop (in-body step, release adopt, VM): `phase29cb_generic_loop_in_body_step_release_adopt_vm`
- BranchN (match return-only, strict shadow, VM): `phase29at_match_return_strict_shadow_vm`
- BranchN (match return-only, release adopt, VM): `phase29at_match_return_release_adopt_vm`
- FlowBox tags gate (strict/non-strict, VM): `phase29av_flowbox_tags_gate_vm`
- FlowBox tag coverage gate (strict/non-strict, VM): `phase29aw_flowbox_tag_coverage_gate_vm`
- Pattern5 (Break, VM): `phase286_pattern5_break_vm`
- Pattern5 (strict shadow, VM): `phase29ao_pattern5_strict_shadow_vm`
- Pattern5 (release adopt, VM): `phase29ao_pattern5_release_adopt_vm`
- Pattern6 (strict shadow, VM): `phase29ao_pattern6_strict_shadow_vm`
- Pattern6 (release adopt, VM): `phase29ao_pattern6_release_adopt_vm`
- Pattern6: `phase29ab_pattern6_*`
- Pattern6 (nested minimal release adopt, VM): `phase29ap_pattern6_nested_release_adopt_vm`
- Pattern6 (nested minimal strict shadow, VM): `phase29ap_pattern6_nested_strict_shadow_vm`
- Pattern7 (strict shadow, VM): `phase29ao_pattern7_strict_shadow_vm`
- Pattern7 (release adopt, VM): `phase29ao_pattern7_release_adopt_vm`
- Pattern7: `phase29ab_pattern7_*`
- この pack が JoinIR 回帰の唯一の integration gate（phase143_* は対象外）
- JoinIR routing is plan/composer SSOT only (legacy loop table removed in Phase 29ap P12)
- phase143_* は LoopBuilder 撤去 / plugin disable 固定 / LLVM exe 期待が古いので除外
- phase286_pattern9_* は plugins disabled 経路の mismatch があるため legacy pack 側で SKIP（phase29ae pack には含めない）
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
- pattern2: PASS（JoinIR main param remap を carrier_order に揃える） `cf95afbd8`
