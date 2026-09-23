---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch5-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost-subset batch 1

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the first 45 unobserved selfhost-subset rows in
  planner_required_selfhost_subset.tsv line order (lines 6-50).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-5 (release VM run + callable-lane MIR dump);
  expected outputs are compared per row against the subset manifest's
  own expected column, not the fast-gate one.
Non-authority: fixture/backend names, planner_tag expectations, VM
  output as route proof, already-observed fast-gate twins as
  selfhost-subset evidence, and unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially subset lines 6-50; record each
  outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change; fast-gate twins do
  not pre-classify these rows.
```

## Fixed boundary

The bounded batch is exactly the first 45 `corpus=selfhost,
mode=selfhost-subset` rows with `observation_state=unobserved`, in
subset-manifest line order:

```text
sub:6   selfhost::if-phi-join                                       (apps/tests/if_phi_join_min.hako)
sub:7   selfhost::rewriteknown_try_apply_loop_true_else_exit_min
sub:8   selfhost::selfhost_subset_scan_funcs_import_min
sub:9   selfhost::map_literal_percent_min
sub:10  selfhost::trim-generic-loop
sub:11  selfhost::cleanup-only
sub:12  selfhost::parse_if_min
sub:13  selfhost::parse_block_expr_min
sub:14  selfhost::parse_block_min
sub:15  selfhost::parse_block_v2_min
sub:16  selfhost::parse_expr2_min
sub:17  selfhost::breakfinder-parse-int
sub:18  selfhost::bundle_resolver_min
sub:19  selfhost::collect_using_entries_loop_min
sub:20  selfhost::decode_escapes_group_if_fallthrough_mutation_min
sub:21  selfhost::parse_program2_if_return_min
sub:22  selfhost::parse_program2_if_return_var_min
sub:23  selfhost::parse_program2_if_return_local_min
sub:24  selfhost::parse_program2_if_fallthrough_join_min
sub:25  selfhost::parse_program2_if_else_return_min
sub:26  selfhost::parse_program2_if_else_return_var_min
sub:27  selfhost::parse_program2_if_else_return_local_min
sub:28  selfhost::parse_program2_if_else_if_return_min
sub:29  selfhost::parse_program2_ws_min
sub:30  selfhost::parse_program2_ws_or_min
sub:31  selfhost::parse_try_program_block_min
sub:32  selfhost::parse_program2_cond_and_min
sub:33  selfhost::parse_program2_effect_if_min
sub:34  selfhost::parse_program2_bool_or_mod_min
sub:35  selfhost::parse_program2_loop_min
sub:36  selfhost::parse_program2_loop_if_fallthrough_join_min
sub:37  selfhost::parse_program2_loop_if_return_min
sub:38  selfhost::parse_program2_loop_if_return_var_min
sub:39  selfhost::parse_program2_loop_if_return_local_min
sub:40  selfhost::parse_program2_loop_continue_if_min
sub:41  selfhost::parse_program2_loop_if_else_return_min
sub:42  selfhost::parse_program2_loop_if_else_return_var_min
sub:43  selfhost::parse_program2_loop_if_else_return_local_min
sub:44  selfhost::parse_program2_loop_if_else_if_return_min
sub:45  selfhost::parse_program2_loop_if_else_if_else_return_min
sub:46  selfhost::parse_program2_nested_loop_if_return_min
sub:47  selfhost::parse_program2_nested_loop_if_else_return_min
sub:48  selfhost::parse_program2_nested_loop_if_return_var_min
sub:49  selfhost::parse_program2_nested_loop_if_return_local_min
sub:50  selfhost::parse_program2_nested_loop_if_else_return_var_min
```

Most fixtures twin already-observed fast-gate rows, but each
selfhost-subset row is its own corpus entry with its own expected
value; they are observed and classified independently. Remaining
subset rows (lines 51+) plus the 4 generic-smoke aliases and 1
fixture-inventory row stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to batches 1-5: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping. Expected
outputs come from `planner_required_selfhost_subset.tsv` for these
rows.

## Finite outcomes and acceptance

Identical to batches 2-5 — all 45 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

(pending — filled serially)
