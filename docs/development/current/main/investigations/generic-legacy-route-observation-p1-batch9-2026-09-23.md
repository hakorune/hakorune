---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch8-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost-subset batch 4 (final subset chunk)

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the last 44 unobserved selfhost-subset rows in
  planner_required_selfhost_subset.tsv line order (lines 161-204).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-8 (release VM run + callable-lane MIR dump);
  expected outputs are compared per row against the subset manifest's
  own expected column.
Non-authority: fixture/backend names, planner_tag expectations, VM
  output as route proof, already-observed twins as this-row evidence,
  and unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially subset lines 161-204; record
  each outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the last 44 `corpus=selfhost,
mode=selfhost-subset` rows with `observation_state=unobserved`:

```text
sub:161-188 local_expr_*_cleanup continuation (28 rows: compare_chain,
        compare_or, compare_mixed_logic, null_compare_{and,or,mixed_
        logic}, null_not_compare, null_parenthesized_logic,
        arith_compare_{and,or}, compare_rel_mixed_logic, unary_compare_
        mixed, double_not_mixed_logic, parenthesized_compare_mixed,
        not_parenthesized_compare, call_compare_{and,or}, new_compare_
        mixed, string_compare_logic, array_len_compare, map_value_
        compare, literal_bool_mixed, blockexpr_compare, compare_fini,
        call_fini, blockexpr_fini, local_fini_multi_lifo,
        local_expr_null_fini)
sub:189-190 control_loop_{if_break_continue,local_fini}_cleanup
sub:191-204 box_member_* deep cleanup chains (14 rows:
        loop_cleanup, local_fini{,_blockexpr{,_compare_logic{,_unary_
        call{,_literals{,_nested_tail{,_nested_loop_branch{,_method_
        chain_tail{,_side_effect_tail{,_nested_join_tail{,_dual_tail_
        sync{,_guard_sync_tail{,_mirror_sync_tail}}}}}}}}}}})
```

After this batch only the 4 generic-smoke aliases and 1
fixture-inventory row remain `unobserved` in the corpus.

## Front definition and mapping

Identical to batches 1-8: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping. Expected
outputs come from `planner_required_selfhost_subset.tsv` for these
rows.

## Finite outcomes and acceptance

Identical to batches 2-8 — all 44 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

(pending — filled serially)
