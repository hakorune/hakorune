---
Status: implementation-ready bounded I0; general conformance remains parked
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-conformance-catalog-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-CONTRACT-CONFORMANCE-I0

## Scope

This row opens only the already-proven exact Query cohort:

```text
declared Query contract
  + VerifiedQueryBodyConformanceEvidenceCatalogV1
  -> one bounded body-conformance row per selected Query declaration
  -> VerifiedCallableBodyConformanceCatalogV1
```

The evidence catalog is the landed `return me` structural-safety and Query
Home no-transfer receipt. General effect/control/Home-flow conformance remains
`NoSafeSlice` and is a separate future design row. This I0 does not widen the
body-shape vocabulary or turn an empty effect list into a universal proof.

## Inputs and authority

The issuer consumes only:

```text
VerifiedDeclaredInstanceMethodContractCatalogV1
VerifiedQueryBodyConformanceEvidenceCatalogV1
```

It does not consume AST, raw syntax, body facts, MIR, `EffectMask`,
`FunctionSignature`, ownership SSA, target, Recipe, or runtime state. It never
reissues Query behavior, Home ABI, semantic signature, ABI representation, or
public result meaning.

The existing evidence issuer remains the bounded evidence authority for this
cohort. A future general evidence design may split structural effect/control
and Home-flow issuers, but that is not opened by this row.

## Exact behavior check

Every selected contract must already declare:

```text
DeclaredQueryBehaviorV1::ReceiverDirectReadNoEffects
```

The conformance issuer compares that declaration with the evidence row. It
does not infer the behavior from the body and does not infer semantic result
type from `return me`.

The evidence row must prove the existing bounded receipt:

```text
exact declaration/owner/body identity
exact parser provenance and resolver brand
exact bounded return/me/relation coverage
receiver Home = Handle
parameters Home = []
result Home = Trivial
Home transfer = None
```

## Identity and coverage

Pair rows by aggregate-owned declaration identity, never by `zip`, vector
position, name, inventory ordinal, `FunctionOrigin`, or numeric owner ID. The
identity includes the same resolver brand, parser provenance, nominal Box,
Box statement site, and method member site. A private identity view or
catalog-owned iterator may be added to the existing aggregate; conformance
must not build a second declaration/home/query join authority.

The result catalog must reject:

```text
missing selected Query row
duplicate conformance/evidence row
foreign parser provenance or resolver brand
foreign nominal Box or source site
extra evidence for an unselected declaration
```

Sparse `Query / non-Query / Query` source order is valid. Non-Query rows are
unselected and receive no default facts or conformance.

## Dispositions

```text
exact declared Query contract + exact evidence -> Candidate
evidence Declined -> nonconforming; abort publishable catalog
evidence NoSafeSlice -> propagate NoSafeSlice
opaque/incomplete evidence -> Unresolved
identity/behavior/coverage mismatch -> Rejected
```

No fallback, retry, default evidence, or alternate provider is allowed.

## Products

The only product opened by this row is:

```text
VerifiedCallableBodyConformanceV1
VerifiedCallableBodyConformanceCatalogV1
```

The later `VerifiedConformantCallableCatalogV1` full same-brand publication
co-seal remains a separate row. This I0 does not open resolver targets,
source-bound call relations, Recipe/CallSlot, Builder/MIR/CFG/PHI, physical
ABI, module publication, or production selection.

## Acceptance tests

```text
one exact Query positive
Query / non-Query / Query sparse subset
missing/duplicate/foreign evidence rejection
parser provenance/resolver brand mismatch rejection
nominal Box/source-site mismatch rejection
declared behavior mismatch rejection
NoSafeSlice propagation
no old source_instance_result_contract import or authority
```

Use real resolver fixtures. Do not add forged `Verified*` constructors or
test-only authority shortcuts. Keep the implementation module and tests under
the 800-line rule, update the resolver README/reference/task pointers in the
same slice, run the focused gate and pointer guard, then commit and push.

