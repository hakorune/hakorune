---
Status: active bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-owner-binding-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-OWNER-CARRIER-I0

## Goal

Issue one AST-free resolver-owned instance-method function carrier through the
accepted parser transaction-scoped syntax lease. This row proves the missing
source-bound function issuer only. It does not bind Query bodies to functions
yet.

```text
ParserResolverBodyTransactionV1
  -> one callback-scoped direct-method syntax lease
  -> FunctionSemanticResolverSessionV1::resolve_forest
  -> VerifiedResolvedInstanceMethodFunctionCarrierV1
```

## Allowed scope

One ordinary direct Rust Box method and one real resolver session. The carrier
may own the existing `VerifiedSemanticOwnerForestV1` and expose its root
function/owner by borrow; it must not duplicate `FunctionOwnerIdV1` or create
a new owner issuer.

The carrier must retain or prove:

```text
resolver-normalized declaration/source site
parser provenance
resolver catalog brand
nominal Box identity
FunctionOriginV1 as consistency receipt
root owner-bearing forest/function product
root profile/body pair
exact ordered body-item coverage
```

The parser syntax lease is private, non-`Clone`, and cannot escape the callback
lifetime. No AST or syntax pointer may be stored in the carrier.

## Required tests

Positive:

```text
one direct instance method
same source site and parser provenance
forest root is the carrier owner
root profile is DeclaredFunction(DeclaredInstance)
body coverage equals the borrowed syntax body length
empty body coverage remains representable
```

Negative:

```text
static/generated/selected-gate source
foreign parser provenance
foreign resolver brand or nominal Box
wrong source site
wrong root profile/receiver policy
body cardinality mismatch
duplicate source row
one carrier row reused for two declarations
```

## Forbidden

```text
normal_source_plan legacy name/AST lookup
method name, arity, inventory ordinal, or FunctionOrigin as pairing key
caller-built function map
second FunctionOwner issuer
owner-link co-seal
body facts / conformance
target / Recipe / CallSlot / Builder / MIR / runtime
fallback or retry
test-only arbitrary Verified constructor
```

## Closeout

The same implementation slice updates:

```text
src/parser/body_source.rs (or a dedicated lease module)
src/mir/resolved_semantics/instance_method_function_carrier.rs
focused tests and module READMEs
docs/reference/language/callable-contracts.md
CURRENT_STATE.toml / 10-Now.md
```

Keep each touched source file below the 760-line split trigger and below 800
lines. Run the focused tests, `cargo check --lib`,
`bash tools/checks/current_state_pointer_guard.sh`, and `git diff --check`.
