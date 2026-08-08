---
Status: active — design stop; implementation unopened
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
`CallableContractSyntaxV1::Query` rows.

```text
VerifiedInstanceMethodDeclarationCatalogV1
  + typed Query syntax already carried on each declaration
      │
      ▼
DeclaredQueryBehaviorIssuerV1::issue(&catalog)
      │
      ▼
VerifiedDeclaredQueryBehaviorCatalogV1
      │
      ▼ later aggregate co-seal
VerifiedDeclaredInstanceMethodContractV1
  + VerifiedDeclaredInstanceMethodHomeCatalogV1
```

The Query product stores only resolver brand/site and the Query behavioral
obligation. It never reissues or copies receiver/parameter/result Home
relations from `VerifiedHomeAbiV1`.

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
one behavior row per selected declaration
Home ABI is present and is not duplicated by Query
```

For this design row:

```text
NoSafeSlice: issuer not implemented
Declined: declaration lacks Query or is outside the bounded cohort
Unresolved: typed source row/site is unavailable
Rejected: foreign/duplicate/conflicting behavior or signature/site mismatch
Candidate: exact typed Query row co-sealed with the declaration
```

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

The implementation row must add a dedicated `query_behavior.rs` and test file,
keep the typed parser syntax as the only source input, update the owner README
and callable-contract reference in the same commit, then open the aggregate
co-seal design. No Query body lowering starts from this card alone.
