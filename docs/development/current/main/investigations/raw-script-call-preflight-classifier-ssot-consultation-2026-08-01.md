---
Status: consultation
Scope: Script `FunctionCall` semantic admission boundary only
Decision requested: reusable preflight receipt, or explicit R4 compatibility retention
---

# Script FunctionCall Preflight: Design Consultation

## Question

The normal Script pipeline has reached a design stop.  Can the existing raw
`FunctionCall` preflight publish one immutable, reusable classified receipt
for a narrow Script Complete closure without creating a second classifier or
moving operational call authority?

If not, should Script `FunctionCall` remain explicitly retained by
`ExistingRootLower` through R4?

## Current production path

```text
selected Script Program item
-> normal_script_root_demand_window
-> Deferred(ExistingRuntimeResponsibility)
-> NormalScriptRootLoweringMode::Deferred
-> RawInvocationSourceTransportV1::script_root(())
-> raw expression dispatcher
-> PreparedRawFunctionPreflightV1::prepare
-> existing call completion
```

`PreparedRawFunctionPreflightV1` is currently the single pre-effect route
selector.  The same `FunctionCall(name, arguments)` surface chooses:

```text
weak reject
explicit extern
Brand constructor
TypeOp
Math
FastMem intrinsic
str normalization
ordinary resolved call
```

It also observes Builder/catalog/header/environment state before ordinary
argument descent.  Existing RootLower then performs header lookup and call
emission through its own ports.

## What is already true

- Script has one Program-root source identity, sparse demand window, shared
  semantic forest/projection, and one Complete/Deferred choice per request.
- Complete may use existing lower owners once; it must not retry or fall back.
- Function/Lambda shadow resolution already has a callable-index-aware path,
  but Script sealing does not receive that index today.
- The raw preflight already owns special-name classification.  Reimplementing
  that match in Script admission would be a second authority.

## Failed candidate

```text
Catalog-resolved ordinary FunctionCall
-> loan VerifiedCallableIndexV1 into Script semantic admission
-> issue a direct-call receipt
-> existing lower once
```

This is not safe as written.  A callable-index loan cannot establish that the
same spelling is not `weak`, extern, Brand, TypeOp, Math, FastMem, or `str`.
It also does not replace the existing RootLower header/environment observation.
Adding a Script-side exclusion table or a second header lookup would create a
second classifier/authority.

## Required answer

Please choose one, with a concrete ownership graph and failure contract.

### A. Reusable preflight receipt (possible)

The existing `PreparedRawFunctionPreflightV1` (or a private neutral core it
already owns) can classify once, before effects, and produce an immutable
receipt for exactly the ordinary resolved route.  Script admission may borrow
that receipt; RootLower consumes the same route decision once.  Explain:

- exact issuer stage relative to CatalogSeal/CatalogInstall/RootLower;
- whether the receipt includes a callable header or only a key;
- how special routes remain classified exactly once;
- how argument source demands and header lookup stay single-authority;
- how source diagnostics keep their existing RootLower precedence;
- one real positive `.hako` fixture that does not require another family.

### B. Explicit R4 retention (recommended if A cannot be made atomic)

`FunctionCall` remains under `ExistingRootLower` through R4.  State the
sunset/retention contract and why the preflight is an indivisible operation
authority rather than a source-only admission seam.

## Non-negotiable constraints

```text
no synthetic AST or reparse/clone
no Script-side special-name classifier
no second header lookup
no Complete -> Deferred downgrade
no semantic rejection -> raw retry
no movement of call emission, effect, ABI, or result/type publication
no activation of MethodCall/New/Field/Index/RecordUpdate in this decision
raw/reference behavior unchanged
all touched source/check files < 800 lines
```

## Existing stop evidence

```text
RAW-SCRIPT-NEXT-NAMED-FAMILY0-D0
  NoSafeSlice: residual Call/Object, Loop/JoinIR, EnumMatch, and
  GroupedAssignment all need competing operational authority.

RAW-SCRIPT-CALL-OBJECT-OWNER-BOUNDARY0-D0
  NoSafeSlice: all Call/Object surfaces co-own route preflight and lowering.

RAW-SCRIPT-DIRECT-CALL-CATALOG-RECEIPT0-D0
  NoSafeSlice: catalog loan cannot replace special-route classification or
  RootLower header observation.
```

Do not propose a fourth census of the same boundary.  Resolve the semantic
unit and authority question above, or recommend R4 retention.
