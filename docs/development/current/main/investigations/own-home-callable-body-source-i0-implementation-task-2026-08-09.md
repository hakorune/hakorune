---
Status: parked — opens only after `CALLABLE-BODY-SOURCE-AUTHORITY-D0` closes
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-source-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-SOURCE-AUTHORITY-I0

## Scope

Implement only the resolver-only, behavior-free body-source capability for
one ordinary top-level Rust Box and one explicit direct instance method:

```hako
box TextLike {
    @rune CallableContract(query)
    length(): i64 {
        return 0
    }
}
```

This row proves that the exact method body is present, belongs to the exact
`SourceBoxMethodSiteV1`, and has complete ordered body-root coverage. It does
not prove Query conformance and does not connect to target, Recipe/CallSlot,
Builder, MIR, runtime, provider, or production.

## Required architecture

Add dedicated parser/resolver modules. Do not grow `source_seal.rs`,
`source_authority.rs`, or `parser/mod.rs` past the 760-line split trigger.

```text
parser private:
  ParserBoxBodySourceEnvelopeV1
  ParserBoxMethodBodySourceRowV1
  one-shot syntax/body callback

resolver:
  VerifiedInstanceMethodBodySourceCatalogV1
  BodySourceIssueV1
  canonical by-value issuer
```

The existing `ParserBoxResolverSourceHandoffV1` remains declaration-only and
AST-free. The body envelope is created by the same rich parse transaction and
is the only pairing authority for body syntax plus that handoff. No caller
may supply an AST, method name, inventory ordinal, or body index separately.

The resolver issuer borrows the existing
`VerifiedDeclaredInstanceMethodContractCatalogV1`; it must not consume,
clone, or rebuild its Home/Query/declaration catalogs. The issued body-source
catalog is non-`Clone`, AST-free, and contains only exact source/body
identity and ordered coverage.

## Acceptance

Positive:

```text
one direct `length(): i64` body
same parser invocation brand
same resolver/catalog anchor
exact direct SourceBoxMethodSiteV1
one body root
ordered body item sites
complete cardinality
```

Negative matrix:

```text
foreign parser brand                         -> Rejected
foreign resolver/declaration anchor          -> Rejected
method source site mismatch                  -> Rejected
missing body row                             -> Rejected
duplicate body row                           -> Rejected
body/root cardinality mismatch               -> Rejected
selected/generated/Hako/interface/static    -> NoSafeSlice
body syntax unavailable with intact identity  -> Unresolved
body observed outside direct cohort           -> Declined
reusing or cloning one-shot envelope          -> Rejected/compile failure
```

No empty/default verified body row is allowed. `FunctionOwnerIdV1` is not
issued here; any body fact requiring resolved function-owner facts remains
closed until `InstanceMethodBodyOwnerBindingIssuer` has its own D0.

## Required tests and guards

* parser envelope is one-shot and cannot be cloned or reused;
* body source and declaration handoff can only be paired through the combined
  envelope;
* exact direct method site/body root/order are retained;
* foreign/missing/duplicate/cardinality cases fail before semantic output;
* body-source modules do not import `EffectMask`, `FunctionSignature`, MIR,
  Recipe, CallSlot, Builder, runtime, or provider modules;
* source files remain below the 760-line split trigger and below 800 lines;
* no test-only arbitrary `Verified*` constructor is added.

## Same-slice closeout

The implementation commit must update, in the same slice:

```text
docs/reference/language/callable-contracts.md
src/mir/resolved_semantics/README.md
docs/development/current/main/investigations/
  own-home-callable-body-source-d0-design-task-2026-08-09.md
  callable-contract-and-instance-call-implementation-task-map-2026-08-08.md
CURRENT_STATE.toml / 10-Now.md
```

The reference receipt must state that this is body-source identity only;
`VerifiedCallableBodyFactsCatalogV1`, conformance, target, Recipe, and
physical lowering remain zero.

## Explicit nonclaims

```text
body is Query-conformant
body is Pure
receiver read footprint
Home Flow
FunctionOwner binding
body facts
declared/conformant catalog
resolver target
source-bound call relation
Recipe CallSlot
Builder/MIR/CFG/PHI
physical ABI
runtime/provider dispatch
fallback/retry
module publication
```

