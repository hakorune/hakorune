---
Status: closed bounded implementation
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-resolved-shape-issuer-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-BODY-RESOLVED-SHAPE-ISSUER-I0

## Goal

Extend the existing resolver traversal to issue one AST-free, neutral body
shape inventory for the exact function root already carried by the instance
method carrier. The issuer must not add a parser transport, owner issuer, or
Query/body-contract inference path.

```text
FunctionSyntaxViewV1 borrowed by the existing parser-private lease
  -> existing ShadowResolverV0 traversal
  -> ResolvedFunctionBodyShapeProductV1
       - VerifiedResolvedFunctionV1
       - VerifiedResolvedBodyShapeInventoryV1
```

## Bounded cohort

The first positive cohort is one direct instance method with:

```text
body:
  zero or more ordinary expression/neutral rows
  one ordinary `return` statement
  returned value either absent or one resolved expression site

receiver evidence:
  lexical `Me` / receiver binding read only

prohibited in this I0:
  direct receiver field/state semantic claims
  method/provider target resolution
  generic/async/lambda/nested callable ownership
```

The existing empty-body source coverage remains a valid source row, but body
Facts/conformance may reject it later when the declared return contract needs
a value. This issuer records source shape; it does not decide that behavior.

## Required implementation shape

Add a dedicated resolver-side module, split before the 760-line trigger:

```text
src/mir/resolved_semantics/body_shape.rs
src/mir/resolved_semantics/body_shape_tests.rs
```

The private shadow draft may carry a small ledger while traversal owns the
borrowed AST. The canonical conversion must issue one catalog-level,
non-`Clone` product with:

```text
same FunctionOwnerId / body root as the resolved function
same parser provenance and resolver brand as the carrier
ordered body statement/expression rows
return -> exact value-site relation
variable-use -> resolved lexical reference relation
neutral effect/control markers
parent/child source-site relations
complete body-root/nested-callable coverage
```

The public resolver session entry is the only issuer. Existing `resolve(view)`
callers must keep their behavior; the new entry may share the same internal
shadow traversal and canonicalization helpers but must not run a second AST
walk after sealing.

## Acceptance tests

Positive:

```text
`return 0` produces one Return row and exact value site
`return me` produces Return -> Me and receiver BindingRef evidence
empty body produces complete empty coverage (no fabricated Return)
same parser transaction and carrier root are co-sealed
```

Negative:

```text
foreign parser provenance or resolver brand
wrong owner/body root/source kind
missing/duplicate/reordered body rows
return row without value-site relation when a value exists
nested lambda/callable body counted in parent
field/index/method-call receiver presented as lexical Me
write, allocation, await, qmark, throw/panic, non-local control
name/ordinal/vector-position pairing
```

Use real sealed resolver products. Do not add arbitrary `Verified*` test
constructors or empty/default receipts. Failure before the issuer exists is a
development `NoSafeSlice`; malformed issued evidence is `Rejected`.

## Forbidden scope

```text
no parser AST escape or second syntax transport
no Query/Home/signature/ABI issuance
no body Facts or conformance
no field/state declaration authority
no resolver target/source-bound call/Recipe/CallSlot
no Builder/MIR/CFG/PHI/physicalization
no fallback/retry/provider/runtime
```

## Closeout requirements

The implementation commit must update together:

```text
focused resolver tests
src/mir/resolved_semantics/README.md
docs/reference/language/callable-contracts.md
this task receipt
CURRENT_STATE.toml / 10-Now.md pointers
```

Run:

```text
cargo test --lib <body-shape-focused-filter> -- --nocapture
cargo check --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Keep every touched Rust file below 760 lines and never above 800 lines.
After this I0, stop before `CALLABLE-BODY-FACTS-QUERY-I0` and update the
design/task pointers in the same closeout slice.

## Landed receipt (2026-08-09)

The existing shadow traversal now records a private neutral ledger and seals
`VerifiedResolvedBodyShapeInventoryV1` beside the canonical resolved
function. `resolve_forest_with_body_shapes` carries the same inventory through
the existing owner-tree walk, so the instance-method carrier retains the root
shape without a second AST traversal. The carrier row performs the final
parser-provenance/resolver-brand co-seal already owned by its declaration
catalog; the shape inventory itself owns only the exact resolver owner and
body-root identity and remains profile-neutral.

Landed evidence:

```text
src/mir/resolved_semantics/body_shape.rs
src/mir/resolved_semantics/body_shape_tests.rs
FunctionSemanticResolverSessionV1::resolve_with_body_shape
FunctionSemanticResolverSessionV1::resolve_forest_with_body_shapes
VerifiedInstanceMethodFunctionCarrierRowV1::body_shape
```

Focused tests are green:

```text
body_shape_tests: 3 passed
resolver_tests: 4 passed
instance_method_function_carrier_tests: 3 passed
instance_method_body_owner_tests: 3 passed
```

The next design stop is `CALLABLE-BODY-FACTS-QUERY-D0`. Body facts,
conformance, field/state authority, targets, Recipe/CallSlot, MIR, and
production remain closed.
