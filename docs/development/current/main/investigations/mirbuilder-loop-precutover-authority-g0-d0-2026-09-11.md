---
Status: active__NoSafeSlice__LoopPrecutoverAuthorityG0__2026-09-11
Task: LOOP-PRECUTOVER-AUTHORITY-G0-D0
Date: 2026-09-11
Priority: name one real Generic production caller and its exclusive cutover boundary
Parent: mirbuilder-loop-semantic-program-coseal-d0-2026-09-11
NextCard: Generic production caller Decision
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
