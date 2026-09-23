---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch6-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost-subset batch 2

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the next 49 unobserved selfhost-subset rows in
  planner_required_selfhost_subset.tsv line order (lines 51-100;
  line 90 has no corpus row).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-6 (release VM run + callable-lane MIR dump);
  expected outputs are compared per row against the subset manifest's
  own expected column.
Non-authority: fixture/backend names, planner_tag expectations, VM
  output as route proof, already-observed fast-gate/subset twins as
  this-row evidence, and unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially subset lines 51-100; record each
  outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the 49 `corpus=selfhost,
mode=selfhost-subset` rows with `observation_state=unobserved` whose
subset-manifest line is in 51-100:

```text
sub:51-67  parse_program2_nested_loop_* family (17 rows:
           if_else_return_local, if_else_if_return,
           if_else_if_else_return, if_fallthrough_join,
           if_else_fallthrough_join{,_return_var,_return_local,
           _else_return_blockexpr{,_var,_local},_return_blockexpr{,
           _local,_local2,_var_blockexpr,_local_blockexpr,
           _blockexpr,_var}})
sub:68-70  scan_methods_nested_loop_no_break_or_continue{,_pure},
           scan_methods_loop
sub:71-74  scan_all_boxes_{empty_then,program_stmt,
           program_stmt_if_nested_program,return_in_debug_guard}
sub:75-77  using_module_roots{,_multi,_priority}
sub:78-83  extract_body_brace_return, find_matching_brace_return_
           continue, generic_loop_print, localssa_block_insts_end,
           loop_cond_if_assign, module_roots_loop
sub:84-89  parse_map, parse_program2_{guard_prog_numeric_parse_loop,
           nested_loop, prelude1_ws_loop, prelude2_depth_ws_loop,
           ws_loopcount_continue}
sub:91-100 parse_stmt_skipws, parse_string2, parse_term2, parse_try,
           parse_using, peek_parse, decode_escapes_if_idx12,
           decode_escapes_loop, parse_loop, parse_string2_real
```

Remaining subset rows (lines 101+) plus the 4 generic-smoke aliases
and 1 fixture-inventory row stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to batches 1-6: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping. Expected
outputs come from `planner_required_selfhost_subset.tsv` for these
rows. Given the batch-6 nondeterminism evidence, canonical admits are
spot-checked once more before classification.

## Finite outcomes and acceptance

Identical to batches 2-6 — all 49 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

(pending — filled serially)
