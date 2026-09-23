---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch3-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost fast-gate batch 2

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the next 42 unobserved selfhost-corpus fast-gate cases in manifest
  order (phase29bq_fast_gate_cases.tsv lines 69-110).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-3 (release VM run + callable-lane MIR dump).
Non-authority: fixture/backend names, manifest planner_tag expectations,
  VM output as route proof, and unobserved cases as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially tsv:69 through tsv:110; record
  each outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the next 42 `corpus=selfhost,
mode=fast-gate` rows with `observation_state=unobserved`, in manifest
order:

```text
tsv:69  selfhost_parse_program2_if_return_var_min
tsv:70  selfhost_parse_program2_if_return_local_min
tsv:71  selfhost_parse_program2_if_fallthrough_join_min
tsv:72  selfhost_parse_program2_if_else_return_min
tsv:73  selfhost_parse_program2_if_else_return_var_min
tsv:74  selfhost_parse_program2_if_else_return_local_min
tsv:75  selfhost_parse_program2_if_else_if_return_min
tsv:76  selfhost_parse_using_min
tsv:77  selfhost_parse_stmt_skipws_min
tsv:78  selfhost_parse_program2_nested_loop_min
tsv:79  selfhost_parse_program2_nested_loop_if_return_min
tsv:80  selfhost_parse_program2_nested_loop_if_else_return_min
tsv:81  selfhost_parse_program2_nested_loop_if_return_var_min
tsv:82  selfhost_parse_program2_nested_loop_if_return_local_min
tsv:83  selfhost_parse_program2_nested_loop_if_else_return_var_min
tsv:84  selfhost_parse_program2_nested_loop_if_else_return_local_min
tsv:85  selfhost_parse_program2_nested_loop_if_else_if_return_min
tsv:86  selfhost_parse_program2_nested_loop_if_else_if_else_return_min
tsv:87  selfhost_parse_program2_nested_loop_if_fallthrough_join_min
tsv:88  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_min
tsv:89  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_var_min
tsv:90  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_local_min
tsv:91  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min
tsv:92  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_var_min
tsv:93  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_else_return_local_min
tsv:94  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_min
tsv:95  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local_min
tsv:96  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_local2_min
tsv:97  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_var_blockexpr_min
tsv:98  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_local_blockexpr_min
tsv:99  selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_blockexpr_min
tsv:100 selfhost_parse_program2_nested_loop_if_else_fallthrough_join_return_blockexpr_var_min
tsv:101 selfhost_parser_stmt_equals_min
tsv:102 selfhost_parse_program2_loop_min
tsv:103 selfhost_parse_program2_loop_if_fallthrough_join_min
tsv:104 selfhost_parse_program2_loop_if_return_min
tsv:105 selfhost_parse_program2_loop_if_return_var_min
tsv:106 selfhost_parse_program2_loop_if_return_local_min
tsv:107 selfhost_parse_program2_loop_continue_if_min
tsv:108 selfhost_parse_program2_loop_if_else_return_min
tsv:109 selfhost_parse_program2_loop_if_else_return_var_min
tsv:110 selfhost_parse_program2_loop_if_else_return_local_min
```

All other corpus rows stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to batches 1-3: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping.

## Finite outcomes and acceptance

Identical to batches 2-3 — all 42 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

(pending — filled serially)
