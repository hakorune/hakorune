---
Status: landed__2026-09-23
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

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`a773b2c6c1`, 2026-09-23. All 41 cases observed serially in manifest
order; no timeouts, no signals.

Classification rule applied: `rejected` = the front's own gate
evaluated and refused the case (loop route front, nested-loop handoff,
callable-loop recipe, qualified preflight on a `Loop` statement, parser
grammar refuse is recorded under `failed-before-loop` since the case
cannot be evaluated at all); `failed-before-loop` = a named non-loop
owner stops evaluation (parser, import view, normal-root policy,
semantic-package install/entry-shape, qualified preflight on a
non-`Loop` expression, package install obligation).

| case_id | tsv:line | A_rc | A_stdout_tail | B_outcome | B_evidence | observation_state | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| loop_simple_while_inline_explicit_step_min | 6 | 0 | `6` | canonical-main | `define i64 @main()`, 2 header PHIs, `icmp Lt`, body Adds, `call_global print`, `ret 0` | accepted | existing callable-loop recipe route admits it (not a Main0 arm); expected `6` reproduced |
| joinir_port04_phi_exit_invariant_min | 7 | 4 | (empty) | named-reject | `route-not-front-selected GenericLoopV1NotSelected` site=Body(1) | rejected | expected `__EMPTY__` + rc 4 matches profile |
| loop_cond_break_continue_nested_loop_depth1_min | 14 | 0 | `2` | named-reject | `callable-loop-handoff/nested-loop-profile-not-admitted` | rejected | expected `2` |
| loop_cond_break_continue_empty_then_min | 15 | 0 | `0` | named-reject | `callable-loop/recipe source-unsupported-body-statement` | rejected | expected `0` |
| loop_cond_break_continue_scopebox_min | 16 | 0 | `0` | named-reject | `route-not-front-selected CarrierRelation(TargetCoverage)` site=Body(3) | rejected | A output `0` vs manifest expected `5` — mismatch recorded |
| loop_true | 18 | 1 | VM err `canonical-call only Global targets admitted` | named-reject | `qualified-preflight statement_not_in_first_family actual=Loop` site=Body(3) | rejected | expected `1`; A fails at VM interp gate |
| cond_update | 22 | 1 | VM err `canonical-call` | named-reject | `callable-semantic-lowering/entry-shape-mismatch` | failed-before-loop | non-loop entry-shape owner |
| loop_cond | 23 | 0 | `13` | named-reject | `callable-loop/recipe source-unsupported-body-statement` | rejected | expected `13` |
| loop_cond_continue_only | 24 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(5) | rejected | expected `3` |
| loop_cond_continue_with_return_min | 25 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(5) | rejected | expected `__EMPTY__` |
| loop_cond_continue_only_group_prelude | 26 | 0 | (empty) | named-reject | `route-not-front-selected GenericLoopV1NotSelected` site=Body(2) | rejected | expected `__EMPTY__` |
| loop_cond_continue_if_else | 27 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(4) | rejected | expected `a` |
| step_tail_break | 28 | 0 | `1` | named-reject | `route-not-front-selected CarrierRelation(MissingIncrement)` site=Body(2) | rejected | expected `1` |
| loop_continue_only_multidelta_min | 37 | 0 | `31` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(3) | rejected | expected `31` |
| parserish_handled | 38 | 0 | `30` | named-reject | `callable-loop/recipe source-unsupported-body-statement` | rejected | expected `30` |
| continue_target_header | 46 | 0 | `33` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(3) | rejected | expected `33` |
| cond_truthiness_value | 124 | 0 | `12345` | canonical-main | `define i64 @main()` | accepted | no loop (StepTree case); expected `12345` |
| cond_truthiness_null | 125 | 1 | VM err `Type error: Void in boolean context` | canonical-main | `define i64 @main()` | accepted | manifest expects exactly this terminal (allowed_rc=1); front compiles and reproduces it |
| scan_loop_v0_comma_close_min | 144 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight expression_not_in_first_family actual=New` site=Body(0) | failed-before-loop | non-loop expression at Body(0) |
| scan_loop_v0_lte_n_minus1_min | 145 | 1 | parse error | named-reject | `parser/while_legacy_replaced_by_loop_condition` line 10 | failed-before-loop | legacy `while` grammar refused before any route |
| loop_header_shortcircuit_continue_only_min | 157 | 0 | `0` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `0` |
| loop_header_shortcircuit_continue_with_return_min | 158 | 0 | `0` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `0` |
| loop_header_shortcircuit_break_continue_min | 160 | 0 | `0` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `0` |
| loop_cond_break_nested_if_tree | 162 | 0 | `0` | named-reject | `route-not-front-selected CarrierRelation(MissingIncrement)` site=Body(2) | rejected | A output `0` vs expected `1` — mismatch recorded |
| map_literal_percent_min | 163 | 1 | VM err `NewBox IntrinsicMap unimplemented` | named-reject | `callable-semantic-package/install MapObligationDescribe(OwnerTerminalHomesUnavailable)` | failed-before-loop | Map obligation at package install, pre-loop |
| blockexpr_basic_min | 164 | 0 | `0` | canonical-main | `define i64 @main()` | accepted | no loop; expected `0` |
| blockexpr_return_min | 165 | 0 | (empty) | canonical-main | `define i64 @main()` | accepted | no loop; expected `__EMPTY__` |
| blockexpr_exit_forbidden_min | 166 | 1 | `[freeze:contract][blockexpr] exit stmt is forbidden` | named-reject | `semantic-package ResolverDeferred(BlockExprNonLocalExit)` site=Body(0).Initializer(0).BlockExprPrelude(0) | rejected | expected terminal reproduced on both axes |
| stageb_blockexpr_return_min | 167 | 0 | (empty) | canonical-main | `define i64 @main()` | accepted | no loop; expected `__EMPTY__` |
| cond_prelude_planner_required_min | 168 | 0 | `0` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(2) | rejected | expected `0` |
| loop_cond_break_else_break_min | 170 | 0 | `0` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(2) | rejected | expected `0` |
| loop_cond_then_only_break_assign_min | 171 | 0 | `5` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(1) | rejected | expected `5` |
| loop_cond_else_only_return_print_min | 172 | 1 | VM err `canonical-call` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(1) | rejected | expected `OK` |
| module_roots_env_parse_min | 173 | 1 | `[static-call/legacy-fallback-retired] env.get/1` | named-reject | `callable-semantic-lowering/entry-shape-mismatch` | failed-before-loop | retired static call blocks entry shape |
| using_module_roots_min | 174 | 1 | `[static-call/legacy-fallback-retired] ModuleRootsSmokeBox.value/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view, pre-loop |
| using_module_roots_multi_min | 175 | 1 | same retired static call | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| using_module_roots_priority_min | 176 | 1 | same retired static call | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| strict_nested_loop_guard_min | 178 | 1 | `[raw-compat/runtime-box-fate-retired/static]` | named-reject | `mir/normal-root/consume SourcePolicy(MainMustBeStatic)` | failed-before-loop | expected text `[plan/freeze:unstructured]` — terminal drifted to the earlier raw-compat retirement |
| strict_nested_loop_guard_accept_min | 179 | 1 | same | named-reject | `mir/normal-root/consume SourcePolicy(MainMustBeStatic)` | failed-before-loop | non-static root |
| entry_ambiguous_break_min | 180 | 0 | `1` | named-reject | `route-not-front-selected GenericLoopV1NotSelected` site=Body(2) | rejected | expected `1` |
| loop_true_receiver_field_nested_exit | 182 | 1 | VM err `legacy-call/global-stopped` | named-reject | `LoopTrueRouteRejected(SourceCallOutsideSelectedFamily)` fn=ReceiverFieldPredicateProbe.run/0 | rejected | instance method loop, not static Main |

## Result

- 6/41 `accepted`: one loop case (`loop_simple_while_inline_explicit_step_min`)
  already reaches canonical `define i64 @main()` through the existing
  callable-loop recipe route — not a Main0 arm; two PHI + `call_global
  print` + `ret 0`. Five non-loop cases (truthiness/blockexpr family)
  also produce canonical main or the manifest-expected terminal.
- 25/41 `rejected`: typed refuses by loop-route front
  (`route-not-front-selected`/`GenericLoopV1NotSelected`/`LoopCondRouteRejected`
  =13), nested-loop handoff (1), callable-loop recipe body (3),
  qualified preflight on `Loop` statements (6), `LoopTrueRouteRejected`
  (1), and one non-loop semantic refuse (`BlockExprNonLocalExit`).
- 10/41 `failed-before-loop`: parser grammar refuse (1), import view (3),
  normal-root `MainMustBeStatic` (2), entry-shape mismatch (2), non-`Loop`
  preflight expression (1), Map install obligation (1).
- 0 `timeout`, 0 left `unobserved`.
- A-axis mismatches vs manifest recorded: `scopebox` (got `0`, expected
  `5`), `nested_if_tree` (got `0`, expected `1`), several cases whose
  compat VM interp hits the `canonical-call only Global targets` gate,
  and `strict_nested_loop_guard_min` whose expected terminal drifted
  from `[plan/freeze:unstructured]` to
  `[raw-compat/runtime-box-fate-retired/static]`. These are compat-lane
  records only, not route authority.
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
