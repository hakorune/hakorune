---
Status: active design stop — disposition matrix and winner/disjointness
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Decision: provisional — docs-only taskization; no production or cfg(test) child yet
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

Every source-backed observation must end in exactly one typed disposition:

```text
ResolvedCandidate
LegacyPreserveExistingSchedule
UnresolvedStop
NoStandaloneRow
NotYetObserved
```

`CompleteNoRecursiveCarrier` is a facts observation, not
`ProvenOutsideTarget`, `Legacy`, or eligibility. A target row with a missing,
foreign, unstable, or mismatched handoff is `UnresolvedStop`; it must never be
silently converted to the old legacy schedule. `NoStandaloneRow` is reserved
for a source view that reaches the boundary but has no canonical facts/raw
schedule. `NotYetObserved` is a design inventory state, not a runtime result.

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

Its current disposition remains
`UnresolvedStop(WinnerEquivalenceUnavailable)`. The distinct direct V0/V1
semantic digests, `LowerSome` stages, and first-effect observations are
post-effect corroboration only; they do not select V1, suppress V0, or prove
semantic equivalence.

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
parsed natural V1-only         NotYetObserved
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

The future winner predicate is deliberately two-stage:

1. **Pre-effect qualification**: exact source/forest/frame identity, same
   strict-ancestor BindingRef, `CompleteRecursiveCarrier`, natural Both, and
   mode/seed seal all pass before Builder effects.
2. **Post-effect corroboration**: direct V0 and V1 candidates are isolated in
   fresh builders, both reach their recorded stage, first-effect ownership is
   stable, alpha-normalized semantic candidates are compared, and repeat
   observations are stable.

The alpha-normalized comparison must include result/Home/PHI/debt meaning, not
only route labels or raw ValueIds. A legacy V0 terminal/no-debt receipt is
corroboration, not winner authority. Until exact target equality, candidate
isolation, fresh-repeat stability, alpha-normalized equivalence, and the
no-debt/different-winner check are all proven, the typed disposition remains
`UnresolvedStop(WinnerEquivalenceUnavailable)` and the old scheduler remains
execution authority.

## Allowed next child after this decision

Only after this card is accepted may one separate cfg(test)-only child be
selected. The smallest coverage child is the parsed V1-only local shape:

```hako
function generic_v1_only_local(i) {
  loop(i < 3) {
    local tmp = 0
    i = i + 1
  }
  return i
}
```

That child, if selected, must prove `[Loop, Return]`, loop body
`[Local, Assignment]`, one resolver forest root, the same write/read
`BindingRefV1`, exact `CompleteNoRecursiveCarrier`, and actual Release/Strict
raw `[GenericLoopV1]`. Its result is typed
`UnresolvedStop(V1OnlyNonRecursive)`, never eligibility or Legacy. Facts
absence, raw `[]`, Both, simple-while, shape drift, or identity drift returns
to this design stop. This paragraph authorizes no implementation by itself.

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

Until acceptance is recorded, the only valid action is further design review;
production implementation and additional source-shape tests are stopped.
