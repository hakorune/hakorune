# MIRBUILDER-EXE-ACCEPTANCE-SOURCE-CALL-COVERAGE-D14

Status: design_stop__2026-09-26
Date: 2026-09-26
Parent: workstream row H (unified resume, gate 1);
  OWNER-SELECTION-D13 / DEAD-ROUTE-SOLE-FAMILY-S1 landed
Mode: design_stop — census and Decision only.

## Blocking observation

```text
EXE (pure-first, debug binary):
  [freeze:contract][callable-loop/route-not-front-selected]
  LoopCondRouteRejected(SourceCallOutsideSelectedFamily {
    call_sites: [
      SourceExprSiteV1([Body(2), LoopCondition, Rhs, Lhs, Lhs]),
      SourceExprSiteV1([Body(2), LoopCondition, Rhs, Rhs, Lhs])
    ] })
  function=StringHelpers.trim/1
  source=Cataloged(StaticBoxMethod "StringHelpers"."trim"/1)
  site=[Body(2)]

VM (debug binary): unchanged known terminal —
  loop winner selection declined: zero selected family
  candidates fn=JsonStreamAggregator.ingest/1 [Body(2)]
  (ledger-less instance method → route_loop spine;
  separate authority, recorded non-claim).
```

`trim`'s whitespace loop now front-selects
`LoopCondBreakContinue` (S1). The route token's
source-call coverage gate rejects: the two `s.substring`
calls live inside `LoopCondition` (`Rhs.Lhs.Lhs`,
`Rhs.Rhs.Lhs` — the object positions of the two
`substring(...) == literal` terms) and have no selected
static publication relation.

## Census findings

### Coverage authority chain (all existing)

```text
resolver method_call rows (condition + body sites)
  -> issue_source_bound_core_method_calls_v1   [ARM GATE]
       supported_placement(StringSubstring,2) = Body only
  -> source_core_method_calls (CallableSemanticLoweringState)
  -> source_target_for_loop -> probe.core_methods
  -> into_selected_relation -> CoreMethod relation
  -> route token co-seal -> lower_loop_cond_break_continue_source
  -> lower_loop_header_cond_input (And/Or/Not short-circuit)
  -> lower_value_input -> exact_source_method_call
  -> take_source_core_method_call (placement-agnostic)
  -> CoreEffectPlan::MethodCall (TextToCaller -> MirType::String)
```

- `raw_loop_child_port.rs source_target_for_loop`: `items` =
  `source_loop_items(parent_site)` — resolver method-call
  rows under the loop site, INCLUDING `LoopCondition`
  positions. `core_methods` =
  `source_core_method_items(parent_site)` — subset of items
  armed by a `source_core_method_calls` contract row.
- The reject's two sites are resolver items with no
  contract row: `issue_source_bound_core_method_calls_v1`
  reached `supported_placement(StringSubstring, 2)` =
  `Body` (core_method.rs:154), the call's actual placement
  is `Condition`, candidate loops filtered to zero →
  `continue` — no row issued. `StringLen/0` is admitted at
  `Condition`; `StringSubstring/2` and `ArrayPush/1` at
  `Body` only.
- `resolver_core_method_callable_contract.rs
  required_target_placement` re-pins the same (op,arity)→
  placement map (`Substring/2 → Body`) — a second pin, not
  a second authority.
- Physical consume is placement-agnostic:
  `take_source_core_method_call` never reads
  `contract.placement()`; `TextToCaller` maps to
  `MirType::String`; `CoreEffectPlan::MethodCall` emits at
  whatever expression position the port walks.
- Condition structure already supported:
  `lower_loop_header_cond_input` recurses `And`/`Or`/`Not`
  into short-circuit blocks; leaf compares go through
  `lower_value_input` → `exact_source_method_call` — the
  same port that consumes core-method contracts at body
  positions.
- Receiver: `s` is a parameter → `Lexical(Local(binding))`
  — inside the arm's declared vocabulary (lexical local
  receivers; qualified/current-owner receivers stay
  `UnsupportedReceiver`).
- `ResolvedLoopPlacementV1` = {Condition, Body} — the
  condition position is already exact vocabulary; the
  contract's per-sub-site placement check (receiver, args,
  result all `Condition`) holds for both trim sites.
- NoSafeSlice analysis: the semantic arm, contract verify,
  probe coverage, and physical consume all exist — only
  the placement table's bounded (op,arity) vocabulary
  lacks `(StringSubstring,2)→Condition`. Not a missing
  owner; a bounded BoxCount gap on the existing
  CoreMethod-contract authority.
- `nearest_loop` needs no change: candidates = loops whose
  resolved placement for the site is in the allowed set;
  for a condition site only `Body(2)` qualifies — unique.

### Compare: same `SourceCallOutsideSelectedFamily` family

- `binary_trees.iterationCheck` / mimalloc `seedBlocks`:
  same reject token, but their uncovered sites are static
  calls / non-StringBox receivers — a different arm
  question; not covered by this card's slice.

## Boundary of this census

- Covers: `SourceCallOutsideSelectedFamily` on
  condition-position call sites for the LoopCondBreakContinue
  owner (`trim`-class), from `CallableLoopSourceTargetProbeV1`
  input assembly to the reject emission.
- Excludes: `ConditionalUpdateIf` parts boundary
  (`ingest`, F2); VM ledger-less lanes; winner spine;
  continue-only loops; body-position call coverage already
  green.

## Decision

```text
Decision: `trim`'s decline is a bounded vocabulary gap in
  the existing CoreMethod contract arm — not a missing
  owner. `(StringSubstring,2)` placement is pinned to
  `Body` in two tables while the physical consume is
  placement-agnostic. Admit `StringSubstring/2` at
  `Condition` on the same authority; the placement becomes
  an allowed-set per (op,arity) rather than a single
  expected value.
Source authority + canonical issuer:
  `issue_source_bound_core_method_calls_v1`
  (source_call_target/core_method.rs) arms resolver call
  rows -> `ResolverCoreMethodCallableContractIssuerV1`
  (resolved_semantics) verifies and seals placement/frame/
  target -> `CallableSemanticLoweringState.
  take_source_core_method_call` is the sole physical
  consume (exact site, one take).
Non-authority: `target_for_source` static publication map
  (different call family); `S6C` scan-with-init target
  plans (their own placement contracts, unchanged); the
  probe (`into_selected_relation` mapping unchanged — it
  already treats CoreMethod rows as coverage).
Fail-fast boundary: `StringLen/0` stays Condition-only;
  `ArrayPush/1` stays Body-only; every other (op,arity)
  stays unarmed; qualified/current-owner receivers keep
  `UnsupportedReceiver`; sub-site placement drift keeps
  `PlacementMismatch`.
Smallest next slice (S2): generalize the two placement
  pins to per-(op,arity) allowed sets and add
  `Condition` to `StringSubstring/2`'s set —
  `supported_placement` becomes an allowed-set filter
  (issue carries the site's actual placement) and
  `required_target_placement` accepts placement ∈ set.
  Pins: substring-at-condition contract issues with
  `placement == Condition` (trim-header fixture); body
  substring still contracts `placement == Body`;
  StringLen-at-Body and ArrayPush-at-Condition remain
  unarmed; real EXE smoke advances past trim's coverage
  gate to the next honest terminal.
Non-claims: does not arm non-StringBox core methods,
  static publication gaps, or `ingest`'s ConditionalUpdateIf
  / VM ledger-less lanes; does not touch S6C target plans
  or the probe's coverage arithmetic; does not widen
  receiver vocabulary.
```

## Exit

- [x] Failing call sites' coverage authority identified
  (CoreMethod contract arm; placement vocabulary).
- [x] Existing owner vs unclaimed gap classified —
  bounded vocabulary gap on the existing owner.
- [x] One bounded S-card emitted
  (`MIRBUILDER-EXE-ACCEPTANCE-COND-SUBSTRING-COVERAGE-S2`).
