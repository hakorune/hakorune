---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch2-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost fast-gate batch 1

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the first 41 unobserved selfhost-corpus fast-gate cases in manifest
  order (phase29bq_fast_gate_cases.tsv lines 4-68).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-2 (release VM run + callable-lane MIR dump).
Non-authority: fixture/backend names, manifest planner_tag expectations,
  VM output as route proof, and unobserved cases as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially tsv:4 through tsv:68; record each
  outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the first 41 `corpus=selfhost,
mode=fast-gate` rows with `observation_state=unobserved`, in manifest
order:

```text
tsv:4  selfhost_parse_string2_return_prelude_call_min
tsv:5  selfhost_trim_generic_loop_min
tsv:8  selfhost_rewriteknown_trim_loop_cond_and_methodcall_min
tsv:9  selfhost_find_matching_brace_return_continue_min
tsv:10 selfhost_scan_methods_program_block_min
tsv:11 selfhost_scan_methods_loop_block_min
tsv:12 selfhost_scan_methods_program_stmt_with_loop_min
tsv:13 selfhost_scan_methods_program_stmt_with_loop_methodcall_min
tsv:17 selfhost_scan_methods_nested_loop_depth1_methodcall_min
tsv:19 selfhost_rewriteknown_try_apply_loop_true_tail_return_min
tsv:20 selfhost_rewriteknown_try_apply_loop_true_else_exit_min
tsv:21 selfhost_rewriteknown_itoa_complex_step_min
tsv:39 selfhost_parse_string2_min
tsv:40 selfhost_parse_string2_real_min
tsv:41 selfhost_parse_block_min
tsv:42 selfhost_localssa_block_insts_end_min
tsv:43 selfhost_parse_try_program_block_min
tsv:44 selfhost_parse_block_v2_min
tsv:45 selfhost_parse_loop_min
tsv:47 selfhost_scan_with_quote_min
tsv:48 selfhost_scan_with_quote_loop_min
tsv:49 selfhost_scan_with_quote_loop_full_min
tsv:50 selfhost_usingcollector_loop_full_min
tsv:51 selfhost_peek_parse_min
tsv:52 selfhost_bundle_resolver_min
tsv:53 selfhost_scan_ident_min
tsv:54 return_continue_hetero
tsv:55 selfhost_breakfinder_parse_int_min
tsv:56 selfhost_parse_try_min
tsv:57 selfhost_parse_map_min
tsv:58 selfhost_parse_block_expr_min
tsv:59 selfhost_parse_term2_min
tsv:60 selfhost_parse_program2_ws_min
tsv:61 selfhost_parse_program2_ws_or_min
tsv:62 selfhost_parse_program2_ws_loopcount_continue_min
tsv:63 selfhost_parse_program2_prelude1_ws_loop_min
tsv:64 selfhost_parse_program2_prelude2_depth_ws_loop_min
tsv:65 selfhost_parse_program2_cond_and_min
tsv:66 selfhost_parse_program2_effect_if_min
tsv:67 selfhost_parse_program2_bool_or_mod_min
tsv:68 selfhost_parse_program2_if_return_min
```

All other corpus rows stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to
`generic-legacy-route-observation-p1-batch2-2026-09-23.md`: two direct
invocations per case (release VM run; `--dump-mir`), same binary, repo
root, timeout 10s, no wrappers; same `B_outcome` vocabulary and
`observation_state` mapping (`accepted`/`rejected`/
`failed-before-loop`/`timeout`/`unobserved`).

## Finite outcomes and acceptance

Identical to batch 2 — all 41 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical: no production-acceptance claim, no route
switch, no membership change, no disposition.

## Observation log

(pending — filled serially)
