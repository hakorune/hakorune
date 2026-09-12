---
Status: closed__DecisionAccepted__R7TypedDiagnostics__2026-09-12
Task: MIR-CALL-COMPATIBILITY-RETIRE-R7-TYPED-DIAGNOSTICS-D0
Date: 2026-09-12
Parent: mir-call-compatibility-retire-r7-m7s-owner-matrix-d0-2026-09-12.md
---

# R7 typed diagnostic bridge boundary

## Six-line brief

```text
Decision: stop before changing bridge error types until each remaining direct_accum/nested_predicate/pending bridge has one stable typed owner.
Source authority + canonical issuer: the existing rejected inner error at its responsibility owner; bridge code transports it without semantic reclassification.
Non-authority: Debug strings, stage labels, tests, and a generic BuilderContract wrapper do not define a machine-readable reject kind.
Fail-fast boundary: preserve the existing pre-effect rejection and diagnostic ownership while retaining the inner typed error through the public compiler boundary.
Smallest next slice: finite read-only inventory of bridge sites, inner error types, existing CanonicalLoweringErrorV1 variants, and required callers/tests.
Non-claims: no enum variant, route, fixture, fallback, warning cleanup, or whole-R7 completion.
```

Census boundary: `resolved_direct_accum_cutover.rs` and
`resolved_nested_predicate_cutover.rs` bridge helpers -> their existing
`CanonicalLoweringErrorV1` callers, plus the one pending/bridge path identified
by the inventory. Include all `format!("{error:?}")` conversion sites in these
families. Exclude the already-closed Call/Static/Boundary slices, Generic G0,
MIR semantic issuance, and unrelated parser/runtime Debug formatting.

## Design questions

- What concrete inner error type and source owner does each bridge receive?
- Can an existing `CanonicalLoweringErrorV1` variant carry it without adding a
  new semantic authority or losing machine-readable classification?
- Which bridge sites are true typed transport and which require a separate
  responsibility owner or remain `NoSafeSlice`?
- What positive/negative tests prove the variant and retain exact reject stage,
  while preserving pre-effect behavior and source-size limits?

The audit is read-only until one Decision is accepted. Do not replace Debug
formatting with guessed variants, broad `From` impls, or string parsing.

## Accepted Decision

The two cutover helpers have a safe typed transport boundary. Add one
meaning-free transport family in `lowering_input.rs`:

```text
CanonicalResolvedCutoverStageErrorV1
  = Header | SourceBinding | PhysicalOpen | PhysicalLower | PhysicalCollection
  | PhysicalCompletion | PhysicalDrain | FinalizationPrepare | Finalization
  | Postprocess | ExternalCommit
CanonicalResolvedCutoverFailureV1 = DirectAccum(StageError) | NestedPredicate(StageError)
CanonicalLoweringErrorV1::ResolvedCutover(Failure)
```

Each payload remains its existing typed owner error. The direct and nested
`bridge_error` helpers add only their fixed family tag; stage text and
`format!("{error:?}")` are removed. `SourceBinding` may use its existing
explicit clone or a narrow owned conversion, but no broad `From` conversion
or string parsing is permitted. The resulting outer variant is transport
only and does not become a new semantic authority.

The `FunctionDraftSeal` pending bridge is explicitly excluded. Its rejected
owner currently exposes a borrowed error and the pending helper owns session
restoration concerns; it needs a separate design/implementation card with an
owned `into_parts()`-style boundary and dedicated pending tests.

## Finite audit result

The audit covered both `bridge_error` helpers and all 22 direct/nested caller
sites. The existing typed owners are available through the physical
open/lower/collect/complete/drain, finalization, postprocess, source-binding,
and external-commit stages. Preflight remains before builder opening; late
candidate/session effects remain discarded before publication. The pending
`FunctionDraftSealErrorV1 -> String` path is one separate remaining bridge.

Implementation scope is four files: `lowering_input.rs`,
`resolved_direct_accum_cutover.rs`, `resolved_nested_predicate_cutover.rs`,
and their existing caller tests. `resolved_lowering/mod.rs` is excluded to
keep its 737-line owner below the 800-line hard stop. Acceptance must match
family and stage variants, retain the original inner type, cover direct and
nested preflight plus late failure, and prove the two Debug bridges are gone.
