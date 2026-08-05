---
Status: accepted design boundary — disposition matrix and winner/disjointness
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Decision: accepted — design-only; V1-only Local child closed as cfg(test) evidence
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`
---

# Generic resolved-carrier disposition matrix

## Purpose

The parent D3 handoff has enough source-backed observations to show that the
current facts labels are not a winner policy. The next step is therefore a
design-only partition of the observed rows, not another source-shape or
selector implementation. This card fixes the partition, the winner/
disjointness proof boundary, and the owner order before any capability or
production handoff is introduced.

## Decision boundary

Every source-backed observation records two independent columns. Evidence
status describes whether the source row is present; selection disposition
describes what the current policy may do with it.

Evidence status is exactly one of:

```text
NoStandaloneRow
NotYetObserved
Observed
```

Selection disposition is exactly one of:

```text
ResolvedCandidate
LegacyPreserveExistingSchedule
UnresolvedStop
```

`CompleteNoRecursiveCarrier` is a facts observation, not
`ProvenOutsideTarget`, `Legacy`, or eligibility. A target row with a missing,
foreign, unstable, or mismatched handoff is `UnresolvedStop`; it must never be
silently converted to the old legacy schedule. `NoStandaloneRow` and
`NotYetObserved` are evidence statuses, not runtime selection results.

The only provisional candidate class is the existing natural nested `Both`
shape:

```text
same invocation
same source kind / function owner / loop forest / frame
same strict-ancestor BindingRef for nested write and outer read
CompleteRecursiveCarrier(["j"])
Release or Strict, planner off
raw schedule [GenericLoopV0, GenericLoopV1]
```

Its qualification column is `ResolvedCandidate`, but its final selection
disposition remains `UnresolvedStop(WinnerCorrectnessUnavailable)`. V0 and V1
are intentionally different candidates, so their distinct direct semantic
digests are not an equivalence proof. `LowerSome` stages and first-effect
observations are post-effect corroboration only; they do not select V1 or
suppress V0.

## Authority map

The source authority chain is:

```text
parsed source
-> VerifiedResolvedFunctionV1
-> resolver-issued loop forest and BindingRefV1 relations
-> actual GenericLoopCarrierObservationV1 / facts presence
-> actual mode and preflight raw schedule
-> direct candidate-stage observation (corroboration only)
```

The resolver owns source sites, forest, and BindingRefs. The existing
Builder-local facts extractor owns the current carrier label. A future neutral
issuer may consume an AST-free `GenericCarrierFactsSnapshotV1`; it must not
mint BindingRefs or retain `CanonicalLoopFacts`. A future private Builder
adapter may package that neutral snapshot with a source-backed eligibility
receipt and one preflight seed, but the selector must consume one opaque,
non-`Clone` wrapper rather than independently pairable facts/capability fields.

The following remain non-authority:

```text
carrier strings, route IDs, registry order, plan digests
legacy receipts, diagnostic labels, runtime/VM results, synthetic fixtures
```

## Matrix status

Source-backed rows already closed as test-only evidence:

```text
  Both + CompleteRecursive       natural nested If/Loop, D3-S0/S2A
  Both + CompleteNoRecursive    flat Assignment, D2-S5-S1
  V1-only + CompleteNoRecursive parsed lexical Local, D3-S1-S1
  Both + Unavailable             nested CompoundAssignment, D2-S3
  Both + Ambiguous               nested IndexWrite, D2-S2
  planner-required Both          same natural source, raw [V1], D2-S1
  shadowing / foreign / repeat   typed identity rejects
  facts absent / raw []          top-level Compound, D2-S4
```

The following remain inventory states and must not be fabricated from the
synthetic matrix:

```text
natural V0-only                NotYetObserved
  parsed natural V1-only         Observed; selection remains UnresolvedStop
typed Neither                  NoStandaloneRow or NotYetObserved per source
Program / nested wrapper       NotYetObserved unless a source view proves it
duplicate-write variants      NotYetObserved
mode cross-product gaps        NotYetObserved
```

The existing collector has explicit `Unavailable` arms for nested
`CompoundAssignment`, `LoopRange`, `Lambda`, `BlockExpr`, `TryCatch`,
`TaskScope`, `ContextScope`, `FastMemRegion`, and `BuildGate`, and an
`Ambiguous("assignment target")` arm for non-variable nested writes. Top-level
compound currently falls through to a facts-absent boundary, which is why
D2-S4 is `NoStandaloneRow`; it must not be relabeled from the collector name.
Scope flattening also means a direct `ScopeBox` arm is not a source-backed
claim until a resolved source view proves that shape.

## Winner/disjointness protocol

The future correctness predicate is deliberately two-stage:

1. **Pre-effect qualification**: exact source/forest/frame identity, same
   strict-ancestor BindingRef, `CompleteRecursiveCarrier`, natural Both, and
   mode/seed seal all pass before Builder effects.
2. **Post-effect corroboration**: direct V0 and V1 candidates are isolated in
   fresh builders, both reach their recorded stage, first-effect ownership is
   stable, the V1 candidate's source BindingRef/PHI/final-value relation is
   fixed, V0 is proven disjoint from that carrier, and repeat observations are
   stable.

The proof must include result/Home/PHI/final-value meaning, not only route
labels or raw ValueIds. A legacy V0 terminal/no-debt receipt is corroboration,
not winner authority. Until the V1 source relation, V0 disjointness, candidate
isolation, fresh-repeat stability, and no-debt/different-winner check are all
proven, the typed disposition remains
`UnresolvedStop(WinnerCorrectnessUnavailable)` and the old scheduler remains
execution authority.

## Allowed next child after this decision

The separately selected cfg(test)-only child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-V1ONLY-LOCAL0-D3-S1-S1`.
Its task card is
`joinir-generic-resolved-carrier-source-matrix-v1only-local-d3-s1-s1-task-2026-08-05.md`.
Its smallest coverage shape is the parsed V1-only local form:

```hako
function generic_v1_only_local(i) {
  loop(i < 3) {
    local tmp = 0
    i = i + 1
  }
  return i
}
```

The child must co-seal `V0 facts = false`, `V1 facts = true`, actual
`has_body_local = false`, actual Release/Strict preflight flags, one resolver
forest root, the same write/read `BindingRefV1`, exact
`CompleteNoRecursiveCarrier`, and raw `[GenericLoopV1]`. It must prove
`[Loop, Return]` and loop body `[Local, Assignment]`. Its result is typed
`UnresolvedStop(V1OnlyNonRecursive)`, never eligibility or Legacy. Facts
absence, raw `[]`, Both, simple-while, shape drift, or identity drift returns
to this design stop. The task card, not this design paragraph, authorizes the
test implementation. The router's `has_body_local` flag denotes the separate
`LoopBreakBodyLocalFacts` TrimSeg/DigitPos break-guard family, not the presence
of an ordinary `Local` statement; this row must preserve that distinction.

The child is now closed as cfg(test)-only evidence. The parsed source produced
`V0 facts=false`, `V1 facts=true`, `CompleteNoRecursiveCarrier`, actual
Release/Strict frame flags, `has_body_local=false`, no recipe contract, and raw
`[GenericLoopV1]`. Its typed result is `Observed` plus
`UnresolvedStop(V1OnlyNonRecursive)`. No eligibility, Legacy, winner, selector,
or production handoff was added.

The selected child was
`JOINIR-GENERIC-RESOLVED-CARRIER-CANDIDATE-STAGE-SOURCE-BRIDGE0-D3-S1-S2-D0`,
recorded in
`joinir-generic-resolved-carrier-candidate-stage-source-bridge-d3-s1-s2-task-2026-08-05.md`.
It is now closed as the smallest cfg(test)-only bridge that co-seals the
parsed natural-Both source/resolver obligation with fresh V0/V1 plan
projections. Existing synthetic plan evidence remains non-authoritative; the
parsed observer and actual plan rows are co-sealed only as a label-backed
projection. Its result remains `Observed +
UnresolvedStop(WinnerCorrectnessUnavailable)`; no typed BindingRef-to-plan
provenance, full-return parity, or production selection is authorized.

## Next design child — typed provenance handoff (`D3-S2-D0`)

The next child is the docs-only design card
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-HANDOFF-DESIGN0-D3-S2-D0`,
recorded in
`joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md`.
It is selected before any scalar Return projection because S2's plan-local
names and `ValueId`/PHI labels cannot establish a source-to-plan relation.
The card fixes the resolver-owned `BindingRefV1` observation, the neutral
AST-free `GenericCarrierFactsSnapshotV1`, the logical `LoopBindingKeyV1`
projection, and the non-Clone opaque selection input as separate owners.

Its reject boundary is pre-effect and typed: missing/foreign/ambiguous
provenance, source/forest/frame mismatch, AST or physical-ValueId leakage,
missing seal/seed, incomplete matrix, or unobserved winner/result/Home/PHI
parity remains `UnresolvedStop`, never Legacy fallback, retry, V1 precedence,
or V0 suppression. A later cfg(test)-only scalar full-function Return row is
allowed only after this relation is sealed; Home semantics remain a separate
design. Natural V0 debt followed by a different V1 winner still requires an
actual producer receipt; synthetic debt is not evidence.

The first child selected under D3-S2 is
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`,
recorded in
`joinir-generic-resolved-carrier-provenance-observation-d3-s2-s0-task-2026-08-05.md`.
It is a private cfg(test)-only observation of resolver forest/frame and exact
`BindingRefV1` role/strict-ancestor relations. It does not assign Generic
logical keys or publish a neutral snapshot; those remain a follow-up design.

The S0 observation child is now closed as cfg(test)-only evidence. Its private
witness consumes the parsed resolver forest/frame and exact `BindingRefV1`
role/strict-ancestor relations, with natural success plus shadowing, foreign
owner, forest-shape, and frame mismatch rejects. Production caller/import is
zero and artifact is none. It does not establish Generic key assignment,
neutral snapshot ownership, opaque seed pairing, or any winner/Return/Home/debt
semantic claim; execution returns to the D3-S2 design stop.

## Prohibited changes

This design card does not authorize a neutral issuer, `InvocationSealV1`,
selector arm, `Option<Capability>`, Legacy target fallback, V0 suppression,
V1 precedence, Recipe/JoinSig/PHI/physicalizer, Builder/MIR/backend/runtime
caller, Retry deletion, or global scheduler cutover. It also does not claim a
natural V0-only source witness.

## Acceptance and closeout

Acceptance requires the parent D3 card, Generic post-effect SSOT, stage-matrix
reference, Generic and resolved-semantics READMEs, `CURRENT_STATE.toml`,
`10-Now.md`, the active workstream, and affected reference indexes to point to
this one design authority. Any later implementation child must update the
language/reference documents after implementation in the same closeout commit,
with focused evidence, caller census, fail-fast boundaries, artifact manifest,
and all touched source/check files below 800 lines. The workstream remains
exactly 1000 lines.

The design remains a policy boundary. The selected child and its reference
closeout are complete; the parent design, Generic post-effect/stage references,
resolved-semantics README, current mirrors, and active workstream were updated
in the implementation closeout. Production implementation, neutral issuer,
selector, and parent Generic D2 handoff remain stopped.
