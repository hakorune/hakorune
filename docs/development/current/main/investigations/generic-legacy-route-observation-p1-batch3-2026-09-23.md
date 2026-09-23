---
Status: landed__2026-09-23
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

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`58c13914af`, 2026-09-23. All 41 cases observed serially in manifest
order; no timeouts, no signals. Same classification rule as batch 2.

| case | tsv | A_rc | A_tail | B_outcome | B_evidence | state | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| parse_string2_return_prelude_call_min | 4 | 1 | `[static-call/legacy-fallback-retired] Main.parse_string2_min/1` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | retired static call + import view |
| trim_generic_loop_min | 5 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `OK` |
| rewriteknown_trim_loop_cond_and_methodcall_min | 8 | 1 | `[static-call/legacy-fallback-retired] TrimMini.run/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| find_matching_brace_return_continue_min | 9 | 1 | printed `OK` then VM err `canonical-call` | named-reject | `qualified-preflight actual=Print` site=Body(4) | failed-before-loop | non-Loop stmt at Body(4) |
| scan_methods_program_block_min | 10 | 1 | `ssa/phi_input/without_def` fn=main pred=bb71 | named-reject | `semantic-package LoopBreakSource Composite(ForestLookup)` | rejected | A-side compat lane hits an SSA finalize failure, not a retire terminal |
| scan_methods_loop_block_min | 11 | 1 | VM err `canonical-call` | named-reject | `semantic-package ResolverDeferred(UnresolvedName "j")` in LoopBody IfElse | rejected | resolver refuse inside loop |
| scan_methods_program_stmt_with_loop_min | 12 | 0 | `0` | named-reject | `semantic-package LoopBreakSource Composite(ForestLookup)` | rejected | expected `0` on compat lane |
| scan_methods_program_stmt_with_loop_methodcall_min | 13 | 1 | VM err `canonical-call` | named-reject | `semantic-package LoopBreakSource Composite(ForestLookup)` | rejected | expected `2` |
| scan_methods_nested_loop_depth1_methodcall_min | 17 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=UnaryOp` site=Body(5) | failed-before-loop | non-Loop expr |
| rewriteknown_try_apply_loop_true_tail_return_min | 19 | 1 | `[static-call/legacy-fallback-retired] RewriteKnownMini.run/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| rewriteknown_try_apply_loop_true_else_exit_min | 20 | 1 | same retired call | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| rewriteknown_itoa_complex_step_min | 21 | 1 | `[static-call/legacy-fallback-retired] ItoaMini.run/0` | named-reject | `semantic-package ResolverDeferred(UnsupportedExpression "expression-if consumer")` in LoopBody | rejected | semantic refuse inside loop |
| parse_string2_min | 39 | 1 | `generic_loop_v1 skeleton failed: MissingTransientType` | named-reject | `callable-loop/composite/source-target SourceItemDispositionMissing` in LoopBody IfThen | rejected | loop composite refuse |
| parse_string2_real_min | 40 | 1 | `[static-call/legacy-fallback-retired] Main.parse_like/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| parse_block_min | 41 | 1 | `[static-call/legacy-fallback-retired] Ctx.skip_ws/2` | named-reject | `semantic-package LoopBreakSource ForestBinding UnsupportedAncestor` | rejected | expected `[]@2` |
| localssa_block_insts_end_min | 42 | 0 | `0` | named-reject | `route-not-front-selected LoopTrueRouteRejected(SourceItemOutsideLoop)` site=Body(2) | rejected | expected `0` |
| parse_try_program_block_min | 43 | 0 | `[ab` | named-reject | `LoopTrueRouteRejected(SourceItemOutsideLoop)` site=Body(3) | rejected | expected `[ab` |
| parse_block_v2_min | 44 | 1 | `[static-call/legacy-fallback-retired] Ctx.i2s/1` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| parse_loop_min | 45 | 1 | `[static-call/legacy-fallback-retired] env.get/1` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| scan_with_quote_min | 47 | 1 | `[static-call/legacy-fallback-retired] Utils.i2s/1` | named-reject | `route-not-front-selected GenericLoopV1NotSelected` fn=Main.scan_with_quote_min/3 | rejected | non-main method loop |
| scan_with_quote_loop_min | 48 | 1 | same retired call | named-reject | `GenericLoopV1NotSelected` fn=Main.scan_with_quote_loop_min/3 | rejected | method loop |
| scan_with_quote_loop_full_min | 49 | 1 | same retired call | named-reject | `GenericLoopV1NotSelected` fn=Main.scan_with_quote_loop_full_min/3 | rejected | method loop |
| usingcollector_loop_full_min | 50 | 1 | `[static-call/legacy-fallback-retired] UsingCollectorBox.collect/1` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| peek_parse_min | 51 | 1 | `[static-call/legacy-fallback-retired] ParserPeekBox.parse/3` | named-reject | `callable-loop-handoff/incomplete-binding-coverage` | rejected | handoff refuse |
| bundle_resolver_min | 52 | 1 | `[raw-compat/runtime-box-fate-retired/static]` | named-reject | `mir/normal-root/consume SourcePolicy(MainMustBeStatic)` | failed-before-loop | non-static root |
| scan_ident_min | 53 | 1 | `[static-call/legacy-fallback-retired] Utils.i2s/1` | named-reject | `qualified-preflight actual=Print` site=Body(1) | failed-before-loop | non-Loop stmt |
| return_continue_hetero | 54 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=New` site=Body(0) | failed-before-loop | non-Loop expr |
| breakfinder_parse_int_min | 55 | 1 | `[static-call/legacy-fallback-retired] Main.parse/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | import view |
| parse_try_min | 56 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `2` |
| parse_map_min | 57 | 1 | `[static-call/legacy-fallback-retired] ParserLiteralBox.parse_map/3` | named-reject | `callable-loop/composite/source-target SourceItemDispositionMissing` in LoopBody | rejected | composite refuse |
| parse_block_expr_min | 58 | 1 | same retired call | named-reject | `composite/source-target SourceItemDispositionMissing` in LoopBody | rejected | composite refuse |
| parse_term2_min | 59 | 1 | `[static-call/legacy-fallback-retired] ParserExprBox.parse_unary2/3` | named-reject | `composite/source-target SourceItemDispositionMissing` in LoopBody | rejected | composite refuse |
| parse_program2_ws_min | 60 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `1` |
| parse_program2_ws_or_min | 61 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `1` |
| parse_program2_ws_loopcount_continue_min | 62 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(5) | rejected | expected `1` |
| parse_program2_prelude1_ws_loop_min | 63 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=BinaryOp` site=Body(6).IfCondition | failed-before-loop | non-Loop expr |
| parse_program2_prelude2_depth_ws_loop_min | 64 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Print` site=Body(7) nested IfThen | failed-before-loop | non-Loop stmt |
| parse_program2_cond_and_min | 65 | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | expected `1` |
| parse_program2_effect_if_min | 66 | 1 | VM err `canonical-call` | named-reject | `route-not-front-selected CarrierRelation(MissingIncrement)` site=Body(2) | rejected | expected `P` |
| parse_program2_bool_or_mod_min | 67 | 0 | `1` | named-reject | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(4) | rejected | expected `1` |
| parse_program2_if_return_min | 68 | 0 | `7` | canonical-main | `define i64 @main()` | accepted | expected `7` |

## Result

- 1/41 `accepted`: `parse_program2_if_return_min` compiles to canonical
  `define i64 @main()` and runs the expected `7`.
- 24/41 `rejected`: loop route front refuses
  (`route-not-front-selected`/`GenericLoopV1NotSelected`/
  `LoopCondRouteRejected`/`LoopTrueRouteRejected` =8), qualified
  preflight on `Loop` statements (6), semantic-package loop issuer
  (`LoopBreakSource`/`ResolverDeferred` inside LoopBody =5),
  `callable-loop/composite/source-target` call-site refuses (4), and
  nested-loop handoff coverage (1).
- 16/41 `failed-before-loop`: `mir/main-import-view/selected-header-
  missing` (9), `MainMustBeStatic` (1), qualified preflight on
  non-`Loop` items — `Print`/`New`/`UnaryOp`/`BinaryOp` (6).
- 0 `timeout`, 0 left `unobserved`.
- Compat-lane context: 4 cases ran green to their expected output
  (12,42,43,67); 9 printed expected output before dying at the
  `[vm-reference/canonical-call] only Global targets` interp gate or a
  retired static-call terminal; `scan_methods_program_block_min`
  uniquely failed A at an `ssa/phi_input/without_def` finalize error —
  recorded as compat-lane evidence only, not a current-change claim.
- Retired static calls (`static-call/legacy-fallback-retired`) are the
  dominant A-side terminal: 16 cases stop there before any loop work,
  matching the recorded baseline retirement family.
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
