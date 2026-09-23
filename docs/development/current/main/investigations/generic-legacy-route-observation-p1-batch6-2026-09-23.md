---
Status: landed__2026-09-23
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

Binary: `target/debug/hakorune` (`--features vm-reference`), HEAD
`d888419d6e`, 2026-09-23. All 45 rows observed serially in subset-
manifest line order; no timeouts, no signals. `exp` is the subset
manifest expected value.

| sub | case | A_rc | A_tail | B_outcome | B_evidence | state | notes |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 6 | if-phi-join | 0 | `12` | named-reject | `route-not-front-selected GenericLoopV1NotSelected` site=Body(2) | rejected | exp `12` on compat lane |
| 7 | rewriteknown_try_apply_loop_true_else_exit | 1 | `[static-call/legacy-fallback-retired] RewriteKnownMini.run/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | exp `XY` |
| 8 | subset_scan_funcs_import | 1 | `[static-call/legacy-fallback-retired] ParserStringUtilsBox.starts_with/3` | named-reject | `semantic-package LoopBreakSource ForestBinding UnsupportedAncestor` | rejected | exp `0` |
| 9 | map_literal_percent | 1 | VM err `NewBox IntrinsicMap unimplemented` | named-reject | `semantic-package/install MapObligationDescribe(OwnerTerminalHomesUnavailable)` | failed-before-loop | package install refuse, non-loop owner |
| 10 | trim-generic-loop | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | exp `OK` |
| 11 | cleanup-only | 0 | `11` | named-reject | `semantic-package ResolverDeferred UnsupportedStatement TryCatch` site=Body(1) | failed-before-loop | resolver refuse before loop eval |
| 12 | parse_if_min | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Print` site=Body(10) | failed-before-loop | exp `P` |
| 13 | parse_block_expr_min | 1 | `[static-call/legacy-fallback-retired] ParserLiteralBox.parse_block_expr/3` | named-reject | `composite/source-target SourceItemDispositionMissing` in LoopBody | rejected | exp error JSON |
| 14 | parse_block_min | 1 | `[static-call/legacy-fallback-retired] Ctx.skip_ws/2` | named-reject | `LoopBreakSource ForestBinding UnsupportedAncestor` | rejected | exp `[]@2` |
| 15 | parse_block_v2_min | 1 | `[static-call/legacy-fallback-retired] Ctx.i2s/1` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | exp `[]@2` |
| 16 | parse_expr2_min | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(2) | rejected | exp `P` |
| 17 | breakfinder-parse-int | 1 | `[static-call/legacy-fallback-retired] Main.parse/0` | named-reject | `mir/main-import-view/selected-header-missing` | failed-before-loop | exp `-1` |
| 18 | bundle_resolver_min | 1 | `[raw-compat/runtime-box-fate-retired/static]` | named-reject | `mir/normal-root/consume MainMustBeStatic` | failed-before-loop | exp `1` |
| 19 | collect_using_entries_loop_min | 1 | `[static-call/legacy-fallback-retired] UsingCollectorBox.collect/1` | named-reject | `semantic-package CoreMethodSource NamedArray(TextSourceMissing)` | failed-before-loop | exp `0` |
| 20 | decode_escapes_group_if_fallthrough | 0 | `0` | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `0` on compat lane |
| 21 | parse_program2_if_return_min | 0 | `7` | canonical-main | `define i64 @main()` | accepted | exp `7` |
| 22 | parse_program2_if_return_var_min | 0 | `7` | canonical-main | `define i64 @main()` | accepted | exp `7` |
| 23 | parse_program2_if_return_local_min | 0 | `7` | canonical-main | `define i64 @main()` | accepted | exp `7` |
| 24 | parse_program2_if_fallthrough_join_min | 0 | `1` | canonical-main | `define i64 @main()` | accepted | exp `1` |
| 25 | parse_program2_if_else_return_min | 0 | (empty) | canonical-main | `define i64 @main()` | accepted | exp `__EMPTY__`; **nondeterministic**: also emits `define void @main()` ~1/4 runs |
| 26 | parse_program2_if_else_return_var_min | 0 | (empty) | canonical-main | `define i64 @main()`; serial run hit `define void @main()` (legacy emission) | accepted | exp `__EMPTY__`; **nondeterministic**: `void` 1/6 then `i64` 5/6 |
| 27 | parse_program2_if_else_return_local_min | 0 | (empty) | canonical-main | `define i64 @main()` | accepted | exp `__EMPTY__` |
| 28 | parse_program2_if_else_if_return_min | 0 | (empty) | canonical-main | `define i64 @main()` | accepted | exp `__EMPTY__` |
| 29 | parse_program2_ws_min | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | exp `1` |
| 30 | parse_program2_ws_or_min | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | exp `1` |
| 31 | parse_try_program_block_min | 0 | `[ab` | named-reject | `route-not-front-selected LoopTrueRouteRejected(SourceItemOutsideLoop)` site=Body(3) | rejected | exp `[ab` on compat lane |
| 32 | parse_program2_cond_and_min | 1 | VM err `canonical-call` | named-reject | `qualified-preflight actual=Loop` site=Body(3) | rejected | exp `1` |
| 33 | parse_program2_effect_if_min | 1 | VM err `canonical-call` | named-reject | `route-not-front-selected CarrierRelation(MissingIncrement)` site=Body(2) | rejected | exp `P` |
| 34 | parse_program2_bool_or_mod_min | 0 | `1` | named-reject | `route-not-front-selected LoopCondRouteRejected(SourceItemsMissing)` site=Body(4) | rejected | exp `1` on compat lane |
| 35 | parse_program2_loop_min | 0 | `1` | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `1` on compat lane |
| 36 | loop_if_fallthrough_join_min | 0 | `1` | named-reject | `route-not-front-selected LoopCondRouteRejected(SourceItemsMissing)` site=Body(1) | rejected | exp `1` on compat lane |
| 37 | loop_if_return_min | 0 | (empty) | named-reject | `route-not-front-selected GenericLoopV1NotSelected` site=Body(1) | rejected | exp `__EMPTY__` |
| 38 | loop_if_return_var_min | 0 | (empty) | named-reject | `route-not-front-selected GenericLoopV1NotSelected` site=Body(1) | rejected | exp `__EMPTY__` |
| 39 | loop_if_return_local_min | 0 | (empty) | canonical-main | `define i64 @main()` | accepted | exp `__EMPTY__` |
| 40 | loop_continue_if_min | 0 | `1` | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `1` on compat lane |
| 41 | loop_if_else_return_min | 0 | (empty) | named-reject | `callable-loop-handoff/carrier-cardinality` | rejected | exp `__EMPTY__` |
| 42 | loop_if_else_return_var_min | 0 | (empty) | named-reject | `carrier-cardinality` | rejected | exp `__EMPTY__` |
| 43 | loop_if_else_return_local_min | 0 | (empty) | named-reject | `carrier-cardinality` | rejected | exp `__EMPTY__` |
| 44 | loop_if_else_if_return_min | 0 | (empty) | named-reject | `carrier-cardinality` | rejected | exp `__EMPTY__` |
| 45 | loop_if_else_if_else_return_min | 0 | (empty) | named-reject | `carrier-cardinality` | rejected | exp `__EMPTY__` |
| 46 | nested_loop_if_return_min | 0 | (empty) | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `__EMPTY__` |
| 47 | nested_loop_if_else_return_min | 0 | (empty) | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `__EMPTY__` |
| 48 | nested_loop_if_return_var_min | 0 | (empty) | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `__EMPTY__` |
| 49 | nested_loop_if_return_local_min | 0 | (empty) | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `__EMPTY__` |
| 50 | nested_loop_if_else_return_var_min | 0 | (empty) | named-reject | `nested-loop-profile-not-admitted` | rejected | exp `__EMPTY__` |

## Result

- 9/45 `accepted`: `if_return{,_var,_local}`, `if_fallthrough_join`,
  `if_else_return{,_var,_local}`, `if_else_if_return`, and
  `loop_if_return_local` reach canonical `define i64 @main()`; all ran
  green to expected compat-lane output.
- 28/45 `rejected`: `callable-loop-handoff` (`nested-loop-profile-not-
  admitted` 8, `carrier-cardinality` 5), qualified preflight `Loop`
  statements (5), `route-not-front-selected` (`GenericLoopV1NotSelected`
  3, `LoopCondRouteRejected` 2, `LoopTrueRouteRejected` 1,
  `CarrierRelation` 1), `callable-loop/recipe
  source-unsupported-body-statement` via composite/source-target (1),
  and semantic-package `LoopBreakSource` issuers (2).
- 8/45 `failed-before-loop`: `mir/main-import-view/selected-header-
  missing` (3), preflight `Print` (1), `MainMustBeStatic` (1),
  `CoreMethodSource NamedArray(TextSourceMissing)` (1),
  `ResolverDeferred UnsupportedStatement TryCatch` (1), and
  `callable-semantic-package/install
  MapObligationDescribe(OwnerTerminalHomesUnavailable)` (1).
- 0 `timeout`, 0 left `unobserved`.
- **Nondeterminism first recorded here**: `if_else_return_min` and
  `if_else_return_var_min` intermittently emit `define void @main()`
  (legacy callable emission with a value `ret`) instead of `define i64
  @main()` — observed ~1/4 runs on repeat checks; all other accepted
  fixtures were stable across 4 reruns. The canonical admit and the
  silent legacy emission coexist for the same fixture; recorded as
  front evidence, not repaired or classified as a new arm.
- `map_literal_percent` fails A-side inside the VM interpreter on
  `NewBox IntrinsicMap` (unimplemented instruction) — a compat-lane
  interp limit, distinct from the `canonical-call` terminal.
- Compat-lane context: 29/45 ran green to expected output (subset
  expected values match the fast-gate twins where fixtures overlap).
- No fixture, source, corpus-universe, receipt, or route change was
  made. `observation_state` is the only manifest column touched.
