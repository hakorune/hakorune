# MIRBUILDER-EXE-ACCEPTANCE-DEAD-ROUTE-SOLE-FAMILY-S1

Status: landed__2026-09-26
Date: 2026-09-26
Parent: MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D13 (accepted)
Owner: workstream row H / unified resume gate 1
Authority: D13 Decision. `LoopBreakRecipe` is TypedDeclined
  wire vocabulary (its plan-lane pipeline retired) — an
  owner-less route must not suppress the designed
  `LoopCondBreakContinue{NoExitBody}` route or contend for
  `sole_family`.

## Slice

1. `src/mir/builder/control_flow/joinir/route_entry/registry/
   predicates.rs`: drop `!pred_loop_break_recipe` from
   `pred_loop_cond_break_continue` and
   `pred_loop_true_break_continue` — the suppression was
   authored for the retired plan-lane LoopBreak pipeline;
   break-bearing loops are consumed upstream by the
   LoopBreak source-candidate lane today.
2. `src/mir/builder/normal_callable_loop_source_route.rs`
   `sole_family()`: treat `LoopBreakRecipe` matched rows as
   diagnostic-only when counting the surviving family —
   exactly one of {LoopCondBreakContinue, LoopTrueBreakContinue}
   still required. `LoopSimpleWhile` (V1Parity) and other
   route IDs keep contending (the no-exit S0's recorded
   `matched=[LoopSimpleWhile, LoopCond]` → node-route
   boundary is preserved).

## Overlap analysis (required, source-read)

- `matched=[LoopBreakRecipe]` alone: sole_family stays
  `None` → `NonGenericOrOverlapping` → node route (unchanged).
- `matched=[LoopBreakRecipe, LoopCondBreakContinue]`
  (trim-header loops): previously `None` → spine decline;
  now `Some(LoopCondBreakContinue)` → `loop_cond::issue` →
  `lower_loop_cond_break_continue_source` — the designed
  no-exit owner (no-exit S0, landed 2026-09-25).
- `matched=[LoopSimpleWhile, LoopCondBreakContinue]`
  (`loop(i<literal){i=i+1}`): still `None` → node route —
  S0's zero-behavior-change boundary kept.
- Break-bearing loops reaching the generic issuer are rare
  (direct/composite LoopBreak candidates are consumed
  upstream; `RootExitMissing` is the only tolerated skip).
  A break-bearing loop whose LoopCond base facts matched now
  front-selects LoopCond — LoopCond's native vocabulary is
  break/continue items, so the owner is legitimate.
- `matched` keeps `LoopBreakRecipe` for diagnostics
  (`NonGenericOrOverlapping{routes}`); only `sole_family`
  contention is filtered.

## Pins (required before close)

- Positive: `loop(i<n && (s.substring==" "||...)){i=i+1}`
  shape (trim-header class): `matched` contains
  `LoopCondBreakContinue` and `sole_family()` returns
  `Some(LoopCondBreakContinue)` when `loop_break` subset
  facts also matched.
- Positive unit: facts with `loop_break` +
  `loop_cond_break_continue` → `pred_loop_cond_break_continue`
  true; `sole_family()` = `LoopCondBreakContinue`.
- Negative: `matched=[LoopSimpleWhile, LoopCondBreakContinue]`
  → `sole_family()` `None`; `matched=[LoopBreakRecipe]` →
  `None`.
- Guard: extend `mirbuilder_qualified_route_scope_guard.sh`
  — pin the `sole_family` `LoopBreakRecipe` exemption and
  the removed suppression comments; register touched files.
- Real app: json_stream_aggregator EXE advances past
  `StringHelpers.trim/1` winner decline — record the next
  honest terminal.

## Fail-fast boundary

- `ConditionalUpdateIf` items (ingest) stay the F2 parts
  boundary — unchanged.
- `ingest`'s VM ledger-less lane unchanged (route_loop
  spine → typed `Declined`).
- Exit-free loops whose facts yield zero front-selectable
  matches still reach `NonGenericOrOverlapping` → spine.

## Evidence (landed — filled at close)

- `predicates.rs`: `!pred_loop_break_recipe` dropped from
  `pred_loop_cond_break_continue` and
  `pred_loop_true_break_continue`; provenance comment
  retained. `sole_family()` filters only
  `LoopRouteId::LoopBreakRecipe`.
- Focused tests (new
  `normal_callable_loop_source_route_dead_route_tests.rs`,
  130 lines): 23/23 green in the route module —
  `sole_family_ignores_owner_less_loop_break_recipe`
  ([LBR,LCC]→LCC, [LBR,LTC]→LTC, [LBR]→None),
  `loop_simple_while_overlap_keeps_sole_family_none`
  ([LSW,LCC]→None — S0 node-route boundary preserved),
  `trim_header_dead_route_does_not_hide_loop_cond_sole_family`
  (real planner facts: `matched` keeps `LoopBreakRecipe`
  provenance AND `LoopCondBreakContinue`, sole = LCC).
- Guards green: `current_state_pointer_guard.sh`,
  `mirbuilder_qualified_route_scope_guard.sh` (S1 section
  pins the removed suppression + filter + test names;
  `ROUTE_PREDICATES`/`CALLABLE_ROUTE`/`DEAD_ROUTE_TESTS`
  registered in the 800-line list). `rustfmt --check` clean
  on touched files; tests file kept under the 800-line hard
  stop by the new sibling test module.
- EXE smoke (`NYASH_BIN=target/debug/hakorune`,
  `json_stream_aggregator_exe_runtime_boundary.sh`):
  `StringHelpers.trim/1` no longer declines at winner
  selection — it front-selects `LoopCondBreakContinue` and
  reaches the honest next terminal
  `[freeze:contract][callable-loop/route-not-front-selected]
  LoopCondRouteRejected(SourceCallOutsideSelectedFamily)`
  with the two `s.substring` call sites inside
  `LoopCondition` (the inventoried D5 call-coverage
  boundary — separate authority, not a loop-owner failure).
- VM smoke (`target/debug/hakorune --backend vm`):
  unchanged known terminal — `loop winner selection
  declined: zero selected family candidates
  fn=JsonStreamAggregator.ingest/1` (ledger-less instance
  method → route_loop spine; recorded non-claim).

## Non-claims

- Does not fix `ingest` ConditionalUpdateIf (F2) or its VM
  ledger absence.
- Does not retire `LoopBreakFacts` subset extractors or the
  `LoopBreakRecipe` route ID — they remain diagnostics/
  provenance.
- Does not generalize sole-family filtering beyond
  `LoopBreakRecipe`; other TypedDeclined routes keep
  contending until their own census.
- Does not claim `trim` or json_stream_aggregator green —
  condition call-coverage (D5 family) is the expected next
  honest terminal.
