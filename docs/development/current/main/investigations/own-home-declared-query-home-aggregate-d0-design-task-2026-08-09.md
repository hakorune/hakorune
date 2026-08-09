---
Status: closed — accepted design; implementation opened in the bounded child task
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-query-behavior-i0-implementation-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# DECLARED-QUERY-HOME-AGGREGATE-D0

This row defines the one atomic co-seal that joins the already-landed Query
behavior catalog and Home ABI catalog. The Home catalog is the sole owner of
the declaration catalog; the aggregate consumes it by value and never accepts
a third declaration catalog or reissues any axis.

```text
VerifiedDeclaredInstanceMethodHomeCatalogV1
  owns declaration catalog + all Home rows
VerifiedDeclaredQueryBehaviorCatalogV1
  owns the exact non-empty Query subset rows
      │
      ▼
DeclaredInstanceMethodContractIssuerV1::issue(home, query)
      │
      ▼
VerifiedDeclaredInstanceMethodContractCatalogV1
  owns home catalog + query catalog
  owns only their same-declaration relational co-seal
```

The aggregate's only new truth is compatibility of existing receipts. It must
not create a second Home axis, Query effect axis, semantic signature, body
fact, or physical ABI.

## Exact co-seal invariants

The issuer rejects before any later consumer when:

```text
home and query resolver catalog brands differ
either catalog is empty
Home declaration catalog is not internally aligned with Home ABI rows
each Query row does not match exactly one declaration by nominal Box identity
  + Box statement ordinal + method member ordinal
the matching Home row has a different exact identity
Query rows are duplicated or not in declaration order
selected Query coverage is detached from the Home declaration catalog
```

The Query catalog may be a strict non-empty subset of a larger Home catalog;
non-Query declarations are outside this behavior family. A strict all-row
Query cohort is a separate caller policy and must decline a mixed catalog
instead of silently filling missing behavior rows.

The aggregate stores the two owned catalogs and exposes borrowed projections.
It does not copy `HomeDemandV1`, `HomeResultRelationV1`, semantic parameter or
result classes, `HomeRelationBrandV1`, body facts, `EffectMask`, or physical
ABI. The Home relation-batch brand remains provenance only.

## Disposition and boundaries

```text
Candidate:
  non-empty Query subset and Home catalog pass exact same-brand/site/order
  co-seal
Declined:
  no selected Query rows or strict all-row mixed cohort
Unresolved:
  future source/capability coverage is unavailable
Rejected:
  foreign, duplicate, stale, misordered, or cardinality-mismatched receipts
NoSafeSlice:
  aggregate issuer is not implemented (development state only)
```

Body conformance remains a separate complete-coverage verifier. Recursive
target resolution may use declared contracts later, but this row does not open
targets, CallSlot, Recipe, Builder/MIR, physical ABI, runtime, provider,
fallback, or production selection.

The implementation row is:

`docs/development/current/main/investigations/own-home-declared-query-home-aggregate-i0-implementation-task-2026-08-09.md`

It must keep the Home catalog as declaration owner, add only defensive
identity/coverage checks, and update the owner README and language reference
in the same closeout.
