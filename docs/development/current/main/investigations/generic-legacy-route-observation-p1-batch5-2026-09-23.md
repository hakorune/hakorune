---
Status: landed__2026-09-23
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch4-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost fast-gate batch 3 (final selfhost chunk)

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the remaining 43 unobserved selfhost-corpus fast-gate cases in
  manifest order (phase29bq_fast_gate_cases.tsv lines 111-181).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-4 (release VM run + callable-lane MIR dump).
Non-authority: fixture/backend names, manifest planner_tag expectations,
  VM output as route proof, and unobserved cases as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially tsv:111 through tsv:181; record
  each outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the last 43 `corpus=selfhost,
mode=fast-gate` rows with `observation_state=unobserved`, in manifest
order:

```text
tsv:111 selfhost_parse_program2_loop_if_else_if_return_min
tsv:112 selfhost_parse_program2_loop_if_else_if_else_return_min
tsv:113 selfhost_parse_program2_guard_prog_numeric_parse_loop_min
tsv:114 selfhost_read_next_number_literal_staged_loop_break_min
tsv:115 selfhost_read_number_sign_break_fullish_min
tsv:116 selfhost_read_number_decimal_exponent_min
tsv:117 selfhost_scanner_multi_exit_min
tsv:118 selfhost_read_number_continue_staged_min
tsv:119 selfhost_parse_expr2_min
tsv:120 selfhost_parse_if_min
tsv:121 selfhost_generic_loop_print_min
tsv:126 selfhost_scan_methods_nested_loop_depth1_no_break_or_continue_min
tsv:127 selfhost_scan_methods_nested_loop_depth1_no_break_or_continue_pure_min
tsv:128 selfhost_scan_methods_nested_loop_state_machine_min
tsv:129 selfhost_scan_methods_nested_loop_cluster3_min
tsv:130 selfhost_scan_methods_nested_loop_idx19_min
tsv:131 selfhost_scan_methods_nested_loop_idx28_min
tsv:132 selfhost_scan_methods_nested_loop_idx35_min
tsv:133 selfhost_scan_all_boxes_empty_then_min
tsv:134 selfhost_scan_all_boxes_program_stmt_min
tsv:135 selfhost_scan_all_boxes_program_stmt_if_nested_program_min
tsv:136 selfhost_scan_all_boxes_return_in_debug_guard_min
tsv:137 selfhost_phi_missing_step_join_cursor_min
tsv:138 return_continue_hetero_simple
tsv:139 selfhost_seek_array_end_return_if_min
tsv:140 selfhost_extract_body_brace_return_min
tsv:141 decode_escapes_if_idx12_min
tsv:142 decode_escapes_group_if_fallthrough
tsv:143 while_cap
tsv:146 selfhost_blocker_scan_methods_loop_min
tsv:147 selfhost_collect_using_entries_loop_min
tsv:148 selfhost_decode_escapes_loop_min
tsv:149 phi_injector_len_loop
tsv:150 phi_injector_var_step_len_loop
tsv:151 selfhost_phi_injector_k_loop_no_exit_min
tsv:152 selfhost_phi_collect_outer_loop_min
tsv:153 phi_injector_nested_loop_no_exit_var_step_min
tsv:154 selfhost_stageb_trace_if_no_else_min
tsv:155 selfhost_loop_cond_if_assign_min
tsv:156 selfhost_stageb_bundle_mod_if_min
tsv:161 selfhost_phi_injector_nested_loop_count_min
tsv:177 selfhost_module_roots_loop_min
tsv:181 return_continue_hetero_nested_return_depth3
```

After this batch the selfhost `fast-gate` cohort is fully observed;
non-`fast-gate` corpus rows stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to batches 1-4: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping.

## Finite outcomes and acceptance

Identical to batches 2-4 — all 43 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`3e20b17e53`, 2026-09-23. All 43 cases observed serially in manifest
order; no timeouts, no signals. Same classification rule as batches 1-4.

| case | tsv | A_rc | A_tail | B_outcome | B_evidence | state | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| loop_if_else_if_return_min | 111 | 0 | (empty) | named-reject | `callable-loop-handoff/carrier-cardinality` | rejected | expected `__EMPTY__` |
| loop_if_else_if_else_return_min | 112 | 0 | (empty) | named-reject | `carrier-cardinality` | rejected | expected `__EMPTY__` |
| guard_prog_numeric_parse_loop_min | 113 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(5) | rejected | expected `5` |
| read_next_number_literal_staged_loop_break_min | 114 | 1 | `[static-call/legacy-fallback-retired] Main.parse_number_prefix/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | expected `123` |
| read_number_sign_break_fullish_min | 115 | 1 | `generic_loop_v1 skeleton failed: MissingTransientType` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | expected `-123` |
| read_number_decimal_exponent_min | 116 | 1 | `[static-call/legacy-fallback-retired] Main.read_number_end/2` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | expected `9` |
| scanner_multi_exit_min | 117 | 1 | `[static-call/legacy-fallback-retired] Main.scan_string_body_end/2` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | expected `-4` |
| read_number_continue_staged_min | 118 | 1 | `[static-call/legacy-fallback-retired] Main.count_digits_skip_sep/2` | named-reject | `callable-loop-handoff/incomplete-binding-coverage` | rejected | expected `3` |
| parse_expr2_min | 119 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(2) | rejected | expected `P` |
| parse_if_min | 120 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Print` site=Body(10) | failed-before-loop | expected `P` |
| generic_loop_print_min | 121 | 1 | VM err `canonical-call` | named-reject | `callable-loop/recipe source-unsupported-body-statement` | rejected | expected `P` |
| nested_loop_depth1_no_break_or_continue_min | 126 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=UnaryOp` site=Body(5) Init | failed-before-loop | non-Loop expr |
| nested_loop_no_break_or_continue_pure_min | 127 | 0 | `0` | named-reject | `nested-loop-profile-not-admitted` | rejected | expected `0` on compat lane |
| nested_loop_state_machine_min | 128 | 0 | `2` | named-reject | `semantic-package LoopBreakSource Composite(ForestLookup)` | rejected | expected `2` on compat lane |
| nested_loop_cluster3_min | 129 | 0 | `0` | named-reject | `LoopBreakSource Composite(ForestLookup)` | rejected | expected `0` on compat lane |
| nested_loop_idx19_min | 130 | 1 | VM err `canonical-call` | named-reject | `LoopBreakSource Composite(ForestLookup)` | rejected | expected `OK` |
| nested_loop_idx28_min | 131 | 1 | VM err `canonical-call` | named-reject | `LoopBreakSource Composite(ForestLookup)` | rejected | expected `OK` |
| nested_loop_idx35_min | 132 | 1 | VM err `canonical-call` | named-reject | `LoopBreakSource Composite(ForestLookup)` | rejected | expected `OK` |
| scan_all_boxes_empty_then_min | 133 | 0 | `0` | named-reject | `callable-loop/recipe source-unsupported-body-statement` | rejected | expected `0` on compat lane |
| scan_all_boxes_program_stmt_min | 134 | 0 | `0` | named-reject | `route-not-front-selected CarrierRelation(DeclarationCoverage)` site=Body(2) | rejected | expected `0` on compat lane |
| scan_all_boxes_stmt_if_nested_program_min | 135 | 0 | `4` | named-reject | `route-not-front-selected CarrierRelation(TargetCoverage)` site=Body(2) | rejected | expected `4` on compat lane |
| scan_all_boxes_return_in_debug_guard_min | 136 | 0 | `3` | named-reject | `route-not-front-selected CarrierRelation(DeclarationCoverage)` site=Body(2) | rejected | expected `3` on compat lane |
| phi_missing_step_join_cursor_min | 137 | 1 | VM err `canonical-call` | named-reject | `route-not-front-selected CarrierRelation(MissingIncrement)` site=Body(2) | rejected | expected `0` |
| return_continue_hetero_simple | 138 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `-1` |
| seek_array_end_return_if_min | 139 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `0` |
| extract_body_brace_return_min | 140 | 1 | `[static-call/legacy-fallback-retired] ExtractMini.run/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | expected `1` |
| decode_escapes_if_idx12_min | 141 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(4) | rejected | expected `0` |
| decode_escapes_group_if_fallthrough_min | 142 | 0 | `0` | named-reject | `nested-loop-profile-not-admitted` | rejected | expected `0` on compat lane |
| while_cap_min | 143 | 1 | Parse error `parser/while_legacy_replaced_by_loop_condition` | named-reject | same parser grammar-contract reject at `while` | failed-before-loop | expected `10`; source cannot parse on either front |
| blocker_scan_methods_loop_min | 146 | 1 | `[static-call/legacy-fallback-retired] ParserStringUtilsBox.starts_with/3` | named-reject | `semantic-package LoopBreakSource ForestBinding UnsupportedAncestor` | rejected | expected `0` |
| collect_using_entries_loop_min | 147 | 1 | `[static-call/legacy-fallback-retired] UsingCollectorBox.collect/1` | named-reject | `semantic-package CoreMethodSource NamedArray(TextSourceMissing)` | failed-before-loop | core method source unavailable before loop eval |
| decode_escapes_loop_min | 148 | 1 | `[raw-compat/runtime-box-fate-retired/static]` | named-reject | `mir/normal-root/consume MainMustBeStatic` | failed-before-loop | expected `57005` |
| phi_injector_len_loop_min | 149 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `60` |
| phi_injector_var_step_len_loop_min | 150 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `10` |
| phi_injector_k_loop_no_exit_min | 151 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `6` |
| phi_collect_outer_loop_min | 152 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `6` |
| phi_injector_nested_no_exit_var_step_min | 153 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `18` |
| stageb_trace_if_no_else_min | 154 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(2) | rejected | expected `0` |
| loop_cond_if_assign_min | 155 | 0 | `2` | named-reject | `route-not-front-selected LoopCondRouteRejected(SourceItemsMissing)` site=Body(1) | rejected | expected `2` on compat lane |
| stageb_bundle_mod_if_min | 156 | 1 | `mir/main-expansion/compatibility-preflight DuplicateMainBox` | named-reject | `mir/normal-root/consume IntegrityInvalid` | failed-before-loop | duplicate Main box; root integrity refuse |
| phi_injector_nested_loop_count_min | 161 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `3` |
| module_roots_loop_min | 177 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(8) | rejected | expected `3` |
| return_continue_hetero_nested_return_depth3 | 181 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) Init | failed-before-loop | expected `1` |

## Result

- 0/43 `accepted`: the final selfhost chunk has no canonical admits.
- 23/43 `rejected`: `callable-loop-handoff` (`nested-loop-profile-not-
  admitted` 3, `carrier-cardinality` 2, `incomplete-binding-coverage`
  1), qualified preflight on `Loop` statements (6), semantic-package
  `LoopBreakSource` issuers (6), `callable-loop/recipe
  source-unsupported-body-statement` (2), and
  `route-not-front-selected` (`CarrierRelation` 4,
  `LoopCondRouteRejected` 1).
- 20/43 `failed-before-loop`: `mir/main-import-view/selected-header-
  missing` (5), qualified preflight on non-`Loop` items (10 — mostly
  `New` initializers in `phi_injector_*`/`hetero` fixtures),
  `mir/normal-root/consume` (`MainMustBeStatic` 1, `IntegrityInvalid`
  1), `parser/while_legacy_replaced_by_loop_condition` grammar reject
  (1), and `CoreMethodSource NamedArray(TextSourceMissing)` (1).
- 0 `timeout`, 0 left `unobserved` — the selfhost `fast-gate` cohort
  (126 cases, tsv:4-181) is now fully observed.
- Compat-lane context: 11/43 ran green to expected output;
  `while_cap_min` cannot even parse (`while` is a retired grammar
  profile on both fronts); `stageb_bundle_mod_if_min` carries a
  duplicate Main box caught by root integrity; retired static calls
  remain a dominant A-side terminal.
- New refuse families first recorded here:
  `callable-loop/recipe source-unsupported-body-statement`,
  `CarrierRelation(DeclarationCoverage|TargetCoverage)`,
  `CoreMethodSource NamedArray(TextSourceMissing)`,
  `mir/normal-root/consume IntegrityInvalid`, and the parser
  `while_legacy_replaced_by_loop_condition` grammar profile.
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
