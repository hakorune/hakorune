---
Status: accepted design — implementation parked at semantic issuer/type authority
Date: 2026-08-08
Decision: declaration/contract split accepted; semantic issuer implementation remains closed
Parent: `loop-resolver-canonical-callable-contract-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-INSTANCE-DECLARATION-AND-CONTRACT-RECEIPTS-D0

## Decision brief

```text
Decision:
  Do not issue an instance contract from lexical receiver presence or a
  FreeStatic header. Consume the landed parser→resolver source handoff by
  value, issue one resolver-owned semantic declaration/signature under a new
  catalog brand, then let the separate Home issuer provide the exact
  same-declaration `VerifiedHomeAbi` before any target.

Source authority:
  one-shot `ParserBoxResolverSourceHandoffV1`, a new resolver catalog brand,
  resolver-owned nominal receiver type identity, ordered semantic
  parameter/result declarations, typed `CallableContractSyntaxV1::Query`,
  and the separate `VerifiedHomeAbi` issuer.

Non-authority:
  BindingKind::Receiver, ReceiverPolicy::DeclaredInstance, HashMap/name order,
  generated/property methods, body inference, Contract/Profile metadata,
  MIR FunctionSignature/EffectMask, physical ABI projections, Builder/runtime
  registries, Recipe/CallSlot, or an empty receipt.

Fail-fast boundary:
  missing resolver type authority or semantic signature issuer is
  `NoSafeSlice`; a missing ordered source row, foreign/duplicate declaration,
  or conflicting source fact is `Rejected`/`Unresolved`. Absence of the
  optional Query annotation is a later `Declined`, not a declaration-layer
  `NoSafeSlice`. No target is issued.

Smallest next slice:
  design and then implementation of one resolver semantic declaration/signature
  for exact `length(): i64`; Home ABI and Query behavior remain separate
  follow-up rows. No body conformance or target is included here.

Non-claims:
  body conformance, resolver target, Recipe/CallSlot, source-bound relation,
  Home implementation, Builder/MIR, physical calls, provider/runtime
  dispatch, fallback, production, and legacy deletion.
```

## Why the current code cannot issue the contract

`BindingKindV1::Receiver` and `ReceiverPolicyV1::DeclaredInstance` prove only
that a lexical receiver slot exists. They carry no nominal Box type or Home
classification. `CallableHeaderSyntaxViewV1` sees a function header but not the
enclosing Box identity, and `VerifiedCallableIndexV1` is function-only
FreeStatic. The Builder-side instance catalog is not a resolver authority.

`ExactTrivialParameterAbiV1`/`ExactTrivialReturnAbiV1` are source-spelling to
MIR projection helpers. `EffectPlan` is currently unverified rune metadata;
`@rune Contract(pure|readonly)` is not the accepted whole-call issuer.
`EffectMask` and `FunctionSignature` belong to physical MIR. The source
Decision now names `CallableContract(query)`, but parser/source inventory and
resolver issuers remain production zero. Emitting an unconditional Handle or
query envelope would still create a second guessed truth.

## Required source-backed products

### 1. Resolver-owned instance declaration catalog

```text
ResolverNominalTypeEnvironmentV1 {
  fresh resolver catalog/type brand,
  exact nominal Box declaration identities,
}

SemanticInstanceDeclarationIssuerV1::issue(
  ParserBoxResolverSourceHandoffV1,
  ResolverNominalTypeEnvironmentV1,
) -> VerifiedInstanceMethodDeclarationCatalogV1 {
  same resolver brand,
  exact Box/method source sites,
  nominal receiver type,
  instance/static status,
  ordered semantic parameter/result signature,
}
```

The issuer consumes the handoff by value through its single `into_parts` path;
it must not call `boxes()` and clone rows or retain a parser row as a partial
receipt. The resulting declaration catalog is non-Clone, AST-free, and owns a
fresh resolver catalog/type brand. The parser invocation brand remains only as
provenance/membership evidence. It cannot be reconstructed from a method or
Box name, inventory ordinal, Builder catalog, or `FunctionOwnerIdV1` brand.
Generated/property rows, duplicate/foreign/stale identity, static/instance
cross-wiring, and unknown nominal type fail before any contract is attempted.

The bounded positive is `TextLike.length(): i64` with arity zero and semantic
`I64`. Typed Query syntax is carried for the later behavior issuer; it is not
required to issue the declaration/signature product and its absence is a
later `Declined`.

### 2. Typed semantic call-contract receipts

The declaration inventory alone is insufficient. The accepted language
Decision supplies one declared whole-call obligation:

```text
receiver demand: supplied only by same-declaration VerifiedHomeAbi
receiver effect: exact receiver read allowed; no writes
effects/control: no alloc, IO/FFI, Fault, suspension, or non-local control
parameter/result: exact semantic declaration from the method signature
```

The source spelling is `@rune CallableContract(query)`, normalized by the
parser to a typed Query row. It is a declared
obligation, not body proof. `Pure` is stricter and disallows receiver/heap
reads; `length()` is therefore query/readonly behavior, not Pure. Physical
scalar ABI remains a later projection from semantic `i64`.

### 3. Declaration contract and body conformance

The resolver product name must contain `Declared`. The declaration may enter
the target catalog before body lowering so recursion is resolvable, but module
publication later requires a separate
`VerifiedCallableContractConformanceV1`. Body verification checks the declared
meaning; it never infers or rewrites the public contract.

## Read-only source census (2026-08-08)

The bounded Rust parser now has enough syntax and a one-shot handoff to feed a
future instance declaration, but the resolver still lacks the nominal type
authority and semantic signature issuer needed to issue one:

```text
ASTNode::BoxDeclaration:
  name, methods: BoxMethodInventoryV1, is_static, type parameters, fields and
  declaration metadata; the inventory remains passive AST data. The landed
  parser seal and `ParserBoxResolverSourceHandoffV1` are the only source
  authority crossing into the resolver boundary.

ASTNode::FunctionDeclaration:
  method name, params/ParamDecl, declared return type, is_static, attrs

CallableHeaderSyntaxViewV1 / CallableHeaderSourceUnitV1:
  body-free FunctionDeclaration view; the catalog rejects non-function
  top-level statements and `VerifiedCallableIndexV1` is FreeStatic-only

FunctionSyntaxViewV1:
  maps `is_static` to a lexical ReceiverPolicy, but carries no enclosing
  Box identity or nominal receiver type

VerifiedCallableHeaderSourceUnitV1::embedded_function:
  private raw `(statement_index, method_key)` helper; it is not a sealed
  Box/method declaration relation or a reusable resolver target

PreparedNormalProgramDeclarationFactsV1 / CompilationContext:
  Builder-owned declaration/field/static facts; not resolver authority

FunctionOwnerIdV1::compilation_brand:
  invocation-local membership brand, not source Box identity or type identity
```

Therefore the passive inventory is still not an issuer. R1-R5 fixed ordering
and compatibility transport, and the bounded Rust source-seal/handoff I0 now
closes the parser boundary. Hako/selfhost and compatibility cohorts remain
outside this I0. Method/Box strings, raw `ExplicitSource`, JSON,
`ReceiverPolicy`, the embedded helper, Builder declaration facts, and
`FunctionOwnerIdV1::compilation_brand` cannot be combined after the fact to
manufacture a declaration. The next issuer must consume the handoff by value,
issue a fresh resolver catalog brand, and retain the parser brand only as
provenance/membership evidence.

The declared-contract issuer audit is also closed at `issuer=0` for the current source:

```text
@contract(pure|readonly|no_alloc|no_safepoint):
  parser metadata only; the current EffectPlan consumes only no_alloc and
  no_safepoint and marks the plan `verified = false`

@rune Contract / Profile:
  accepted annotation/metadata surface, not a resolver Pure or lifecycle
  receipt

CallableContract(query):
  typed parser carriage is landed for the bounded Rust handoff; semantic
  declaration and behavior issuers remain production 0

receiver Handle / query effect-control envelope:
  no source-backed typed issuer in the current resolver catalog
```

`LANGUAGE-TYPED-CALLABLE-PROFILE-D0` is now closed by the explicit language
Decision. Reusing old metadata, translating `EffectMask` into source meaning,
or bypassing the ordered frontend inventory remains forbidden.

## Disposition and precedence

```text
Rejected:
  foreign brand/frame/site, duplicate/ambiguous declaration, forged receipt,
  static/instance mismatch, or conflicting identity.

Unresolved:
  contract is present but the ordered source row, nominal Box type, or exact
  declaration relation is unavailable.

Declined:
  no query contract in the exact-family observer, or fully observed outside
  the first instance `(): i64` cohort.

Candidate:
  the source declaration and declared query contract co-seal under the same
  catalog/compilation brand.
```

Use `Rejected > Unresolved > Declined > Candidate`. `NoSafeSlice` remains a
development state and must not be converted into a source disposition.

## Exit criteria for the next semantic-declaration I0

1. The semantic issuer consumes one `ParserBoxResolverSourceHandoffV1` by
   value and cannot drop its parser brand while retaining rows.
2. A fresh `ResolverNominalTypeEnvironmentV1`/catalog brand issues the exact
   nominal Box and method declaration relation; parser brand is provenance
   only. Missing/unknown type authority is `NoSafeSlice` or `Unresolved`, never
   a guessed type.
3. One semantic signature owns ordered arity/types without
   `ExactTrivial*Abi`/`MirType` authority.
4. Home ABI is a later, separate issuer; its bounded `Handle`/empty-parameter/
   `Trivial` relation must come from the Home classifier and same-declaration
   brand, never from fixture defaults. Query behavior is independent and
   absence of Query remains `Declined`.
5. One declaration aggregate and negative matrix are fixed; no target,
   Recipe, body conformance, or physical ABI is opened.

## I0 guard and acceptance matrix

The semantic declaration implementation gets a dedicated resolver module and
must remain below the 760-line split trigger (800 hard boundary). Its imports
must exclude `ExactTrivial*Abi`, `MirType`, `FunctionSignature`, `EffectMask`,
Builder, Recipe, CallSlot, target, provider, and runtime modules.

The focused matrix must prove:

```text
positive:
  TextLike.length(): i64, arity 0, same resolver brand/site

rejected:
  foreign/stale/duplicate Box or method site
  static/instance mismatch
  generated/property/compatibility row
  mutated inventory ordinal used as source identity
  forged or partial declaration receipt

unresolved / no-safe-slice:
  unknown nominal Box type
  missing resolver type environment
  missing semantic signature issuer

ownership:
  handoff reused after one consuming issue
  row cloning/partial re-issuance API absent from the issuer path
```

`CallableContract(query)` may be present or absent in this row. Absence is
preserved as a later behavior `Declined`; declaration/signature issuance does
not require Query and must not fabricate Home, effect, or ABI facts.

## Ordered follow-up

This is a resolver-declaration view only. The complete executable order,
including old-target retirement, publishable-catalog co-seal, physical ABI
projection, and same-slice reference updates, is owned only by
`callable-contract-and-instance-call-implementation-task-map-2026-08-08.md`.

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0                 closed
  -> RESOLVER-BOX-SOURCE-HANDOFF-D0/I0             closed (bounded Rust)
  -> semantic declaration/signature D0/I0
  -> OWN-HOME-CALLABLE-ABI-D0 -> RELATION0-S0
  -> ABI0-S0 / Query behavior / declared aggregate
  -> RESOLVER-INSTANCE-CALL-TARGET-D0/I0
  -> SOURCE-BOUND-INSTANCE-CALL-D0/I0
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0
  -> publishable catalog / physical ABI projection
  -> production activation only after complete conformance
```
