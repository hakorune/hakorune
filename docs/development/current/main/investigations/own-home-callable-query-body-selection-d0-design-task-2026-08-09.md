---
Status: accepted design stop — implementation not open
Date: 2026-08-09
Parent: `docs/development/current/main/investigations/own-home-callable-body-source-d0-design-task-2026-08-09.md`
Authority: `docs/reference/language/callable-contracts.md`
---

# CALLABLE-QUERY-BODY-SELECTION-D0/I0

## Decision

General body-source authority and declared Query selection are separate
products. The body-source issuer validates the complete supported direct
instance-method cohort without inspecting Query behavior. This row projects
that already validated catalog through the already sealed selected Query view.

```text
VerifiedInstanceMethodBodySourceCatalogV1
  + borrowed selected Query contract view
  -> VerifiedDeclaredQueryBodySourceCatalogV1
```

The projection does not issue Query, Home, signature, effect, or FunctionOwner
meaning. It only proves that each selected Query declaration has exactly one
validated body-source row from the same parser transaction and resolver
catalog.

## Input authority

The projection borrows a read-only selected view from
`VerifiedDeclaredInstanceMethodContractCatalogV1`. It must not rebuild the
selection from:

```text
raw CallableContract rune syntax
method name
Box name
arity
inventory ordinal
declaration/query vector positions
```

If the aggregate does not expose a selected-contract view, add that view at
the aggregate owner. Do not make the projection repeat the declaration/query
join or copy Home/Query receipts.

## Exact behavior

For a source containing:

```text
method A = Query
method B = no declared contract
method C = Query
```

and a complete body-source catalog for `A`, `B`, and `C`, the projection
contains `A` and `C` in their original branded source order. It does not
rebase source ordinals, create a default row for `B`, or discard the source
identity carried by `A`/`C`.

Every body row is validated against the general catalog before projection.
An unknown, foreign, duplicate, or contradictory extra row is rejected even
when its apparent declaration is non-Query. A valid non-Query row is simply
unselected.

Empty body coverage (`[]`) is valid at this source layer; missing return or
effect behavior belongs to body facts/conformance.

## Disposition and failure boundary

```text
unsupported parser/source cohort or missing issuer -> NoSafeSlice
foreign/duplicate/mismatched body identity       -> Rejected
incomplete evidence with intact identity          -> Unresolved
fully observed source outside Query meaning       -> Declined (observer only)
valid non-Query row                                -> unselected, not Declined
```

The projection never emits an empty/default verified row to repair missing
coverage. It never sorts by name or uses an inventory placement ordinal as
identity.

## Required acceptance tests

Positive:

```text
Query / non-Query / Query sparse selection
original source order and SourceBoxMethodSiteV1 retained
exactly one selected row per Query declaration
empty body coverage retained without behavior inference
```

Negative:

```text
missing selected Query row                 -> Rejected
duplicate selected Query row               -> Rejected
foreign extra row                          -> Rejected
same name but foreign source site          -> Rejected
selection rebuilt from raw rune            -> guard failure
name-sorted/rebased sparse projection      -> Rejected
non-Query row with no contract             -> unselected, no default row
```

## Nonclaims

```text
FunctionOwner binding
body facts
Query/Pure conformance
Home or ABI reissuance
resolver target
source-bound call relation
Recipe/CallSlot
Builder/MIR/CFG/PHI
runtime/provider dispatch
fallback/retry
```

## Implementation task

`CALLABLE-QUERY-BODY-SELECTION-I0` may open only after
`CALLABLE-BODY-SOURCE-AUTHORITY-I0` has landed with a general
`VerifiedInstanceMethodBodySourceCatalogV1`. Its implementation commit must
update this card, `docs/reference/language/callable-contracts.md`,
`src/mir/resolved_semantics/README.md`, the task map, and current mirrors in
the same slice. The next row is
`CALLABLE-BODY-OWNER-BINDING-D0/I0`.
