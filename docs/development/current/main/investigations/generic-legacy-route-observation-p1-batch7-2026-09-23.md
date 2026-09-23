---
Status: landed__2026-09-23
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

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`50dee7a92e`, 2026-09-23. All 49 rows observed serially in subset-
manifest line order; no timeouts, no signals. `exp` is the subset
manifest expected value. Every fixture twins an already-observed
fast-gate row; the serial invocations reproduced the twins' outcomes
verbatim, recorded here per row.

| sub | case | A_rc | A_tail | B_evidence | state |
| --- | --- | --- | --- | --- | --- |
| 51-67 | nested_loop_{if_else_return_local, if_else_if_return, if_else_if_else_return, if_fallthrough_join, if_else_fallthrough_join{,_return_var,_return_local,_else_return_blockexpr{,_var,_local},_return_blockexpr{,_local,_local2,_var_blockexpr,_local_blockexpr,_blockexpr,_var}}} (17 rows) | 0 | (empty) | `callable-loop-handoff/nested-loop-profile-not-admitted` | rejected |
| 68 | scan_methods_nested_loop_no_break_or_continue | 1 | VM err `canonical-call` | `qualified-preflight actual=UnaryOp` site=Body(5) Init | failed-before-loop |
| 69 | scan_methods_nested_loop_no_break_or_continue_pure | 0 | `0` | `nested-loop-profile-not-admitted` | rejected |
| 70 | scan_methods_loop | 1 | `[static-call/legacy-fallback-retired] ParserStringUtilsBox.starts_with/3` | `semantic-package LoopBreakSource ForestBinding UnsupportedAncestor` | rejected |
| 71 | scan_all_boxes_empty_then | 0 | `0` | `callable-loop/recipe source-unsupported-body-statement` | rejected |
| 72 | scan_all_boxes_program_stmt | 0 | `0` | `route-not-front-selected CarrierRelation(DeclarationCoverage)` site=Body(2) | rejected |
| 73 | scan_all_boxes_stmt_if_nested_program | 0 | `4` | `CarrierRelation(TargetCoverage)` site=Body(2) | rejected |
| 74 | scan_all_boxes_return_in_debug_guard | 0 | `3` | `CarrierRelation(DeclarationCoverage)` site=Body(2) | rejected |
| 75 | using_module_roots | 1 | `[static-call/legacy-fallback-retired] ModuleRootsSmokeBox.value/0` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 76 | using_module_roots_multi | 1 | same retired call | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 77 | using_module_roots_priority | 1 | `[static-call/legacy-fallback-retired] ModuleRootsPriorityBox.value/0` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 78 | extract_body_brace_return | 1 | `[static-call/legacy-fallback-retired] ExtractMini.run/0` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 79 | find_matching_brace_return_continue | 1 | printed `OK` then VM err `canonical-call` | `qualified-preflight actual=Print` site=Body(4) | failed-before-loop |
| 80 | generic_loop_print | 1 | VM err `canonical-call` | `callable-loop/recipe source-unsupported-body-statement` | rejected |
| 81 | localssa_block_insts_end | 0 | `0` | `LoopTrueRouteRejected(SourceItemOutsideLoop)` site=Body(2) | rejected |
| 82 | loop_cond_if_assign | 0 | `2` | `LoopCondRouteRejected(SourceItemsMissing)` site=Body(1) | rejected |
| 83 | module_roots_loop | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(8) | rejected |
| 84 | parse_map | 1 | `[static-call/legacy-fallback-retired] ParserLiteralBox.parse_map/3` | `composite/source-target SourceItemDispositionMissing` in LoopBody | rejected |
| 85 | guard_prog_numeric_parse_loop | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(5) | rejected |
| 86 | parse_program2_nested_loop | 0 | `2` | `nested-loop-profile-not-admitted` | rejected |
| 87 | prelude1_ws_loop | 1 | VM err `canonical-call` | `qualified-preflight actual=BinaryOp` site=Body(6) IfCondition | failed-before-loop |
| 88 | prelude2_depth_ws_loop | 1 | VM err `canonical-call` | `qualified-preflight actual=Print` site=Body(7) nested IfThen | failed-before-loop |
| 89 | ws_loopcount_continue | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(5) | rejected |
| 91 | parse_stmt_skipws | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(3) | rejected |
| 92 | parse_string2 | 1 | `generic_loop_v1 skeleton failed: MissingTransientType` | `composite/source-target SourceItemDispositionMissing` in LoopBody IfThen | rejected |
| 93 | parse_term2 | 1 | `[static-call/legacy-fallback-retired] ParserExprBox.parse_unary2/3` | `composite/source-target SourceItemDispositionMissing` in LoopBody | rejected |
| 94 | parse_try | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(3) | rejected |
| 95 | parse_using | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(3) | rejected |
| 96 | peek_parse | 1 | `[static-call/legacy-fallback-retired] ParserPeekBox.parse/3` | `callable-loop-handoff/incomplete-binding-coverage` | rejected |
| 97 | decode_escapes_if_idx12 | 1 | VM err `canonical-call` | `qualified-preflight actual=Loop` site=Body(4) | rejected |
| 98 | decode_escapes_loop | 1 | `[raw-compat/runtime-box-fate-retired/static]` | `mir/normal-root/consume MainMustBeStatic` | failed-before-loop |
| 99 | parse_loop | 1 | `[static-call/legacy-fallback-retired] env.get/1` | `mir/main-import-view/selected-header-missing` | failed-before-loop |
| 100 | parse_string2_real | 1 | `[static-call/legacy-fallback-retired] Main.parse_like/0` | `mir/main-import-view/selected-header-missing` | failed-before-loop |

## Result

- 0/49 `accepted`: no canonical admits in this chunk.
- 37/49 `rejected`: `callable-loop-handoff` (`nested-loop-profile-not-
  admitted` 19, `incomplete-binding-coverage` 1), qualified preflight
  on `Loop` statements (7), `route-not-front-selected`
  (`CarrierRelation` 3, `LoopTrueRouteRejected` 1,
  `LoopCondRouteRejected` 1), `callable-loop/recipe
  source-unsupported-body-statement` (2), `composite/source-target`
  call-site refuses (3), semantic-package `LoopBreakSource` (1).
- 12/49 `failed-before-loop`: `mir/main-import-view/selected-header-
  missing` (6), `MainMustBeStatic` (1), qualified preflight on
  non-`Loop` items — `UnaryOp`/`BinaryOp`/`Print` (5).
- 0 `timeout`, 0 left `unobserved`.
- 26/49 ran green to expected subset output on the compat lane;
  every B-side outcome matched the already-observed fast-gate twin
  verbatim — the two manifests classify the same fixtures identically
  under the same two fronts.
- No nondeterminism appeared in this chunk (no canonical admits to
  vary).
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
