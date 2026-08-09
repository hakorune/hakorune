---
Status: parked behind D0 acceptance; implementation not open
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-query-body-selection-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-QUERY-BODY-SELECTION-I0

## Purpose

Project the landed general direct instance-method body-source catalog through
the declared aggregate's already sealed Query selection. This is a small
resolver-only projection slice. It does not add a new semantic profile, body
observer, FunctionOwner, target, Recipe, or physical lowering route.

## Preconditions

Do not open this row while the current design stop is unresolved. Before the
implementation commit, the following must be true:

```text
CALLABLE-BODY-SOURCE-AUTHORITY-I0
  -> VerifiedInstanceMethodBodySourceCatalogV1 is landed

declared Query/Home aggregate
  -> owns the private selected-pair relation
  -> exposes one borrowed, read-only selected-contract view

current work mode
  -> implementation row, not design_stop
```

If the aggregate cannot expose the selected view without rejoining parallel
declaration/Home/Query vectors, stop with `NoSafeSlice`. Do not add a second
selection authority or a test-only constructor to unblock this row.

## Input and output

```text
VerifiedInstanceMethodBodySourceCatalogV1
  + aggregate-owned selected Query view
  -> VerifiedDeclaredQueryBodySourceCatalogV1
```

The input body catalog contains every supported direct instance-method body
row. The output contains exactly the selected Query rows, in their original
branded source order. The output is non-`Clone`, AST-free, and resolver-only.

The aggregate view is borrow-scoped and read-only. A suitable shape is:

```text
DeclaredInstanceMethodContractRefV1<'catalog>
  declaration: declaration ref
  home_abi: Home ABI ref
  query: declared Query behavior ref

VerifiedDeclaredInstanceMethodContractCatalogV1::selected_contracts()
```

The projection may inspect the view but must not copy or re-issue its Home or
Query receipts. It must not read private pair indices directly.

## Identity contract

Two independent identity axes are checked:

```text
parser provenance
  body-source transaction and declared contract catalog came from one parser
  invocation/transaction

resolver brand
  body source, declaration, Home, and Query rows belong to one resolver
  catalog allocation
```

The body envelope never accepts a caller-supplied resolver brand. The output
inherits the resolver brand from the issued catalog/view. A bare ordinal,
inventory placement, method name, Box name, arity, vector position, numeric
FunctionOwner, or MIR fact is not an identity.

For the bounded direct cohort, the body row uses the resolver-normalized,
branded method source site plus parser provenance. It does not treat a raw
ordinal alone as source identity. If a future selected-gate/source-path
cohort needs path identity, that path receipt must be added by a separate
parser/source-seal decision; this I0 must not invent it.

## Exact behavior

For source order:

```text
Query A
non-Query B
Query C
```

with complete general body rows for A/B/C, the output is A/C in original
source order. It preserves the source sites and the gap; it does not rebase
ordinals, sort names, or emit a default row for B.

```text
valid non-Query row -> remains in general catalog, unselected here
unknown/foreign/duplicate row -> rejected by the general seal or projection
empty body row [] -> valid source coverage
missing return/effect/conformance -> later body facts/conformance
```

Query selection is borrowed from the aggregate. Raw rune syntax is never
reparsed and declaration/query rows are never joined again in this module.

## Disposition boundary

```text
missing issuer / unsupported parser cohort -> NoSafeSlice (development state)
foreign/duplicate/mismatched identity    -> Rejected
incomplete evidence with intact identity  -> Unresolved
valid non-Query body row                  -> unselected, not Declined
fully observed body outside Query         -> not a projection error
```

Never synthesize an empty/default verified body row to repair a missing Query
body. An empty *body* (`[]`) is different from a missing *body row*.

## Required tests

Positive:

```text
Query/non-Query/Query sparse projection
original source order and source site retained
one selected row per Query declaration
empty Query body row retained as []
valid non-Query row has no projected default
```

Negative:

```text
missing selected Query row              -> Rejected
duplicate selected Query row            -> Rejected
foreign extra row                       -> Rejected
same name, different source site        -> Rejected
parser provenance mismatch              -> Rejected
resolver brand mismatch                 -> Rejected
raw rune/name/ordinal/vector rejoin     -> guard failure
name-sorted or rebased sparse output    -> Rejected
```

The tests must use real issued products from the parser/resolver path. Do not
forge brands, source sites, or verified products with arbitrary test
constructors.

## Implementation files and same-commit closeout

Keep the projection in a dedicated module below the resolver semantic owner,
for example:

```text
src/mir/resolved_semantics/declared_query_body_source.rs
src/mir/resolved_semantics/declared_query_body_source_tests.rs
```

Before the commit is accepted, update in the same slice:

```text
this task receipt
own-home-callable-query-body-selection-d0-design-task-2026-08-09.md
callable-contract-and-instance-call-implementation-task-map-2026-08-08.md
docs/reference/language/callable-contracts.md
src/mir/resolved_semantics/README.md
CURRENT_STATE.toml / 10-Now.md pointers
```

Run the focused body-source/query tests, `cargo check --lib`,
`bash tools/checks/current_state_pointer_guard.sh`, and `git diff --check`.
Keep every touched Rust file below the 760-line review trigger and add no
unconditional debug logging.

## Nonclaims

This row does not claim:

```text
FunctionOwner binding
body Facts
Query/Pure conformance
Home or ABI reissuance
resolver target
source-bound call relation
Recipe/CallSlot
Builder/MIR/CFG/PHI
runtime/provider dispatch
fallback/retry
production activation
```

The next row is `CALLABLE-BODY-OWNER-BINDING-D0/I0`. It must consume this
selected Query body-source product and must not reselect Query rows.
