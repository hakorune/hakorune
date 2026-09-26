# MIRBUILDER-EXE-ACCEPTANCE-SUITE-RED-DISPOSITION-D21
## Census: post-S8 Gate-1 suite red disposition (D21)

Status: decided — no in-lane bounded slice; all reds owned by existing
  sealed/parked lineages or the confirmed environment/toolchain debt
Date: 2026-09-27
Parent: MIRBUILDER-EXE-ACCEPTANCE-LOOP-CARRIER-BINDING-PUBLICATION-S8
  (landed — carrier header/final phis publish into
  `CallableSemanticLoweringState.values[binding]`; emit-mir-json green
  with 65 retained phis; `find/3` wrong-code closed)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D20 selection lineage.

## Problem

S8's named next blocker was `JsonStreamAggregator.ingest/1`'s
`loop(start < n)` declining every loop-family candidate
(`loop winner selection declined: zero selected family candidates`),
and the full `real-apps-exe-boundary` receipt needed re-classification
after S8. Census question: does any remaining Gate-1 failure sit inside
this lane's authority (callable source route -> document publication ->
MIR emission), and is there a bounded next slice?

## Fresh suite receipt (post-S8, release build)

`tools/smokes/v2/run.sh --suite real-apps-exe-boundary` — 4 pass /
7 fail:

| entry | result | first named terminal |
|---|---|---|
| typed_object_birth_min | PASS | — |
| typed_object_method_min | PASS | — |
| typed_object_birth_param_min | PASS | — |
| real_apps_exe_boundary_probe | PASS | — |
| typed_object_newbox_min | FAIL | backend `no_lowering_variant` |
| typed_object_untyped_field | FAIL | MIR emit `unsupported terminator Invoke` |
| boxtorrent_mini | FAIL | `callable-loop/route-not-front-selected` `LoopCondRouteRejected(SourceCallOutsideSelectedFamily)` `HakoAllocPage.seedBlocks/0` |
| binary_trees | FAIL | `SourceCallOutsideSelectedFamily` `BinaryTreesBench.iterationCheck/3` |
| mimalloc_lite | FAIL | `SourceCallOutsideSelectedFamily` `HakoAllocPage.seedBlocks/0` |
| allocator_stress | FAIL | `CoreMethodSource NamedArray(TextSourceMissing)` |
| json_stream_aggregator | FAIL | ny-llvmc `unsupported_newbox_type` at `new JsonStreamAggregator` (first_block=0 first_inst=3) |

## Census answers

### Q1 — which lane declines `ingest/1`, and is it a Gate-1 defect?

The decline is on the **ledger-free lane**, not the armed EXE lane.
`--backend vm` enters `VmHakoPostMacro` admission
(`compile_bridge.rs`) which builds the `Compatibility(ast)` program
root — no callable ledger is installed, so the loop lowers through
`lower_non_callable_loop_route_v1` -> `route_loop` -> winner spine
(`issue_loop_node_winner_recipe_v1` at router.rs:198 ->
`select_canonical_loop_family_v1` called at
`loop_node_winner_spine.rs:370`, defined at
`family_selector.rs:147`). All five spine
families contract-decline this shape (single `loop(cond)` with a
four-statement body holding a local decl, a nested if, an instance
call, and the carrier update):

| family | outcome |
|---|---|
| DirectAccum | `NotDirectAccumShape` |
| NestedPredicate | `NotNestedPredicateShape` |
| LoopTrue | `NotLoopTrueBreakContinueShape` |
| LoopCond | `BodyArity` |
| GenericG0 | `Structural(FunctionBodySchedule)` — nested two-loop profile only per generic-loop SSOT |

The armed EXE route (`--emit-mir-json` ->
`for_mir_mode_callable_source` -> callable ledger installed) compiles
`ingest/1` fully — S8 evidence: the module emits with 65 retained
phis and `ingest`'s phi spine is present in the MIR JSON. The
ledger-free decline is the named "non-callable Script owner"
compatibility boundary (final-pipeline SSOT: `Callable None` retains
its existing non-callable owner; `ParkedSealed__NoCallableNoneCaller`
governs branch retention) and was already a D17 recorded non-claim:
"`ingest/1` may later still be declined by the unchanged ledger-less
VM spine (Recorded Non-Claim — Gates 2-4)". It is not a Gate-1
blocker.

### Q2 — post-S8 red map: who owns each of the seven?

Zero unowned failures. Every red terminates at a named typed stop
inside an existing sealed/parked lineage — except `newbox_min`,
which is the D0/D1-confirmed environment/toolchain debt:

| app | terminal | owning record |
|---|---|---|
| boxtorrent_mini | `SourceCallOutsideSelectedFamily` (`free_stack.push` on field-read local) | D5 fork (a): B3-ArrayPush lineage — `NoSafeSlice__B3SelectedRuntimeAndOperationOutcomeAuthorityMissing` |
| binary_trees | `SourceCallOutsideSelectedFamily` (`builder.make`, `positive.itemCheck` on param/local receivers) | D5 fork (b): ordinary user-box instance calls — no coverage issuer exists; parked DeclaredInstance lineage |
| mimalloc_lite | `SourceCallOutsideSelectedFamily` (`seedBlocks`) | D5 fork (a) |
| allocator_stress | `CoreMethodSource NamedArray(TextSourceMissing)` | D5-inventoried NamedArray source-demand family (B3 vocabulary) |
| typed_object_newbox_min | `no_lowering_variant` | environment/toolchain debt — resolved `opt` = LLVM 14.0.0 while `opt-18` (18.1.8) exists at `/usr/lib/llvm-18/bin/opt`; D0 recorded `environment/toolchain debt (resolved opt too old)`, D1 confirmed `newbox_min = EnvironmentDebt`: verify with `opt-18` before treating as semantic work |
| typed_object_untyped_field | `unsupported terminator Invoke` | D0 `NoSafeSlice` untyped-storage sentinel |
| json_stream_aggregator | `unsupported_newbox_type` (`new JsonStreamAggregator` — handle-typed fields, construction-ineligible per D18) | D0/D18 ordinary-new eligibility family: "box-typed fields, nested construction reclamation ... its own family" |

The callable loop route's coverage contract
(`into_selected_relation`) admits exactly two issuers —
singleton `SelectedStatic` exact targets and `CoreMethod` rows
(StringBox ops on lexical-local receivers in loops, plus
`ArrayBox.push` only under `NamedArrayConstructionRequirementV1`).
Uncovered call kinds (param/local-receiver instance calls,
non-StringBox core methods outside NamedArray scope) are
deliberately outside the selected family; the freeze is the
designed fail-fast, and widening coverage is each fork's own card —
D5 already sealed both forks family-local: "reopen only through
their own cards".

### Q3 — bounded in-lane slice?

None. Gate-1's lane authority covers the callable source route
through document publication and MIR emission; every remaining red
stops either (a) inside a sealed coverage fork the route contract
names by design, (b) past MIR emission at backend-cohort newbox
admission (`unsupported_newbox_type`), (c) in the untyped-storage
`NoSafeSlice` sentinel (`unsupported terminator Invoke`), or
(d) at the D0/D1-confirmed environment/toolchain debt
(`no_lowering_variant` — stale resolved `opt`). Per the family
scheduler the in-lane inventory is exhausted; the reds reopen
through their owning cards or the toolchain fix, not here.

## Decision

```text
Decision: no bounded slice exists in the Gate-1 failure set — the
  ingest/1 family-selection decline is the D17-recorded ledger-free
  compatibility boundary (non-callable Script owner), and all seven
  EXE reds are owned by existing sealed/parked lineages or the
  confirmed environment/toolchain debt. Gate-1 stays
  externally-blocked on those records' own reopen triggers.
Source authority + canonical issuer: suite red map -> D5-sealed
  coverage forks (B3-ArrayPush, parked DeclaredInstance),
  D5-inventoried NamedArray source demand, D0/D18 handle-field
  newbox-admission family, and the D0/D1 environment/toolchain debt
  for newbox_min; VM spine decline -> ledger-free VmHakoPostMacro/
  Compatibility owner per the final-pipeline SSOT.
Non-authority: this lane does not reopen B3/DeclaredInstance/
  NamedArray or newbox-admission cards, does not extend the loop
  coverage contract, does not weaken family selection, does not
  reclassify the newbox_min toolchain debt as semantic work, and
  does not cross VM/EXE lanes without a production caller census.
Fail-fast boundary: SourceCallOutsideSelectedFamily,
  NamedArray(TextSourceMissing), no_lowering_variant,
  unsupported terminator Invoke, unsupported_newbox_type, and the
  winner-spine NoCandidate decline all remain named terminals —
  no fallback, no silent no-op.
Smallest next slice: none in-lane. Reopen only through the owning
  records: B3-D2 (ArrayPush + NamedArray authorities), parked
  DeclaredInstance admission, D0 handle-field newbox admission
  (unsupported_newbox_type only), toolchain verification with
  opt-18 for newbox_min per D1; a ledger-free single-loop family
  is that spine's own card.
Non-claims: gate-1 stays unsatisfied (4/11 — 3 apps + probe);
  no VM claim for ingest/1; no newbox admission claim; the sealed
  forks' reopen conditions are not asserted met; overall
  MirBuilder is not complete.
```

## Boundary

- Includes: post-S8 `real-apps-exe-boundary` receipt classification;
  the `ingest/1` decline lane attribution; owner mapping to existing
  cards; the sealed disposition with reopen triggers.
- Excludes: implementation; reopening D5/B3/DeclaredInstance/D0/D17/
  D18 decisions; backend toolchain work; Gates 2-4.

## Exit

- [x] Census recorded: zero unowned failures in the post-S8 Gate-1
      receipt; VM decline attributed to the ledger-free lane.
- [x] Named sealed disposition with observable reopen triggers;
      no bounded slice emitted.
- [x] Pointers and workstream row H synced; guard re-run.
