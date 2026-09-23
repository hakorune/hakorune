---
Status: design_stop__serial_route_observation_open
Task: GENERIC-LEGACY-ROUTE-OBSERVATION-P1
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
PreviousCard: generic-legacy-route-observation-p1-2026-09-23.md
NextCard: same-row__next_serial_batch_selected_by_family_scheduler
Implementation permission: false; record route/bypass/RC/output for the
bounded serial batch only. No source, fixture, corpus-universe,
semantic-receipt, producer, selector, or route changes.
---

# Generic legacy route observation P1 — serial phase29bq batch

## Six-line brief

```text
Decision: continue the serial P1 route observation; the bounded batch is
  the 41 unobserved phase29bq-corpus fast-gate cases in manifest order.
Source authority + canonical issuer: the P0-normalized case universe
  (design/fixtures/generic-loop-legacy-disposition-v1.tsv) fixes
  membership; the green front receipt is the selected MIR source-backed
  lane exercised by the same two direct invocations per case as batch 1
  (release VM run + callable-lane MIR dump).
Non-authority: fixture/backend names, manifest planner_tag expectations
  (historical lane evidence only), VM output as route proof, and
  unobserved cases as Declined.
Fail-fast boundary: a timeout, a crash, or a stop before the GenericLoop
  route at a named owner stays unclassified for that axis; no
  manufactured result, no wrapper substitution, no parallel census.
Smallest next slice: observe cases serially in manifest order from
  tsv:6 through tsv:182; record each outcome below and set the manifest
  observation_state for observed rows only.
Non-claims: no producer/Recipe/selector arm, no route_loop switch, no
  source/fixture/smoke change, no disposition classification
  (GENERIC-LEGACY-DISPOSITION-D0 owns that), no S6E/S6G/M9/M10 closeout,
  no corpus re-census, no guard vocabulary change.
```

## Fixed boundary

The bounded batch is exactly the 41 `corpus=phase29bq, mode=fast-gate`
case rows with `observation_state=unobserved`, in
`phase29bq_fast_gate_cases.tsv` line order:

```text
tsv:6   loop_simple_while_inline_explicit_step_min
tsv:7   joinir_port04_phi_exit_invariant_min
tsv:14  loop_cond_break_continue_nested_loop_depth1_min
tsv:15  loop_cond_break_continue_empty_then_min
tsv:16  loop_cond_break_continue_scopebox_min
tsv:18  loop_true
tsv:22  cond_update
tsv:23  loop_cond
tsv:24  loop_cond_continue_only
tsv:25  loop_cond_continue_with_return_min
tsv:26  loop_cond_continue_only_group_prelude
tsv:27  loop_cond_continue_if_else
tsv:28  step_tail_break
tsv:37  loop_continue_only_multidelta_min
tsv:38  parserish_handled
tsv:46  continue_target_header
tsv:124 cond_truthiness_value
tsv:125 cond_truthiness_null
tsv:144 scan_loop_v0_comma_close_min
tsv:145 scan_loop_v0_lte_n_minus1_min
tsv:157 loop_header_shortcircuit_continue_only_min
tsv:158 loop_header_shortcircuit_continue_with_return_min
tsv:160 loop_header_shortcircuit_break_continue_min
tsv:162 loop_cond_break_nested_if_tree
tsv:163 map_literal_percent_min
tsv:164 blockexpr_basic_min
tsv:165 blockexpr_return_min
tsv:166 blockexpr_exit_forbidden_min
tsv:167 stageb_blockexpr_return_min
tsv:168 cond_prelude_planner_required_min
tsv:170 loop_cond_break_else_break_min
tsv:171 loop_cond_then_only_break_assign_min
tsv:172 loop_cond_else_only_return_print_min
tsv:173 module_roots_env_parse_min
tsv:174 using_module_roots_min
tsv:175 using_module_roots_multi_min
tsv:176 using_module_roots_priority_min
tsv:178 strict_nested_loop_guard_min
tsv:179 strict_nested_loop_guard_accept_min
tsv:180 entry_ambiguous_break_min
tsv:182 loop_true_receiver_field_nested_exit
```

All other corpus rows stay `unobserved` for later serial slices.

## Front definition

Same two direct invocations per case as
`generic-legacy-route-observation-p1-2026-09-23.md` (same binary,
working directory = repo root, timeout 10s, no smoke wrappers):

```text
A. release VM run   : env NYASH_DISABLE_PLUGINS=1 NYASH_JOINIR_DEV=0
                      NYASH_JOINIR_STRICT=0 HAKO_JOINIR_STRICT=0
                      NYASH_JOINIR_DEBUG=0 HAKO_JOINIR_DEBUG=0
                      hakorune --backend vm <fixture>
B. callable-lane MIR: hakorune --dump-mir <fixture>
```

`B_outcome` vocabulary is unchanged: `canonical-main`,
`legacy-void-main`, `named-reject(owner)`, `build-red(owner)`,
`timeout`, `spawn-error`. A non-zero signal exit (e.g. SIGSEGV) records
`spawn-error` with the signal, never a manufactured classification.

`observation_state` mapping is unchanged: front-matching run ->
`accepted`; typed named-reject/build-red -> `rejected`; stop before the
GenericLoop route at a named owner -> `failed-before-loop`; timeout ->
`timeout`; otherwise `unobserved`.

## Finite outcomes and acceptance

| Outcome | Evidence | Allowed next step |
| --- | --- | --- |
| All 41 recorded | each row has A_rc + B_outcome; TSV observation_state set | Card closes `landed`; next serial P1 batch selected by the family scheduler. |
| A case stops at a named owner | owner + token recorded | Keep the case unclassified on the missing axis; the owner is a separate repair row. |
| Front itself breaks | recorded as build-red/spawn-error | Stop the batch; classification is not manufactured. |

Non-claims: identical to batch 1 — no production-acceptance claim, no
route switch or retirement, no membership change, no disposition.
`GENERIC-LEGACY-DISPOSITION-D0` remains the only row that may check
dispositions.

## Observation log

(pending — filled serially)
