---
Status: closed — accepted design; implementation opened in the bounded child task
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-abi0-s0-implementation-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# RESOLVER-DECLARED-QUERY-BEHAVIOR-D0

## Decision

Query is a declaration-level behavioral obligation, not an ownership or
physical ABI classifier. A single typed Query issuer consumes the already
issued resolver declaration catalog by reference and issues one non-`Clone`
behavior catalog only when the bounded declaration set has explicit
`CallableContractSyntaxV1::Query` rows. The issuer emits a non-empty selected
Query subset; non-Query declarations are outside this behavior family and are
never represented by a fabricated default row.

```text
VerifiedInstanceMethodDeclarationCatalogV1
  + typed Query syntax already carried on selected declarations
      │
      ▼
DeclaredQueryBehaviorIssuerV1::issue(&catalog)
      │
      ▼
VerifiedDeclaredQueryBehaviorCatalogV1
      │
      ▼ later aggregate co-seal
VerifiedDeclaredInstanceMethodContractCatalogV1
  + VerifiedDeclaredInstanceMethodHomeCatalogV1
```

The Query product stores only resolver brand/site, an optional rune ordinal for
diagnostics, and the Query behavioral obligation. It never reissues or copies
semantic signatures, receiver/parameter/result Home relations, relation-batch
brands, body facts, or physical ABI from another product.

## Bounded behavior

The first Query obligation is:

```text
allowed:
  exact receiver direct-state reads
  ordinary return

forbidden:
  receiver/global writes
  Home transfer/share/end/escape
  allocation
  IO/FFI
  Fault/throw/non-local failure propagation
  suspension
  non-local control transfer
```

`Pure` remains a separate effect family and forbids receiver reads. Query is
selected from typed syntax only; method names, body shape, MIR `EffectMask`,
runtime tags, and physical ABI do not issue the declaration behavior.

## Co-seal and disposition

The later declared-contract aggregate must verify:

```text
Query behavior brand == Home ABI resolver brand
Box/method source site matches exactly
 one behavior row per selected Query declaration
 selected Query declaration count is non-zero
 Home ABI is present and is not duplicated by Query
```

For this design row:

```text
NoSafeSlice: issuer not implemented
Declined: selected cohort has no Query rows, or the declaration is outside the
  bounded Query family (non-Query rows are not defaulted into the catalog)
Unresolved: typed source row/site is unavailable
Rejected: foreign/duplicate/conflicting behavior or signature/site mismatch
Candidate: exact typed Query row co-sealed with the declaration
```

Mixed-catalog policy is explicit: the issuer may issue the Query catalog for
the exact non-empty Query subset of a declaration catalog. A later aggregate
must pass the same selected Query declarations to the Home co-seal. A caller
that requests an all-row Query cohort instead receives
`Declined(MixedQueryCohort)`; it must not silently omit rows.

Body conformance remains a separate complete-coverage verifier. An annotation
does not prove the method body.

## Explicit non-claims

```text
No Home ABI issuance or Home Flow
No take/share/release grammar
No target/source-bound Call relation
No Recipe/CallSlot/Builder/MIR/physical ABI/runtime
No provider/fallback/retry/production activation
```

## Follow-up

The implementation row is:

`docs/development/current/main/investigations/own-home-query-behavior-i0-implementation-task-2026-08-09.md`

It adds a dedicated `query_behavior.rs` and test file, keeps the typed parser
syntax as the only source input, and updates the owner README and
callable-contract reference in the same commit. After that bounded row closes,
open the aggregate co-seal design. No Query body lowering starts from this
card alone.
