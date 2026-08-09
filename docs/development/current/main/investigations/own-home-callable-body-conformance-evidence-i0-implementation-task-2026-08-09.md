---
Status: landed bounded implementation; general conformance remains parked
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-conformance-evidence-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-CONFORMANCE-EVIDENCE-I0

## Change

Implemented only the private, bounded `return me` evidence path:

```text
VerifiedInstanceMethodBodyOwnerCatalogV1
  + VerifiedCallableQueryBodyFactsCatalogV1
  + selected VerifiedHomeAbiV1
      -> VerifiedQueryBodyConformanceEvidenceV1
```

The implementation has one public-in-module issuer and one atomic
non-`Clone` aggregate. Keep the neutral body-shape inventory, declared Home
ABI, and Query behavior as separate authorities.

Landed in `src/mir/resolved_semantics/query_body_conformance_evidence.rs`:

```text
QueryBodyConformanceEvidenceIssuerV1
  -> VerifiedQueryBodyConformanceEvidenceCatalogV1
  -> VerifiedQueryBodyHomeFlowEvidenceV1(transfer = None)
```

The focused `query_body_facts` slice now exercises the real parser/resolver
fixture and asserts the bounded Home no-transfer receipt. No general
conformance catalog was opened.

## Required proof

```text
exact owner/declaration/parser/resolver identity
exact body root and complete bounded statement/expression/relation coverage
one Return(value), one receiver Me BindingRef, one ReturnValue relation
receiver Home demand = Handle
result Home relation = Trivial
no Home consume/create/end/escape in this exact structural cohort
no writes/allocation/call/IO/FFI/failure/suspension/non-local control
```

Use the same resolver owner-tree products. No AST rescan, name/ordinal pairing,
MIR `EffectMask`, `FunctionSignature`, ownership SSA, or runtime state.

## Negative matrix

```text
return 0 / empty / local / field / method call -> Declined
extra statement/effect or unsupported control -> Declined or NoSafeSlice
foreign owner/brand/source/body root        -> Rejected
Home ABI receiver/result mismatch            -> Rejected
missing/duplicate ReturnValue relation      -> Rejected
nested callable or capture                  -> NoSafeSlice
opaque/incomplete effect/control coverage   -> NoSafeSlice
```

Do not create forged `Verified*` constructors merely to exercise impossible
states; retain defensive issuer errors and reuse the real parser/resolver
fixtures. The existing body-shape, owner-link, and Query-facts tests remain
the prerequisite identity/shape evidence.

## Explicit non-claims

```text
general Home Flow or Ownership SSA
field/index/state/call/loop/branch/capture bodies
non-Query callable families
complete conformance catalog
publishable catalog
target/source-bound call/Recipe/CallSlot
Builder/MIR/CFG/PHI/physical ABI
fallback/retry/provider/runtime/production
```

## Closeout

Keep each Rust file under 800 lines. Add focused real-fixture tests, update the
resolved-semantics README, callable reference, this receipt, task map, and
current pointers in one slice. Run focused resolver tests and
`bash tools/checks/current_state_pointer_guard.sh`; commit and push this slice.
`CALLABLE-CONTRACT-CONFORMANCE-I0` remains parked because general
effect/control/Home-flow completeness is still `NoSafeSlice` outside this
bounded cohort. Do not widen `BodyEffectKindV1`, use MIR ownership SSA, or
open target/Recipe/MIR as a workaround.
