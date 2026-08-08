---
Status: parked — frontend ordered method inventory prerequisite
Date: 2026-08-08
Decision: declaration/contract split accepted; implementation remains closed
Parent: `loop-resolver-canonical-callable-contract-d0-design-task-2026-08-08.md`
---

# LOOP-RESOLVER-INSTANCE-DECLARATION-AND-CONTRACT-RECEIPTS-D0

## Decision brief

```text
Decision:
  Do not issue an instance contract from lexical receiver presence or a
  FreeStatic header. Consume the future frontend-owned ordered method source
  capability, then co-seal one declared `CallableContract(query)` with the
  exact nominal Box declaration before any target.

Source authority:
  BoxMethodInventoryV1 Source row, same catalog/compilation brand, nominal
  receiver type identity, ordered parameter/result declarations, ordinary
  receiver Handle rule, and `CallableContract(query)`.

Non-authority:
  BindingKind::Receiver, ReceiverPolicy::DeclaredInstance, HashMap/name order,
  generated/property methods, body inference, Contract/Profile metadata,
  MIR FunctionSignature/EffectMask, physical ABI projections, Builder/runtime
  registries, Recipe/CallSlot, or an empty receipt.

Fail-fast boundary:
  missing ordered source row, missing nominal receiver type, missing declared
  query contract, foreign/duplicate declaration, or conflicting source facts
  keeps the row NoSafeSlice/Unresolved. No target is issued.

Smallest next slice:
  after frontend parity, issue one exact `length(): i64` declared query
  contract with no body conformance or target.

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

### 1. Instance declaration inventory

```text
VerifiedBoxMethodDeclarationV1 {
  catalog/compilation brand,
  enclosing Box identity and declaration site,
  method declaration site and static/instance status,
  ordered parameter declarations,
  declared result declaration,
}
```

It consumes only a `BoxMethodInventoryV1::Source` row. It is immutable,
AST-free after issuance, and cannot be reconstructed from a method or Box
name. Generated/property rows, duplicate/foreign identity, and static/instance
cross-wiring reject before a contract is attempted.

### 2. Typed semantic call-contract receipts

The declaration inventory alone is insufficient. The accepted language
Decision supplies one declared whole-call obligation:

```text
receiver demand: ordinary Handle; no Home transfer/share/end/escape
receiver effect: exact receiver read allowed; no writes
effects/control: no alloc, IO/FFI, Fault, suspension, or non-local control
parameter/result: exact semantic declaration from the method signature
```

The source spelling is `@rune CallableContract(query)`. It is a declared
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

The repository has enough syntax to describe a future instance declaration,
but not enough resolver-owned authority to issue one yet:

```text
ASTNode::BoxDeclaration:
  name, methods: HashMap<String, ASTNode>, is_static, type parameters,
  fields and declaration metadata

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

Therefore the existing syntax is evidence for a future issuer input, not an
issuer itself. The current HashMap has also discarded duplicate declarations,
source order, provenance, and an exact member ordinal. Method/Box strings,
`ReceiverPolicy`, the raw embedded helper, Builder declaration facts, and
owner brand cannot be combined after the fact to manufacture a declaration.
`FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0` owns that correction before the
resolver consumes a source capability.

The declared-contract issuer audit is also closed at `issuer=0` for the current source:

```text
@contract(pure|readonly|no_alloc|no_safepoint):
  parser metadata only; the current EffectPlan consumes only no_alloc and
  no_safepoint and marks the plan `verified = false`

@rune Contract / Profile:
  accepted annotation/metadata surface, not a resolver Pure or lifecycle
  receipt

CallableContract(query):
  accepted language target; parser and resolver issuer remain production 0

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

## Exit criteria before I0

1. Frontend `BoxMethodInventoryV1` and Rust/.hako source parity are closed.
2. The query contract issuer consumes that exact Source row and nominal Box.
3. Receiver demand uses the existing `Handle` vocabulary.
4. Signature owns arity/types; physical ABI remains downstream.
5. One atomic declared-contract co-seal and negative matrix are fixed.
6. Body conformance remains a separate pre-publication gate.

## Ordered follow-up

```text
LANGUAGE-TYPED-CALLABLE-PROFILE-D0                 closed
  -> FRONTEND-ORDERED-BOX-METHOD-INVENTORY-D0
  -> frontend BoxShape/Rust/.hako implementation series
  -> RESOLVER-DECLARED-QUERY-INSTANCE-CONTRACT-I0
  -> RESOLVER-INSTANCE-CALL-TARGET-D0/I0
  -> SOURCE-BOUND-INSTANCE-CALL-D0/I0
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0
```
