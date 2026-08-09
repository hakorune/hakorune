---
Status: closed bounded implementation 2026-08-09
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

The implementation is source-authority only. The general body-source catalog
and its borrowed Query projection remain separate inputs; this carrier does
not select Query rows or perform the owner-link co-seal.

## Bounded I0 tests

Positive:

```text
one direct instance method
same source site and parser provenance
resolver brand is retained from the declaration catalog
forest root is the carrier owner
root profile is DeclaredFunction(DeclaredInstance)
body coverage equals the borrowed syntax body length
empty body coverage remains representable
```

The bounded I0 receipt covers the parser-provenance negative. The remaining
identity/cardinality/profile cases stay as explicit owner-link or parser-seal
guards rather than being forged through test-only carrier constructors:

```text
foreign parser provenance                    -> carrier I0
static/generated/selected-gate source        -> parser/source seal
wrong source site / duplicate source row     -> parser/declaration seal
wrong root profile/receiver policy           -> carrier issuer
one carrier row reused for two declarations  -> owner-link D0/I0
```

Foreign resolver brand/nominal Box, body-source cardinality mismatch, and
one-to-one reuse are owner-link D0/I0 negatives. The carrier trusts only the
already sealed declaration catalog and retains its brand; it does not invent a
second cross-catalog identity check.

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

Receipt:

```text
cargo test --lib instance_method_function_carrier -- --nocapture
  3 passed
```

The next design stop is `CALLABLE-BODY-OWNER-BINDING-D0`. No owner link,
body facts, conformance, target, Recipe/CallSlot, or production route opens
from this carrier receipt.
