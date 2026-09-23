---
Status: design_stop__serial_route_observation_open
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

(pending — filled serially)
