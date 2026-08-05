Status: selected cfg(test)-only execution brief; production remains closed
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-REPEAT-AUDIT0-D3-S2-S3`
Ceremony: T2 neutral passive repeat audit

# D3-S2-S3 — provenance repeat audit

## Purpose

Repeat the closed D3-S2-S2 provenance product across two fresh resolver
sessions. The audit proves that structural provenance may repeat while the
resolver-owned brands remain distinct. It also records that equal raw frame
coordinates are not an identity witness. This row is evidence only; it does
not open the upper source-to-selection boundary.

## Sole authority and input

The resolver-issued owner/brand remains the only source-identity authority.
The repeat observer is a private `cfg(test)` sink in
`mir::resolved_semantics`. Registry, router, `loop_structural_facts`, and
Builder code are not issuers.

The observer accepts exactly one private, non-`Clone` pair input containing two
complete `VerifiedResolvedCarrierProvenanceV1` products, one from fresh
resolver session A and one from fresh resolver session B. It does not accept
loose forests, frames, roles, AST, facts, labels, or route fragments.

## Output witness

The private, non-`Clone`, `cfg(test)` observation retains only:

- source topology and outer/inner site equality;
- typed role and strict-ancestor equality;
- distinct A/B resolver brands;
- raw `FrameKey` coordinate equality as a non-identity observation.

The witness must not retain AST, `CanonicalLoopFacts`, labels, route or
schedule data, `ValueId`, PHI, Generic snapshot/key/seed, selector state,
`InvocationSeal`, or Return/Home/debt meaning.

## Typed rejects and chronology

All rejects are pre-effect and have no fallback or retry. The typed matrix is:

```text
ReusedOrEqualOwnerBrand
FunctionOriginMismatch / SourceKindMismatch
OuterInnerSiteMismatch
ForestTopologyMismatch
RoleKindOrSiteMismatch
BindingRelationMismatch
StrictAncestorMismatch
FrameCoordinateMismatch
MissingOrDetachedProduct
mixed or foreign owner brand
```

Equal raw frame coordinates with distinct resolver brands are an observation,
not a reject. A pair that cannot prove complete product ownership is rejected
before any observer state is published.

## Stop lines and non-claims

Stop and return to the parent D3-S2 design card if implementation needs loose
parts/accessors, AST rescan, a second issuer, a pairing heuristic, raw
`FrameKey` changes, or any Generic snapshot/key/seed, selector, eligibility,
winner, Builder/MIR/Recipe/PHI, Return/ABI/Home/debt, or production meaning.

This row must not add a production caller, import, artifact, or DirectAccum
frame authority. Existing parity remains unchanged. Source and check files
remain below 800 lines.

## Acceptance

```text
fresh session A + fresh session B -> one non-Clone pair -> one opaque witness
same source topology/roles/strict ancestor = observed equal
resolver brands = distinct
equal raw frame coordinates != identity
typed mismatch rejects = pre-effect
production issuer/caller/artifact = 0
DirectAccum frame semantics changed = 0
Generic snapshot/key/seed/selector authority added = 0
focused suite green; pointer guard green; diff check green
```

The implementation commit must update the resolved-semantics README, this
task receipt, the D3-S2 design card, `CURRENT_STATE.toml`, `10-Now.md`, and
the MirBuilder workstream together. Any later implementation of this family
must update the exact reference documentation in the same commit and close
the corresponding reference-closeout receipt.
