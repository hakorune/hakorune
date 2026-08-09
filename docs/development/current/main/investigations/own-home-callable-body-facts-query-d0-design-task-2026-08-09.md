---
Status: design stop after owner-binding I0
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-owner-binding-i0-implementation-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-FACTS-QUERY-D0

## Purpose

Design the first body-behavior observer over the branded owner link. This row
must consume the owner relation; it must not observe AST/body syntax by name,
re-run Query selection, or infer a replacement public contract.

```text
VerifiedInstanceMethodBodyOwnerCatalogV1
  -> private body observer
  -> atomic VerifiedCallableBodyFactsCatalogV1
```

## Required design decisions before I0

```text
source observation authority:
  exact resolved-function/body-root product carried by owner link

accepted cohort:
  bounded direct Query receiver-read/return shape only

facts:
  source-derived receiver read and ordinary return facts

conformance:
  separate later product; facts do not prove declared Query conformance

identity:
  owner-link row / declaration identity only; no name/ordinal repair
```

The observer must state where it obtains resolved expression/statement facts
from `VerifiedResolvedFunctionV1` and its body roots. If the required syntax or
resolved inventory is absent, remain `NoSafeSlice` and add an issuer design;
do not reopen parser AST transport or add a second owner.

## Boundary decision

The owner link is the reusable body-source/owner authority for the supported
direct instance-method cohort. Query is only the first observer projection:

```text
general body-source + exact carrier
  -> VerifiedInstanceMethodBodyOwnerCatalogV1
  -> private Query body observer
  -> VerifiedCallableBodyFactsCatalogV1
```

The facts observer consumes the owner-link rows and the exact resolved
function/body-root receipt already borrowed by each row. It does not inspect
the parser envelope, reconstruct a Query subset from raw rune syntax, or pair
rows by name, inventory ordinal, vector position, `FunctionOriginV1`, or owner
number. A later non-Query body observer may reuse the owner-link product only
through its own accepted profile boundary.

The first bounded body shape is deliberately narrow: a receiver read and one
ordinary return from a direct Query instance method. The facts product records
source-derived behavior only; it does not issue Home, Query, signature, ABI,
or conformance meaning. The facts catalog is catalog-level and atomic so a
missing or duplicate fact cannot be hidden by a partial row.

If `VerifiedResolvedFunctionV1` plus its carried body-root/coverage receipt
does not expose enough resolved expression/statement information to observe
this shape without AST re-scan, the result is development `NoSafeSlice`. Add
the missing resolver issuer as a separate D0/I0 instead of reopening parser
transport or manufacturing an empty facts receipt.

## Forbidden in this row

```text
AST re-scan or name lookup
Query/Home/signature re-issuance
body -> public contract inference
conformance catalog
resolver target / source-bound call / Recipe / CallSlot
Builder / MIR / CFG / PHI / physical ABI
fallback/retry/provider/runtime
```

## Required negative matrix

```text
foreign owner-link catalog
wrong body root or source kind
missing/duplicate body fact site
opaque/unavailable resolved body evidence
write, allocation, IO, FFI, suspension, non-local control
ordinary return vs error/failure escape distinction
```

After D0, open only a bounded I0 and stop before
`CALLABLE-CONTRACT-CONFORMANCE-D0`.
