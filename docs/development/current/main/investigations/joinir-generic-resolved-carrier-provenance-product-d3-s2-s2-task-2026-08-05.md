---
Status: closed cfg(test)-only evidence; production remains closed
Date: 2026-08-05
Parent: joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md
Task: `JOINIR-GENERIC-RESOLVED-CARRIER-PROVENANCE-PRODUCT0-D3-S2-S2`
Ceremony: T2 neutral passive product
---

# Resolved carrier provenance product

## Change

Add a private, non-`Clone` `VerifiedResolvedCarrierProvenanceV1` factory in
the test-only `mir::resolved_semantics` boundary. It consumes one co-sealed
S1 handoff from a verified resolver function and publishes one AST-free source
witness. The factory is the only product constructor; Builder, structural facts,
registry selection, labels, and route IDs never issue or own it.

## Contract

- Retain only owner/source kind, exact outer/inner sites, branded forest/frame,
  and exactly two resolver role rows (`NestedWrite`, `PostLoopRead`) with the
  exact `BindingRefV1` and strict-ancestor relation.
- Keep the product opaque and non-detachable: no public constructor,
  `Clone`, or independently pairable `parts()` accessor.
- Reject foreign/missing/ambiguous/duplicate roles, mixed brand, unequal
  bindings, incomplete forest, source/frame mismatch, AST/ValueId leakage, or
  non-natural-Both role shape before any Builder effect.
- Preserve the S0/S1 tests and leave DirectAccum's ownerless structural frame
  unchanged. No `Option`, fallback, retry, Legacy route, or V0 suppression.

## Done

- Added the private non-`Clone` `resolved_semantics` handoff/product boundary
  and a five-row focused positive/mixed/typed-negative matrix; all ten legacy
  S0/S1 plus S2 tests pass. Production caller/import and artifact remain
  zero/none.
- The only product constructor consumes one co-sealed handoff. Fixture-only
  `for_test(...)` ingress cannot become a production issuer; no detachable
  parts accessor or `Clone` implementation exists.
- `GenericCarrierFactsSnapshotV1`, `LoopBindingKeyV1`, `InvocationSeal`,
  preflight seed, selector/eligibility/winner, Recipe/JoinSig/PHI, Return/ABI/
  Home/debt, MIR/VM, and runtime routes remain unclaimed.
- Updated the D3-S2 design card, resolved-semantics README, `CURRENT_STATE.toml`,
  10-Now, workstream, and this task card in the implementation commit. Every
  touched source/check file remains below 800 lines; focused suite, pointer
  guard, diff check, and line guard are required receipts.

## Stop

Return to D3-S2 design if the factory needs a loose component input, a second
source/BindingRef issuer, Generic logical-key assignment, a production
consumer, or any Return/PHI/Home/debt interpretation. Do not open a selector
arm or neutral Generic snapshot from this row.
