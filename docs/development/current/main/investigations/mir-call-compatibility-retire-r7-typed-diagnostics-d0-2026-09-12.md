---
Status: selected__DesignStop__R7TypedDiagnostics__2026-09-12
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
