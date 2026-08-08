---
Status: closed — accepted design; implementation opened in the bounded child task
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-abi-d0-design-task-2026-08-09.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
---

# OWN-HOME-ABI0-S0

## Decision

The bounded Home ABI has one semantic issuer and one non-`Clone` catalog. The
issuer consumes the already-issued resolver declaration catalog by value and a
resolver-owned capability environment whose resolver brand must match that
catalog. It returns one Home ABI row per declaration and never publishes a
partial receiver, parameter, or result receipt.

```text
VerifiedInstanceMethodDeclarationCatalogV1
  + ResolverHomeCapabilityEnvironmentV1
      │
      ▼
CallableHomeAbiIssuerV1::issue(...)
      │
      ▼
VerifiedDeclaredInstanceMethodHomeCatalogV1
  ├─ owned declaration catalog
  └─ VerifiedHomeAbiV1[]
```

`HomeRelationBrandV1` is only a relation-batch/provenance brand. It is not a
resolver catalog brand, nominal Box type identity, or source declaration
identity. The ABI row stores the resolver brand/site and the relation-batch
brand separately; no relation ordinal is used as a declaration key.

## Bounded classifier

The first resolver-owned capability environment has one explicit schema:

```text
ResolverHomeCapabilityEnvironmentV1
  resolver_brand: ResolverCatalogBrandV1
  schema: I64UnitTrivialV1
  relation_batch_brand: HomeRelationBrandV1
```

The schema is issued only by the resolver-owned environment factory. It is not
constructed from method names, Query syntax, runtime tags, `MirType`,
`FunctionSignature`, `ExactTrivial*Abi`, body shape, or backend layout.

For the already-landed semantic declaration rows, the classifier rule is:

```text
ordinary instance receiver -> Handle
semantic I64/Unit parameter -> Trivial
semantic Unit result       -> Unit
semantic I64 result        -> Trivial
```

The current declaration issuer exposes only `I64` and `Unit`; unsupported
source types therefore reject at that earlier declaration boundary. No
`Unknown` enum or default-to-`Trivial` path is added here. When the semantic
type vocabulary grows, generic/Any/composite/recursive capability must enter
as `Unresolved` until an explicit classifier rule exists.

`CallableContractSyntaxV1::Query` is not read by this issuer. Query behavior
later co-seals with the already-issued Home ABI and never reissues its axes.

## Home ABI row and co-seal invariants

`VerifiedHomeAbiV1` stores only semantic source-bound identity and the Home
relations:

```text
resolver catalog brand
nominal Box type id
Box statement ordinal
method member ordinal
relation-batch brand (provenance only)
receiver demand
ordered parameter demands
result relation
```

It does not store `ValueId`, `BasicBlockId`, Home Flow state, Ownership SSA,
runtime handles, reference counts, physical ABI, target pointers, Recipe keys,
or Query/body facts.

The issuer rejects before any Builder effect when:

```text
catalog brand != capability-environment resolver brand
nominal Box/site differs from the declaration row
declaration is static or outside the ordinary instance cohort
duplicate declaration/site or Home row
Home row count/order differs from declaration count
parameter demand count differs from semantic parameter count
result relation differs from semantic result type
relation batch is foreign or detached
```

The aggregate is the only later input. A caller cannot obtain a standalone
receiver demand and combine it with another declaration.

## Disposition and stop lines

```text
NoSafeSlice:
  canonical classifier/issuer is not implemented (development state)

Unresolved:
  future semantic type/capability rule is unavailable or opaque

Declined:
  fully observed declaration is outside the bounded ordinary-instance cohort

Rejected:
  foreign brand/site, duplicate, static/instance mismatch, detached relation
  batch, signature/cardinality mismatch, or forged/contradictory receipt

Candidate:
  exact same-brand declaration and complete Home rows co-sealed
```

The current `I64|Unit` declaration issuer may report unsupported types before
this row; that is not a reason to add a guessed `Unknown` Home value.

## Acceptance matrix for the later implementation slice

Positive fixtures:

```text
length(): i64       -> receiver Handle, params [], result Trivial
reset(): Unit       -> receiver Handle, params [], result Unit
read(i64): i64      -> receiver Handle, params [Trivial], result Trivial
```

Negative fixtures:

```text
catalog/environment resolver-brand mismatch
foreign nominal Box or method source site
static declaration
duplicate/missing/reordered Home row
wrong parameter/result cardinality
foreign relation-batch brand
unsupported future type without classifier rule
```

Focused tests must prove that Query presence is irrelevant to Home issuance,
that no partial public constructor exists, and that a fresh catalog can be
issued only through the single issuer. The module stays below the 760-line
split trigger; tests remain in a separate file.

## Explicit non-claims

```text
No Query behavior or body conformance
No Home Flow / Ownership SSA / transfer failure
No take/share/release grammar
No field/container/projection destination
No generic/composite physical representation
No resolver target or source-bound Call relation
No Recipe/CallSlot, Builder/MIR, physical ABI, provider/runtime, fallback,
or production caller
```

## Ordered follow-up

```text
OWN-HOME-ABI0-S0 (this design stop)
  -> OWN-HOME-ABI0-S0 implementation: home_abi.rs + focused tests
  -> declared Query behavior + Home aggregate co-seal
  -> conformance catalog
  -> resolver target / source-bound Call relation
```

The implementation slice must update the owner README, the language ownership
and callable-contract references, the focused negative matrix, and current
pointers in the same commit. No Home production activation is implied.
