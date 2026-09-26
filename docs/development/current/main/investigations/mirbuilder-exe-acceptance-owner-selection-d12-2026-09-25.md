# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D12

Status: closed__2026-09-26 (S0 landed)
Date: 2026-09-25
Parent: workstream row H (unified resume, gate 1)
Mode: design_stop — census and Decision only.

## Blocking observation

```text
[freeze:contract] loop winner selection declined: zero selected
family candidates
```

`json_stream_aggregator` advanced past
`callable-semantic-lowering/placement-local-missing` (nested
`{ }` local now resolves its `locals` row) and now stops at the
next honest terminal: `issue_loop_node_winner_recipe_v1` returns
`Declined` for a walked loop in
`joinir/route_entry/router.rs:198-205`.

## Known context

- The router establishes exact loop membership
  (`resolver loop inventory` match on condition+body structure)
  before winner selection — the freeze fires AFTER membership
  succeeded, i.e. the walked loop IS in the inventory but no
  selected family candidate admits it.
- Loop family admission is decided by `issue_loop_node_winner_
  recipe_v1` — the one authority for family winner selection on
  this lane. `Declined` = zero selected candidates; distinct
  from `Unresolved`/`Rejected` (spine terminal).
- The app's loop lives in `apps/json-stream-aggregator/main.hako`
  (and/or imported `StringHelpers` helpers) — shape unknown
  until identified.

## Census questions

1. Which exact loop statement declines (function + site + loop
   condition/body shape)?
2. Which family candidates were probed, and which named reject
   reason did each produce? (family reject vocabulary, not a
   guess)
3. Is the shape inside an existing family's bounded vocabulary
   (a missing admission edge) or genuinely unclaimed shape
   (NoSafeSlice → design)?
4. Existing owner for the shape — is there a designed family
   admission path the walked loop fails to reach, or is a new
   bounded family edge needed?

## Bounded surface

Read-only census across `issue_loop_node_winner_recipe_v1`
family candidates, the per-family reject vocabulary, and the
resolver loop inventory for the failing function. Does not
touch the nested-program item-site producer (landed), the env
route (landed), or static-result ingress.

## Census results

1. **Failing loop identified** (temporary instrumentation,
   reverted): `JsonStreamAggregator.ingest/1` — instance method of
   `box JsonStreamAggregator` — `loop(start < n)` at
   `[Body(2)]`, condition `BinaryOp{Less, Variable(start),
   Variable(n)}`. Body: `{local end = JsonLine.find(...); if end <
   0 { end = n }; me.ingestLine(...); start = end + 1}` — a
   general `loop(cond)` with a 4-statement no-exit body.
2. **All five spine families decline** (`issue_loop_node_winner_
   recipe_v1` rows, observed):
   `DirectAccum=NotDirectAccumShape`,
   `NestedPredicate=NotNestedPredicateShape`,
   `LoopTrue=NotLoopTrueBreakContinueShape` (condition is not
   literal `true`), `LoopCond=NotLoopCondBreakContinueShape`
   (`BodyArity` — the projection requires exactly
   `loop(cond){if{break}else{continue}}`),
   `GenericG0=NotGenericG0Shape` (`FunctionBodySchedule` — G0 is a
   nested 2-loop canon: 2-stmt fn body, outer loop + tail
   return, not a general loop family).
3. **Lane path**: `ingest`'s loop DID reach
   `RawInvocationChildPortV1::lower_loop` (located port) but
   `callable_ledger` is **absent** — instance methods are not
   ledger-bound → `callable_handoff=None` →
   `lower_non_callable_loop_route_v1` → `lower_loop_or_freeze_v1`
   → `route_loop` → winner spine → `Declined`. The
   `RouteNotFrontSelected` arm was never reached (the callable
   facts issuer only runs for ledger-bound loops).
4. **Designed owner on the callable lane exists**: general
   `loop(cond)` no-exit bodies are admitted on the callable lane
   via `LoopCondBreakContinueFacts::NoExitBody`
   (`try_extract_loop_cond_no_exit_facts` → sole
   `LoopCondBreakContinue` family → `lower_loop_cond_break_
   continue_source` — the ConditionalUpdateIf/no-exit item work
   landed exactly for this lane). But that lane requires a
   `callable_ledger`, which `ingest` lacks. On the winner-spine
   lane there is no no-exit/general loop-cond vocabulary at all
   (no `NoExit` token under `src/mir/compiler/`,
   `loop_recipe_contract/`, `loop_structural_facts/`,
   `loop_route_policy/`).

5. **Correction — the actual first blocker is NOT `ingest`**:
   with compat-reason instrumentation the smoke's first decline
   is `StringHelpers.starts_with/3` `loop(k < m)` at `[Body(5)]`
   (`compilation:2` — the imported `using` unit compiles before
   the app). Body: `{if src.substring(..) != pat.substring(..)
   { return 0 }; k = k + 1}` — the `simple_if_return_then_step`
   return-in-body shape: one `return`, zero break/continue.
   Unlike `ingest`, `starts_with` IS ledger-bound: the callable
   route match reports `RouteNotFrontSelected
   matched=[LoopCondReturnInBody]` — the canonical-facts
   predicate admits the shape (observation only), but
   `CallableLoopSoleFamilyV1` covers only
   LoopCondBreakContinue/LoopTrueBreakContinue, so the route is
   not front-selected and dispatches to `route_loop` → winner
   spine → the five families decline (as row 2).
6. **The composite LoopBreak lane already carries Return
   end-to-end**; one admission gate excludes it:
   - resolver: `ResolvedExitOriginV1::ExplicitReturn`,
     `ResolvedControlTransferV1::Return{target_function}` are
     recorded per source site (records.rs);
   - forest: `issue_loop_cond_break_continue_source_forest_
     projection_v1` collects `function.resolved_exits()` under
     the root — the `return` is already in `forest.exits()`;
   - body roles: `issue_statement_role` maps
     `Break|Continue|Return` stmts to `Exit{record}` and accepts
     `Return{..}` transfers unconditionally
     (`loop_break_composite_body_role.rs:418-443`);
   - recipe: `build_body_block` emits `ExitKind::Return` and the
     `if{return}` becomes `IfV2{ExitOnly{ExitIf}}`; loop root is
     `LoopV0{WhileLike, body_contract: ExitAllowed}`
     (`composite.rs:140-153,246-275`);
   - physical: `(ExitAllowed, OpaqueExit)` → `lower_opaque_exit`
     → `lower_loop_cond_exit_source_input` →
     `CoreExitPlan::Return` (`parts/exit.rs:98-105`).
   - the sole blocker: `validate_exit_ledger` requires
     `root_break_count >= 1` — `return` is not counted →
     `RootBreakMissing` → tolerated as "no composite candidate"
     (`loop_break.rs:665-667`) → the loop falls to the spine.
7. **`starts_with`'s direct-lane escalation is correct**:
   `issue_loop_break_source_projection_v1` fails with an
   `unsupported_shape` reason (no break-if), so the designed
   composite fallback is attempted — and rejected only by the
   root-break gate.
8. **Companion loops**: `JsonLine.intField`'s
   `loop(end < n){local ch; if ch<"0"||ch>"9"{break}; end=end+1}`
   has a root break → composite-admitted today.
   `StringHelpers.index_of`'s `loop(pos+m<=n){if{..;if{return
   pos}}else{..}}` has a return → admitted by this slice.
   `ingest` (ledger-less + exit-free) and `StringHelpers.trim`'s
   `loop(cond){i=i+1}` (exit-free) carry **no explicit exit
   evidence** → still `RootExitMissing` → spine → typed
   `Declined` — a separate ownership question for the next
   card, not widened here.

## Decision

```text
Decision: admit `loop(cond){…return…}` (return-in-body, no
  break/continue) into the existing composite LoopBreak lane by
  counting `Return{target_function}` exits as root control
  evidence in `validate_exit_ledger` (`RootBreakMissing`
  renamed `RootExitMissing`).
Source authority + canonical issuer:
  `issue_loop_break_composite_source_projection_v1` (resolver
  exit ledger, compiler) → `issue_composite_source_candidate_v1`
  (builder Facts/Recipe co-seal) → `lower_loop_break_composite_
  source` (sole physical owner, unchanged).
Non-authority: `CanonicalLoopFacts`/`LoopCondReturnInBody`
  predicates (provenance only — not a dispatch key);
  `CallableLoopSoleFamilyV1` (unchanged 2-family vocabulary);
  winner spine (unchanged 5 families); retired
  `features/*_pipeline`/`loop_cond_unified` (stay deleted).
Fail-fast boundary: roots with zero explicit exits (no
  root-Break AND no Return) still fail `RootExitMissing` → no
  candidate → spine → typed `Declined`; continue-only loops are
  NOT admitted (unverified physically).
Smallest next slice: extend `validate_exit_ledger` Break|Return
  counting + tolerate arm + positive fixture (return-in-body
  projection/candidate) + negative (exit-free →
  `RootExitMissing`) + guard pins; then re-run the app smoke to
  expose the next honest terminal (expected: `ingest`/`trim`
  exit-free loops → next card).
Non-claims: does not admit continue-only or exit-free loops;
  does not fix `ingest`'s ledger absence (separate ownership
  question); does not extend `sole_family` or spine vocabulary;
  does not resurrect the retired ReturnInBody pipeline — the
  composite lane IS the one physical owner.
```

## Exit

- [x] Failing loop identified; family reject reasons enumerated.
- [x] One bounded S-card emitted, or NoSafeSlice recorded.

## Close

S0 landed 2026-09-26 — `validate_exit_ledger` counts
`Return{target_function}` as root exit evidence
(`RootBreakMissing`→`RootExitMissing`); `starts_with/3`
return-in-body loop lowers through the existing composite
LoopBreak owner. Observed next terminals (debug binary):
EXE `fn=StringHelpers.trim/1` exit-free `loop(i<n && …)`,
VM `fn=JsonStreamAggregator.ingest/1` exit-free
`loop(start<n)` — both inside this card's non-claim boundary;
exit-free `loop(cond)` ownership is the next census
(OWNER-SELECTION-D13).
