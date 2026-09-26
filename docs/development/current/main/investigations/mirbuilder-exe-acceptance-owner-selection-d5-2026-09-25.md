# MIRBUILDER-EXE-ACCEPTANCE-OWNER-SELECTION-D5

Status: decision__2026-09-25
Date: 2026-09-25
Parent: MIRBUILDER-EXE-ACCEPTANCE-LOOP-COND-NO-EXIT-S0
  (landed — disjoint `loop_cond_no_exit` extractor feeding
  `LoopCondBreakContinueFacts` with `NoExitBody`)
Owner: workstream row H / unified resume gate 1
Authority: mirbuilder-final-pipeline-ssot.md acceptance table;
  D0–D4 selection lineage.

## Problem

LOOP-COND-NO-EXIT-S0 landed: all three `callable-loop/facts-absent`
entries cleared and now stop at
`[freeze:contract][callable-loop/route-not-front-selected]
LoopCondRouteRejected(SourceCallOutsideSelectedFamily)`. Select the
next bounded design slice from the remaining 7 failing real-app EXE
entries.

## Fresh class map (receipt after LOOP-COND-NO-EXIT-S0)

| class | entries | first named stop |
|---|---|---|
| `callable-loop/route-not-front-selected SourceCallOutsideSelectedFamily` | 3 | boxtorrent_mini (`HakoAllocPage.seedBlocks/0`, imported), binary_trees (`BinaryTreesBench.iterationCheck/3`), mimalloc_lite (`seedBlocks`) |
| `callable-loop/parts loop-cond-item-unsupported` | 1 | json_stream_aggregator (`ConditionalUpdateIf` in `JsonStreamAggregator.ingest`, main.hako:145) |
| `CoreMethodSource/NamedArray(TextSourceMissing)` | 1 | allocator_stress |
| backend `no_lowering_variant` (toolchain) | 1 | typed_object_newbox_min |
| inference panic `return_type_strategy` | 1 | typed_object_untyped_field_min |

`SourceCallOutsideSelectedFamily` fires from
`normal_callable_loop_source_route_items.rs::into_selected_relation`
— every resolver method-call item under the loop must be covered by
either one exact same-module static "selected" publication relation
(singleton contract) or resolver-issued CoreMethod rows covering ALL
items. Loop-internal calls in the failing bodies are instance or
builtin-box calls (`free_stack.push`, `builder.make`,
`positive.itemCheck()`, `me.bump`) that carry neither row.

## Observed coverage sub-boundaries (main-thread probes)

- call-free body `loop(i<n){i=i+1}` in a callable →
  `SourceItemsMissing`
  (`normal_callable_loop_source_route.rs:257` requires ≥1 item).
- instance/builtin-only body (`me.bump(i)`) →
  `SourceCallOutsideSelectedFamily`.
- singleton-covered body (`acc = acc + Step.twice(i)`) → route
  issues, lowering runs, then atomic commit fails at
  `StaticResultPublicationResidual(UnconsumedSelected)` — the
  publication handoff for loop-internal static calls is only
  pre-taken when `has_loop_break_composite_source_candidate(site)`
  (`raw_loop_child_port.rs:88-95`), so non-composite loops never
  install the row into the callable ledger for the body emitter.

## Questions

1. Do the three `SourceCallOutsideSelectedFamily` entries share one
   coverage-authority gap (instance/builtin method calls inside loop
   bodies hold no issued coverage row), or do they fork (ArrayBox
   builtin vs user-box instance receiver)?
2. Is the singleton-plus-anchor contract (`into_selected_relation`
   accepts one `selected` and does not verify remaining items) the
   intended coverage semantic, or is per-item coverage the contract
   the failing loops should satisfy? json reached the parts stage only
   because `JsonLine.find` supplied the anchor — `me.ingestLine` and
   `stream.substring` were never verified.
3. Which class has a bounded slice: single owner + fail-fast tuple +
   acceptance coverage? Candidates:
   - F3a: extend loop-source item coverage so instance/CoreMethod
     rows issued for the body satisfy `into_selected_relation`
     (route-contract layer).
   - F3b: publication consumption for loop-internal static calls
     when no composite loop-break candidate exists
     (`consume_publication` gating).
   - F3c: `SourceItemsMissing` for call-free no-exit bodies
     (vacuous-coverage semantics).
   - F2 (already inventoried): `ConditionalUpdateIf` arm in the
     located-source parts driver — unblocks json but does not move
     the three `SourceCallOutsideSelectedFamily` entries.

## Boundary

- Includes: owner census for the 3-entry
  `SourceCallOutsideSelectedFamily` class; divergence check vs
  `SourceItemsMissing` and `UnconsumedSelected`; whether the
  singleton anchor is the intended contract; one Decision with a
  six-line brief or NoSafeSlice naming the highest-information class.
- Excludes: implementation; backend toolchain; inference panic
  family; NamedArray source-demand family; F2 implementation
  (inventoried, stays next-in-line after the coverage decision).

## Exit

- [x] Decision recorded (bounded slice OR sealed NoSafeSlice with
      reopen triggers) with the six-line brief.
- [x] Next S-card emitted (`LOOP-COND-COND-UPDATE-S0`, landed);
      pointers synced.

## Census answers (read-only worker + main-thread probes)

### Q1 — do the three `SourceCallOutsideSelectedFamily` entries share one family?

No — one terminal, two coverage-authority forks.

The item inventory is universal: `CallableLoopSourceBridgeV1::from_input`
copies every resolver `method_calls()` row under each armed root loop
(`source_loop_bridge.rs:94-105`), and the ledger records calls of every
receiver kind. Coverage, however, has exactly two issuers today:

- `SelectedStatic`: exact same-module static calls with an unconsumed
  publication handoff (`raw_loop_child_port.rs:159-233`).
- `CoreMethod`: `StringBox` `len/0` (Condition) and `substring/2` (Body)
  on lexical-local receivers (`core_method.rs:66-160`), plus
  `ArrayBox.push/1` (Body) only when `NamedArrayConstructionRequirementV1`
  issues — in-scope `new ArrayBox()` construction, statement position,
  Text argument (`named_array_method.rs`, `named_array_requirement.rs`).

Observed forks:

- `seedBlocks` (boxtorrent_mini, mimalloc_lite): `free_stack.push(i)` is
  a `push/1` on a local bound to `me.free_stack` — a field-read
  initializer (`ConstructionMissing`) with an `i64` argument
  (`TextSourceMissing`). The B3 lineage
  (`array-push-b3-loopcond-carrier-relation-d2`,
  `NoSafeSlice__B3SelectedRuntimeAndOperationOutcomeAuthorityMissing`)
  owns ArrayPush-in-LoopCond coverage and is already sealed pending the
  operation-outcome authority; this sub-shape is outside even that
  issuer's vocabulary.
- `iterationCheck` (binary_trees): `builder.make` (parameter receiver),
  `positive.itemCheck()` / `negative.itemCheck()` (local receivers) are
  ordinary user-box instance calls. `VerifiedDeclaredInstanceCallRelationV1`
  issues only for `me.`-receiver calls inside InstanceBoxMethod owners
  and is consumed solely by the raw-lane `CanonicalInstance` ingress —
  it never reaches the loop item-coverage contract. No coverage issuer
  exists for ordinary instance calls.

### Q2 — is the singleton-plus-anchor contract the intended semantic?

Yes for the direct route — `into_selected_relation`
(`normal_callable_loop_source_route_items.rs:292-345`) deliberately
returns the single selected relation without verifying the remaining
items; `SourceCallOutsideSelectedFamily`'s own doc names it "the
unsupported family terminal, not a missing-evidence one". Per-item
coverage (`into_item_dispositions`) is consumed only by the LoopBreak
composite arm (`composite_physical.rs:77-81`); the singleton LoopCond
route has no per-item consumer. Physical emitters for all observed call
kinds exist in the Raw port (`CanonicalInstance`,
`CoreEffectPlan::MethodCall`, `take_source_array_push`,
`lower_selected_static_result_publication_v1`) — the route contract is
the gate, not emission capability.

`consume_publication` (`raw_loop_child_port.rs:88-95`) is armed only by
`has_loop_break_composite_source_candidate`, which the LoopBreak
composite topology issuer alone produces; cataloged instance methods
receive `loop_break_take: None`
(`cataloged_instance_scope.rs:56`). On the singleton route the anchored
static call's handoff is therefore never installed into the callable
ledger — the Raw port's `SourcePublication` arm finds nothing
(`take_source_static_result_publication` → `None` → `emit_raw_effect`)
and the row drains as `StaticResultPublicationResidual(UnconsumedSelected)`.

### Q3 — bounded slice selection

Family disposition for the dominant class: `SourceCallOutsideSelectedFamily`
is **not one bounded slice**. Fork (a) is sealed inside the B3 lineage
(parked pending operation-outcome authority; seedBlocks adds an
unowned sub-shape). Fork (b) needs a coverage issuer for ordinary
instance calls that does not exist; production admission of that call
kind is part of the parked DeclaredInstance lineage. Per the family
scheduler both are family-local sealed — selection returns to the next
inventoried candidate rather than reopening them.

```text
Decision: select F2 — add the `ConditionalUpdateIf` arm to the
located-source LoopCond parts driver, reusing the existing raw
lowering arm.
Source authority + canonical issuer: resolver recipe item
`ConditionalUpdateIf` (issued by the loop_cond recipe builder); sole
physical owner `lower_loop_cond_source_item` ->
`lower_conditional_update_if_assume_with_break_phi_args_recipe_first`.
Non-authority: route coverage contract, publication consumption,
DeclaredInstance/ArrayPush issuers, Facts extractors.
Fail-fast boundary: `then_exit` arms other than the inventoried None/
break shapes stay on `loop-cond-item-unsupported`; no partial lowering.
Smallest next slice: one item-kind arm in the located-source parts
driver with positive/negative pins and the json_stream / binary-trees
`run` receipts.
Non-claims: no coverage relaxation (SourceItemsMissing /
SourceCallOutsideSelectedFamily / UnconsumedSelected stay named
terminals), no new Facts arm, no ArrayPush/DeclaredInstance admission,
no green claim for the four loop-blocked apps.
```

Follow-on inventory recorded, not selected (order only — each still
needs its own owner census at entry):

- F3b `UnconsumedSelected`: arm publication consumption for the
  singleton LoopCond route (probe-verifiable e2e via the
  `Step.twice` no-exit loop).
- F3c `SourceItemsMissing`: call-free loops are "a new callable
  admission shape" per B3-D2 — contract question, not a bug.
- F3a coverage forks: B3-ArrayPush (sealed D2) and
  ordinary-instance-call coverage (no issuer; parked
  DeclaredInstance lineage) — reopen only through their own cards.
