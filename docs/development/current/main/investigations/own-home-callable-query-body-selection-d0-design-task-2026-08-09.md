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

### Required selected-view boundary

The aggregate currently owns the private selected-pair relation. Query body
selection may not read that relation indirectly through parallel slices. The
aggregate owner must expose one borrowed, read-only view whose rows already
bind:

```text
declaration identity
home-ABI identity
declared Query behavior identity
same resolver brand
same parser provenance
```

The view is a borrow-scoped projection, not a new semantic issuer. It must not
clone or re-issue Home/Query receipts, and it must not expose mutable vectors or
raw pair indices. A suitable shape is:

```text
DeclaredInstanceMethodContractRefV1<'catalog>
  declaration: declaration ref
  home_abi: Home ABI ref
  query: declared Query behavior ref
```

with one aggregate-owned iterator such as
`VerifiedDeclaredInstanceMethodContractCatalogV1::selected_contracts()`.
The projection consumes only this iterator and the general body-source
catalog. If this view cannot be provided without reconstructing the join, the
row remains `NoSafeSlice`; no second selection authority is introduced.

### Identity axes

Both identities are required and have different owners:

```text
parser provenance:
  body envelope and declaration/contract catalog came from one parser
  invocation/transaction

resolver brand:
  selected declaration, Home ABI, Query behavior, and body catalog belong to
  one resolver catalog allocation
```

The parser envelope never accepts a caller-supplied resolver brand. The
projection compares the envelope/catalog parser provenance and inherits the
resolver brand from the sealed aggregate/catalog. Numeric owner IDs,
method/Box names, arity, and inventory placement are not identity.

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

The general body-source catalog is the reusable source authority. The Query
catalog is only a deterministic sparse projection over it:

```text
general direct body rows (all supported declarations)
  -> selected Query declaration refs
  -> selected Query body rows in original source order
```

Non-Query rows remain valid source rows but are unselected. An extra body row
that is not present in the general declaration catalog is not ignored merely
because it appears non-Query. The general identity/coverage seal must reject
it before this projection runs.

The projection does not need to inspect body syntax. Empty bodies therefore
remain valid source coverage (`[]`); return/effect validity is a later Facts /
conformance concern. If a future body observer needs syntax, it must consume a
documented AST-free/resolved body view from the owner-binding boundary rather
than re-opening parser AST or reparsing method names.

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
original source order and resolver-normalized branded source site retained
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

Additional identity/transaction tests:

```text
body catalog from parser transaction A + contract catalog from B -> Rejected
same numeric ordinals with foreign parser provenance            -> Rejected
same parser provenance with foreign resolver brand              -> Rejected
same declaration name but different source site                  -> Rejected
empty Query body row                                             -> accepted source coverage
one-shot parser transaction split twice                           -> compile/API failure
AST-only declaration API used to rebuild body pairing             -> guard failure
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

`CALLABLE-QUERY-BODY-SELECTION-I0` (parked implementation card:
`own-home-callable-query-body-selection-i0-implementation-task-2026-08-09.md`)
may open only after
`CALLABLE-BODY-SOURCE-AUTHORITY-I0` has landed with a general
`VerifiedInstanceMethodBodySourceCatalogV1`. Its implementation commit must
update this card, `docs/reference/language/callable-contracts.md`,
`src/mir/resolved_semantics/README.md`, the task map, and current mirrors in
the same slice. The implementation is one bounded projection slice:

```text
1. add the aggregate-owned borrowed selected-contract view
2. add parser-provenance and resolver-brand co-seal checks
3. issue sparse Query body rows without copying semantic receipts
4. add positive/negative tests above
5. update reference/README/current mirrors in the same commit
```

The projection must remain resolver-only and AST-free. It must not open
FunctionOwner, body Facts, conformance, target, Recipe/CallSlot, Builder/MIR,
or production selection. After this I0 lands, the next design row is
`CALLABLE-BODY-OWNER-BINDING-D0/I0`, which binds the selected source rows to
exact resolved-function products and performs no Query re-selection.

## Follow-on task order (fixed)

```text
CALLABLE-QUERY-BODY-SELECTION-D0   current design stop
  -> CALLABLE-QUERY-BODY-SELECTION-I0
  -> CALLABLE-BODY-OWNER-BINDING-D0/I0
  -> CALLABLE-BODY-FACTS-QUERY-D0/I0
  -> CALLABLE-CONTRACT-CONFORMANCE-D0/I0
  -> VERIFIED-CONFORMANT-CALLABLE-CATALOG-D0/I0
  -> RESOLVER-INSTANCE-METHOD-TARGET-D0/I0
  -> SOURCE-BOUND-INSTANCE-CALL-D0/I0
  -> Recipe CallSlot / physical ABI / Lower
```

No later row may be opened early because a body-source row happens to be
green. In particular, a declared Query annotation is not body conformance,
and a body-source row is not a FunctionOwner, target, or Recipe claim.
