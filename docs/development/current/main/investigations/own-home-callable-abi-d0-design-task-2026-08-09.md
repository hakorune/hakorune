---
Status: accepted — complete ABI contract retained; bounded entry-flow loan added
Date: 2026-09-29
Parent: `docs/development/current/main/investigations/hakorune-home-ownership-task-2026-08-04.md`
Authority: `docs/development/current/main/design/ownership-home-model-ssot.md`
---

# OWN-HOME-CALLABLE-ABI-D0

## Decision

The callable Home ABI has one semantic authority and one publication boundary.
The existing resolver declaration/signature catalog is consumed by a single
Home ABI issuer together with an explicit resolver-owned capability
classification environment. The issuer returns one non-`Clone` declared
contract catalog that owns the declaration catalog and one exact
`VerifiedHomeAbi` row per declaration.

```text
VerifiedInstanceMethodDeclarationCatalogV1
  + resolver-owned Home capability classification environment
      │
      ▼
CallableHomeAbiIssuerV1::issue(...)
      │
      ▼
VerifiedDeclaredInstanceMethodHomeCatalogV1
  ├─ declaration catalog (owned, unchanged semantic source facts)
  └─ VerifiedHomeAbi[] (one-to-one, same declaration brand/site)
```

The aggregate does not create a second receiver/parameter/result vocabulary.
`VerifiedHomeAbi` rows are the sole call-site ownership authority. Query
behavior, body conformance, targets, Recipe/CallSlot, Builder/MIR, physical
ABI, provider/runtime, and grammar activation remain later boundaries.

## Exact bounded cohort

The first implementation cohort is an ordinary instance method with the
already-landed semantic signature classes:

```text
receiver demand:
  Handle

parameter demands:
  one `Trivial` row per semantic `I64`/`Unit` parameter

result relation:
  `Unit` for semantic Unit
  `Trivial` for semantic I64
```

This is a classifier rule, not a fixture default. The Home capability
classification environment is resolver-owned, carries the same declaration
catalog brand/source identity, and must explicitly classify every type row
used by the issuer. Method names, `CallableContract(query)`, runtime tags,
`MirType`, `FunctionSignature`, `ExactTrivial*Abi`, and backend layout never
classify Home capability.

`CallableContractSyntaxV1::Query` is not read by this issuer. A missing Query
does not prevent an ordinary declaration Home ABI. A later callable aggregate
issuer co-seals Query behavior with the already-issued Home ABI; it does not
reissue receiver, parameter, or result demands.

## Product and co-seal invariants

`VerifiedHomeAbi` and the aggregate are non-forgeable/non-`Clone` semantic
products. The issuer rejects before any Builder effect when:

```text
declaration catalog brand != classifier environment brand
Box/method source site or nominal type differs
Home row count != declaration count
duplicate declaration or Home row exists
parameter demand count != semantic parameter count
static declaration enters the instance cohort
classifier provenance is missing, foreign, or unknown
result relation is incompatible with the semantic signature
```

The aggregate is the only product that may be handed to a later target or
call-site resolver. Callers cannot obtain a standalone receiver demand,
parameter demand, or result relation and combine it with another declaration.

### Source entry-flow projection (2026-09-29)

The complete call-site ABI rule above remains unchanged. The same
`CallableHomeAbiIssuerV1` owner may additionally lend **entry-only** demands
to source Home Flow, without publishing a complete `VerifiedHomeAbiV1`.
This is an accepted interface design; the current-batch entry path is not yet
implemented. Its bounded production task is
`MIRBUILDER-GATE1-INSTANCE-ENTRY-HOME-S0` in the
[Gate-1 card](mirbuilder-gate1-callable-loop-string-indexof-s0-2026-09-27.md).

The owner retains one current verified callable-batch loan, its exact common
parameter-contract catalog, and same-declaration resolver capability evidence.
It checks parser/batch/declaration identity, instance mode, exact receiver
binding and complete ordered parameter rows before lending entry demands.
The existing ordinary-instance classifier supplies receiver `Handle`; parameter
demands are projections of the common catalog's `home_demand()`, with original
parameter kinds retained. Neither the scanner nor a second issuer classifies
them. Borrowed Map/Text representation is not collapsed into generic Handle.

The entry loan remains scoped to that owner aggregate and source invocation;
it cannot be detached, paired by an owner key, converted into target/result
authority, or reused with another declaration. Missing/foreign/duplicate
evidence does not yield an empty entry. Captures and consuming receiver demands
stay unsupported. Handle entries introduce no owned Home or release duty.

For this projection the result remains explicitly unresolved: no missing return
annotation becomes Unit, no complete Home ABI row is created, and no call-site
ABI publication is allowed. Existing complete I64/Unit issuance is unchanged.
This narrow projection does not grant general CFG Home Flow, field destruction,
new argument/result ABI, Query, or backend admission. Their owners remain
separate; source flow still rejects statements it cannot account for.

## Disposition and stop lines

```text
NoSafeSlice:
  canonical Home classifier/issuer is not implemented (development state)

Unresolved:
  generic/Any/composite/recursive type or capability evidence is unavailable

Declined:
  fully observed declaration is outside the bounded ordinary-instance cohort

Rejected:
  foreign brand/site, duplicate, forged receipt, static/instance mismatch,
  signature mismatch, or conflicting source identity

Candidate:
  exact same-brand declaration and complete Home capability rows co-sealed
```

`NoSafeSlice` is never a source disposition. Unknown capability never defaults
to `Trivial`, `Unique`, or `Shared`. The issuer does not infer public ABI from
body shape and does not inspect Home Flow, CFG, runtime counts, or physical
representation.

## Explicit non-claims

The explicitly selected entry-flow S0 above is the sole bounded production
exception. This D0 authorizes no additional code or production caller for:

```text
general Home Flow / Ownership SSA (including per-exit CFG flow)
take/share/release grammar
field/container/projection destinations
Shared/Weak/Unique physical representation
generic/composite capability classification
result Home forwarding or temporary lifetime extension
Query behavior or body conformance
resolver target or source-bound Call relation
Recipe/CallSlot, Builder/MIR, provider/runtime, fallback, or publication
```

The existing C′ terminal-finalization and `release root` decisions remain the
only lifecycle/source direction. This row does not reopen `drop`, direct
`obj.fini()`, or an alternate Home authority.

## Ordered follow-up

The sequence below concerns complete call-site integration. The entry-only
Gate-1 row is selected separately by CURRENT_STATE; it does not reopen this chain.

```text
OWN-HOME-CALLABLE-ABI-D0                  closed (this card)
  -> OWN-HOME-RELATION0-S0                passive branded relation vocabulary
  -> OWN-HOME-ABI0-S0                     design stop, then exact I64/Unit instance Home rows
  -> declared Query + Home aggregate co-seal
  -> body conformance catalog
  -> resolver target / source-bound Call relation
```

Every implementation slice must update its owner README, the relevant
`docs/reference/**` receipt, focused negative matrix, and current pointers in
the same commit. Only the explicitly selected entry-flow S0 has bounded
production scope; no broader Home production activation is implied.
