---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-batch7-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial selfhost-subset batch 3

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the next 60 unobserved selfhost-subset rows in
  planner_required_selfhost_subset.tsv line order (lines 101-160).
Source authority + canonical issuer: the P0-normalized case universe
  fixes membership; the front is the same two direct invocations per
  case as batches 1-7 (release VM run + callable-lane MIR dump);
  expected outputs are compared per row against the subset manifest's
  own expected column.
Non-authority: fixture/backend names, planner_tag expectations, VM
  output as route proof, already-observed twins as this-row evidence,
  and unobserved rows as Declined.
Fail-fast boundary: a timeout, crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe serially subset lines 101-160; record
  each outcome below and set the manifest observation_state only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the 60 `corpus=selfhost,
mode=selfhost-subset` rows with `observation_state=unobserved` whose
subset-manifest line is in 101-160:

```text
sub:101-137 mostly fast-gate twins (parse_string2_return_prelude,
        parse_local_fini{,_no_init,_nested_scope}, parser_stmt_equals,
        phi_injector_* (5), phi_missing_step_join_cursor,
        return_continue_hetero{,_nested_return_depth3,_simple},
        rewriteknown_{itoa_complex_step,trim_loop_cond_and_methodcall,
        try_apply_loop_true_tail_return}, seek_array_end_return_if,
        stageb_{bundle_mod_if,trace_if_no_else}, scan_with_quote{,
        _loop,_loop_full}, usingcollector_loop_full,
        scan_methods_loop_block, scan_methods_nested_loop_idx{19,28},
        scan_methods_program_block, while_cap, phi_collect_outer_loop,
        scan_methods nested-loop legacy-alias stems (130-134),
        scan_methods_nested_loop_state_machine, scan_ident,
        phi_injector_nested_no_exit_var_step)
sub:138-160 new selfhost cleanup/fini fixtures (23 rows:
        fini_only, local_fini_slot_capture, fini_cleanup_coexist,
        local_{no_init,multibind,multibind_init,multibind_mixed_init,
        triple_noinit}_cleanup, local_expr_{compare,logic,unary_not,
        unary_minus,unary_mix,call_new,array_map_literal,string_
        concat_len,string_subcmp,null_logic,string_trim_chain,
        bool_combo,logic_precedence,logic_parenthesized,double_not_
        compare}_cleanup)
```

Remaining subset rows (lines 161+) plus the 4 generic-smoke aliases
and 1 fixture-inventory row stay `unobserved` for later serial slices.

## Front definition and mapping

Identical to batches 1-7: two direct invocations per case (release VM
run; `--dump-mir`), same binary, repo root, timeout 10s, no wrappers;
same `B_outcome` vocabulary and `observation_state` mapping. Expected
outputs come from `planner_required_selfhost_subset.tsv` for these
rows.

## Finite outcomes and acceptance

Identical to batches 2-7 — all 60 recorded closes `landed`; named-owner
stops stay unclassified on the missing axis; a front break stops the
batch. Non-claims identical.

## Observation log

(pending — filled serially)
