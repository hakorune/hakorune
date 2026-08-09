---
Status: closed bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-facts-query-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-FACTS-QUERY-I0

## Goal

Implement only the accepted D0 projection:

```text
VerifiedInstanceMethodBodyOwnerCatalogV1
  -> private QueryBodyFactsObserver
  -> VerifiedCallableQueryBodyFactsCatalogV1
```

The observer reads the owner row's carrier and its already-sealed
`VerifiedResolvedBodyShapeInventoryV1`. It does not receive or reconstruct a
parser envelope, declaration/Home/Query catalog, AST, owner number map, or
second resolved-function array.

## Bounded positive shape

For every selected Query owner row, issue one private facts row only when the
carrier shape proves exactly:

```text
one Return statement with a value
one BodyExpressionShapeV1::Me
Me.receiver is a BindingRef owned by the exact root function
the binding record kind is BindingKindV1::Receiver
one BodyShapeRelationV1::ReturnValue links Return to Me
no other statement, expression, relation, or effect
```

The positive fixture is `@rune CallableContract(query) length(): i64 {
return me }`. The facts issuer deliberately does not validate the declared
result type; that remains a later type/conformance owner.

## Product boundary

Add a dedicated module and focused tests:

```text
src/mir/resolved_semantics/query_body_facts.rs
src/mir/resolved_semantics/query_body_facts_tests.rs
```

The public-in-module boundary is one catalog-level, non-`Clone` aggregate.
Per-row DTOs and constructors remain private to the issuer. A row borrows the
owner-link authority and exposes only source-derived facts:

```text
VerifiedCallableQueryBodyFactsCatalogV1
  -> VerifiedCallableQueryBodyFactsRowV1
       owner/declaration identity by borrow
       ReceiverReadFact { expression site, receiver BindingRef }
       OrdinaryReturnFact { statement site, value site }
```

The catalog must seal exactly one row per selected Query owner row, preserving
owner-catalog order and rejecting missing, duplicate, foreign, or extra rows.
No name, method ordinal, vector position, `FunctionOriginV1`, or numeric
`FunctionOwnerIdV1` comparison may pair rows.

## Validation rules

The issuer must check, using resolver products already carried by the owner:

```text
owner row root == carrier root
shape.owner == carrier root.owner
shape.body_root == carrier body_root
Me.receiver.owner == root.owner
root.binding(Me.receiver).kind == BindingKindV1::Receiver
Return.value == Me expression site
ReturnValue relation exists exactly once
```

The owner-link has already checked parser provenance, resolver brand, source
identity, and body coverage. The facts issuer may recheck local shape/root
consistency, but must not create a second identity authority.

## Dispositions

```text
exact bounded shape             -> Candidate
fully observed out-of-cohort   -> Declined
opaque/incomplete required row -> Unresolved
foreign/duplicate/missing/mismatch -> Rejected
missing shape issuer            -> NoSafeSlice (development state only)
```

`return 0`, empty body, local read, multiple return, field access, method call,
extra statement/effect, and unsupported control are negative fixtures. A
valid non-Query row remains unselected upstream and receives no default facts.

## Landed focused tests

```text
query_body_facts_accept_exact_return_me
query_body_facts_decline_constant_and_empty_shapes
query_body_facts_decline_field_and_extra_statement_shapes
query_body_facts_preserve_sparse_selected_query_order
```

The existing owner-link and body-shape focused suites retain the preceding
foreign-brand, source-root, coverage, and neutral-shape rejection evidence.
The facts issuer itself consumes only valid owner-link products, so invalid
owner/root/duplicate relation fixtures are not forged inside this slice. Use
real parser/resolver/carrier/owner products; do not add arbitrary `Verified*`
constructors, default facts, AST rescans, name repair, or test-only forged
owners.

## Explicit non-claims

```text
no complete effect-absence receipt
no Query/Home/signature/ABI re-issuance
no body-to-contract/type inference
no field/state authority
no conformance catalog
no resolver target/source-bound call/Recipe/CallSlot
no Builder/MIR/CFG/PHI/physical ABI
no fallback/retry/provider/runtime/production
```

## Closeout

The implementation and same-slice documentation are landed. The private
facts issuer is `src/mir/resolved_semantics/query_body_facts.rs` (265 lines)
with focused tests in `query_body_facts_tests.rs` (123 lines), both below the
800-line ceiling. The exact bounded receipt is:

```text
query_body_facts:                         4 passed
body_shape_tests:                         3 passed
instance_method_body_owner_tests:         3 passed
resolver_tests:                           4 passed
current_state_pointer_guard.sh:           passed
```

The implementation consumes only the landed owner-link/carrier products,
preserves sparse selected-Query order, and emits no public contract,
conformance, target, Recipe, or MIR meaning. The next design stop is
`CALLABLE-CONTRACT-CONFORMANCE-D0`; do not continue directly to target or
Recipe work.
