---
Status: closed__DecisionRecorded__LoopPrecutoverAuthorityG0__2026-09-11
Task: LOOP-PRECUTOVER-AUTHORITY-G0-D0
Date: 2026-09-11
Priority: name one real Generic production caller and its exclusive cutover boundary
Parent: mirbuilder-loop-semantic-program-coseal-d0-2026-09-11
NextCard: LOOP-G0-CANONICAL-PREFLIGHT-ISSUER-D0
---

# Loop pre-cutover authority G0 D0

## Six-line brief

```text
Decision: design one Generic G0 production handoff around the existing source parent; do not implement or switch a route yet.
Source authority + canonical issuer: ResolvedFunctionLoweringInputV1 plus existing issue_generic_g0_source_parent_v1.
Non-authority: route_loop, cfg(test) ingress, physical admission/session, MIR observations, and tests-only parent/cohort witnesses.
Fail-fast boundary: source-parent validation and the named consumer's typed rejection must complete before physical session/artifact effects.
Smallest next slice: identify one real caller, replacement edge, terminal, split/re-pair-free parent path, and exclusive old-edge deletion set.
Non-claims: no new semantic receipt, route selection, physical cutover, fallback/retry retirement, M10b/M11/M12, or whole-MirBuilder completion.
```

Census boundary: `issue_generic_g0_source_parent_v1` -> Generic physical
session/collector or the retained legacy route terminal; includes the finite
Generic source-parent, admission, router, and publication edges; excludes
Callable, Dynamic, M8/M9, Call/R7, backend parity, and unrelated constructors.

## Existing authority and boundary

The source authority is `ResolvedFunctionLoweringInputV1` plus the resolver
selection evidence consumed by `generic_g0_source_parent.rs`. The canonical
issuer already co-seals Generic demand/Recipe, source relations, ABI, effect,
Completion, storage lane, and body coverage. It is caller-zero material, not
a new universal semantic-program issuer.

The legacy route remains selected by `route_loop`. The design must not infer a
caller from that route, pass a second `LoopRouteContext`, rebuild Facts/Recipe,
or pair the source parent with physical IDs. The selected replacement must
consume the existing parent once and end in one named physical owner or a
typed pre-effect terminal.

## Production caller census

The finite caller inventory was checked against the current source tree:

| edge | evidence | disposition |
| --- | --- | --- |
| raw Loop child -> legacy route | `raw_loop_child_port.rs:28-43` -> `lower_loop_or_freeze_v1` | existing old edge; not a G0 source-parent consumer |
| located callable Ready -> source Facts | `raw_loop_child_entry.rs:198-270` -> `CallableGenericLoopSourceFactsIssuerV1` -> `CallableGenericLoopV1PhysicalAdapterV1` | real production edge, but a distinct Callable source authority and product |
| G0 source parent -> physical cohort | `generic_g0_physical_operation_cohort.rs:171-181` | caller-zero wrapper; only test/canary reachability found |
| G0 admission -> Builder session | `generic_g0_physical_emitter_session.rs:67-151` | caller-zero preflight; its admission calls are test-only |
| canonical source package -> selected plan | `source_bound_package.rs:600-640` | real production terminal, but its plan sum has no Generic G0 variant |
| old router -> legacy scheduler | `routing.rs:552-555` -> `route_entry/router.rs:255-...` | selected old edge; no source parent is available there |

Therefore the non-test caller count for
`issue_generic_g0_source_parent_v1` is zero. The existing Callable adapter
cannot be relabeled as G0: it consumes a different source Facts receipt and
`CallableGenericLoopV1SemanticRecipeV1`, while the G0 parent requires
`ResolvedFunctionLoweringInputV1` plus canonical G0 selection evidence. Passing
both products would create split/re-pair ingress.

The only viable production-caller design candidate is the existing
source-bound package terminal (`SourceBoundCanonicalPackageV1::consume_parts`),
but opening it requires a named canonical G0 plan variant, its source issuer,
and its exact lifecycle mapping. That is semantic design work, not a
BoxShape-only edit; this card remains a design stop until the mapping is
accepted.

## Canonical package mapping review

The package choice is now narrowed to one admissible shape:

```text
ResolvedFunctionLoweringInputV1
  -> canonical G0 preflight issuer
  -> CanonicalLoopFamilyPlanV1::GenericG0
  -> ExactCanonicalPreflightPlanV1::Loop
  -> SourceBoundCanonicalPackageV1
  -> one unpublished Generic physical terminal
  -> existing collect/complete/drain owner
```

The separate-package option is rejected. It would duplicate the package
bind/open/lower/collect/complete lifecycle and require a second route or
continuation authority. Reusing `DirectAccum` or `NestedPredicate` is also
rejected: those variants carry different source products and lower through
different Builder consumers; relabeling one would split or re-pair the G0
source parent and physical input. The existing Callable adapter is excluded
for the same reason: its source Facts and Recipe are not the Generic G0
products.

The intended external lifecycle is the existing
`CanonicalSourceRouteV1::BindingSsaTrivial` /
`ModuleInvocationFamilyV1::BindingSsaTrivial` mapping, with no new G0 token.
That mapping is conditional, not yet proven: the future canonical G0 plan
must co-seal the owner header, drain manifest, one source-bound token, and the
Generic physical completion so that the existing single-owner completion path
does not infer or rebuild any relation. If that correspondence cannot be
proved, the design returns to `NoSafeSlice`; a new token is not a shortcut.

The future plan payload may reuse the existing
`issue_generic_g0_source_parent_v1` product, but its canonical preflight
issuer must receive the same `ResolvedFunctionLoweringInputV1` source context
and issue the G0 selection/parent as one lineage. The current
`CanonicalLoopFamilySelectionV1` is evidence, not a permission to call the
test-only cohort/session wrappers from production. The production terminal
must be an arm of `SourceBoundCanonicalPackageV1::consume_parts` that returns
an unpublished canonical draft to the existing collector; the current
`generic_g0_physical_emitter_session` callback, which always discards its
outer draft, is only a mechanical preflight aid until that owner contract is
closed.

The old-edge deletion set is deliberately not accepted yet. Candidate edges
to classify at the next slice are the raw-child handoff
(`raw_loop_child_port.rs`), `routing.rs::route_loop` /
`route_entry::router::route_loop`, and the GenericLoopV0/V1 registry execution
rows. They may be deleted only after the new caller is G0-exclusive and the
same commit proves that non-G0 rows do not enter the deleted edge. No broad
legacy-router deletion is implied by this card.

## Decision outcome

The real production caller boundary is now named:

```text
MirCompiler::compile_resolved
  -> compile_resolved_first_family
  -> CanonicalLoweringPreflightV1::verify
  -> future CanonicalLoopFamilyPlanV1::GenericG0 arm
```

This closes the package-mapping decision only. It does not claim a caller
switch, a physical result, or old-route retirement. The next card owns the
source selector and single issuer needed to make the named arm real.

### Design-stop exit evidence carried forward

Implementation remains forbidden until the next card records all of these:

1. the exact canonical G0 source selector and single issuer boundary;
2. the plan payload and proof of `BindingSsaTrivial` header/manifest/lifecycle
   compatibility;
3. the production terminal's unpublished-draft and typed-rejection contract;
4. one source-to-exe positive, one pre-effect negative, and a no-split/re-pair
   guard; and
5. an exclusive G0 old-edge delete set with caller-zero evidence.

## Finite state table

| state | owner | effect | allowed next state | legacy route |
| --- | --- | --- | --- | --- |
| `SourceParentCallerZero` | G0 source-parent issuer | none | design candidate only | not entered |
| `NamedCallerCandidate` | this D0 | none | accepted cutover Decision or `NoSafeSlice` | forbidden |
| `NoProductionCaller` | scheduler boundary | none | design stop | unchanged, not a fallback |
| `SourceParentRejected` | existing G0 reject owner | none | terminal discard | forbidden |
| `PhysicalFailureBeforeAcceptance` | named future consumer | unpublished only | terminal discard | forbidden |

No state may turn a missing caller into a default route, retry, or successful
physical result.

## Required design evidence

The next Decision must record:

1. one real production caller receiving the existing Generic source parent;
2. one replacement edge and one selected terminal consumer;
3. proof that source parent, Recipe/JoinSig continuation, and physical input
   remain one lineage without split/re-pair ingress;
4. source-aware rejection before Builder/session/artifact effect;
5. the exact old route/edge symbols deleted by the same cutover series;
6. positive/negative acceptance and reusable existing guards.

The design review must also decide whether the G0 plan is admitted as a new
variant of `CanonicalLoopFamilyPlanV1`/`ExactCanonicalPreflightPlanV1` or as a
separate source-bound package branch. It must preserve one source issuer and
one lifecycle token mapping; a parallel route registry or raw-loop bridge is
not an admissible answer.

If any item requires a new `Verified*`/`Prepared*` semantic product, a second
source walk, or an inferred caller, return to `NoSafeSlice` and name its
source-backed issuer. Do not use a fixture or a local green test to cross the
design boundary.

## Non-claims

This card does not claim a Generic production caller exists, `route_loop` is
switched, the old route is at caller zero, all-19 coverage is complete, or
MIRBuilder is complete. Its acceptance is one authority-backed cutover
Decision with finite caller/delete evidence; implementation starts only after
that Decision is accepted.
