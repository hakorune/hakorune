Task: MIR-CALL-MAP-ARRAY-ENTRY-SOURCE-I0
Parent: mir-call-map-nested-entry-slot-i0-2026-09-15.md
NextCard: non-scalar-local entry ownership; then per-owner lifecycle
undertaking (C5)
Implementation permission: pending six-line brief + worker audit
---

# Map array-literal entry source I0

## Entry contract

The nested-entry-slot card landed: a nested `%{...}` EntryValue child
issues its own flow row with `MapDestinationV1::EntrySlot`, and merged
route evidence shows nested-map failures eliminated — the residual
uncovered entries are array literals (`"locals" => []`,
`"args" => [...]`) and non-scalar locals.

`[...]` array literals currently hit `MapCandidateNotCovered`: the child
site is neither a variable (`variable_ref` is `None`), a literal, nor a
`MapLiteral` row. Merged shapes needing coverage:

```text
"locals" => []              empty array literal
"args"   => [a, b, ...]     array literal with element expressions
```

`Census boundary: merged entry program -> `[...]` EntryValue children;
includes empty and element-bearing literals inside `%{...}` entries;
excludes `local x = [...]`/`new ArrayBox()` prefix coverage (upstream
blocker, separate card) and non-scalar-local entry values (sibling
card).`

## Open design questions (worker audit — pending)

1. Does the body-shape issuer emit an `ArrayLiteral` row (or element
   relations) for `[...]` sites, the way `MapLiteral`/`EntryValue` rows
   exist for `%{...}`? What exact sealed evidence identifies the child
   site and its element children?
2. Is an array entry a `MapEntryOwnership`/`MapValueSource` variant
   (value leaf with element-site evidence) or a destination-bearing
   construction like `EntrySlot` (its own flow row + recursion)?
3. Element classes: which element source classes does the same
   value-source classification admit (Integer/Bool/String/TrivialLocal/
   BorrowedHandle/nested maps)? Do element `EntryValue`-style relations
   exist, and does a nested map inside `[...]` recurse via EntrySlot?
4. Downstream fail-closed: array entries must not claim scalar_kind or
   a transfer home — which new variant keeps every physical gate closed
   with zero extra code?
5. Upstream coupling: `local x = [...]`/`new ArrayBox()` initializers
   fail at `PrefixNotCovered` before entry checks — does this card's
   entry-value coverage depend on that prefix card landing first?

## Six-line brief

```text
Decision: pending worker audit.
Source authority + canonical issuer: pending — likely `observe_map` /
`home_map_flow` plus whatever body-shape evidence the `[...]` site
carries.
Non-authority: no physical array construction, no emission claim.
Fail-fast boundary: pending — non-scalar locals and Home/Map-backed
handles stay Unavailable.
Smallest next slice: pending — the array entry class + focused
positive/negative tests.
Non-claims: physical array construction, `local x = [...]` prefix
coverage, Map return ABI, C5 per-owner gate.
```

## Acceptance

Pending: array-literal entry values produce exact source evidence on
the parent row (Complete or a dedicated child row per the Decision);
merged route's `EntryValue` failures drop to non-scalar-local classes
only; install/emission stays fail-closed.
