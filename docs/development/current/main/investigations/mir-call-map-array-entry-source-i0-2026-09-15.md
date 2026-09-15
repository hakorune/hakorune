Task: MIR-CALL-MAP-ARRAY-ENTRY-SOURCE-I0
Parent: mir-call-map-nested-entry-slot-i0-2026-09-15.md
NextCard: mir-call-map-local-entry-source-i0-2026-09-15.md
(non-scalar-local entry ownership); then per-owner lifecycle
undertaking (C5)
Implementation permission: landed
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

Worker audit (2026-09-15) resolved the open questions: sealed evidence
already exists — `BodyExpressionShapeV1::ArrayLiteral{site,element_count}`
plus per-element `Element(ordinal)` relation rows, exactly parallel to
`MapLiteral`/`EntryValue`; `ResolvedLiteralSourceV1` has no array variant
(body-shape channel only); `local x = [...]` hits `PrefixNotCovered` but
is out of scope — the merged `=> [...]` shape goes through `observe_map`
only. The audit suggested a destination-bearing child row; the Decision
is one step smaller: leaf elements carry no transfer/consume state, so
element evidence lives on the parent entry.

```text
Decision: admit `[...]` EntryValue children as
`MapEntryOwnership::NestedArray { elements }` — each `Element(ordinal)`
child classified through the same leaf chain (Integer/Bool/String/
TrivialLocal/BorrowedHandle). No child flow row: leaf elements issue no
transfer and consume no Home.
Source authority + canonical issuer: `observe_map`'s entry loop in
`home_map_flow.rs`, consuming sealed `ArrayLiteral` + `Element(ordinal)`
rows; `array_literal_element_count` added beside `map_literal_keys` in
`home_terminal_relation.rs`.
Non-authority: no physical array construction/emission, no element Home
transfer, no `local x = [...]`/`new ArrayBox()` prefix coverage.
Fail-fast boundary: element Home transfers, nested `%{...}`/`[...]`
elements, non-scalar-local elements and every other class stay
`MapCandidateNotCovered`; `NestedArray` returns None from
transfer_home/binding/value_source so every physical gate stays closed
with zero downstream arms.
Smallest next slice: the array arm + leaf-classifier extraction +
focused positive/negative tests.
Non-claims: physical array construction, element transfer/container
recursion, Map return ABI, C5 per-owner gate.
```

## Acceptance

Pending: array-literal entry values produce exact source evidence on
the parent row (Complete or a dedicated child row per the Decision);
merged route's `EntryValue` failures drop to non-scalar-local classes
only; install/emission stays fail-closed.

## Implementation receipt (2026-09-15, landed)

Code:

- `home_terminal_relation.rs`: `array_literal_element_count` — sealed
  `ArrayLiteral` row lookup beside `map_literal_keys`.
- `home_map_flow.rs`: `MapEntryOwnership::NestedArray { elements }` +
  `ArrayElementSource { site, value }`; `array_elements()` accessor;
  `transfer_home`/`binding`/`value_source` all `None`. The entry loop's
  array arm walks each sealed `Element(ordinal)` relation through the
  extracted `map_value_leaf` classifier (Integer/Bool/String/
  TrivialLocal/BorrowedHandle only — Home/Map locals return `Handle`
  but fail `is_self_rooted_handle`, so they stay uncovered). No child
  flow row; elements issue no transfer and consume no Home.

Focused tests (`map_home_flow_tests.rs`, +3; quick profile, 80/80):

- `array_entry_records_exact_leaf_elements` — Integer/String/
  BorrowedHandle elements with exact `Element(0)` site paths
- `array_entry_empty_literal_and_local_position_complete`
- `array_entry_rejects_home_and_container_elements` — live Home,
  map-installed local, `[[]]`, `[%{}]` all stay Unavailable
- Updated pins: `[]` moved out of
  `return_boundary_map_rejects_uncovered_entry_value_classes`
  (replaced by `[[]]`); `nested_map_child_failure_...` child class
  `[]` -> `[[]]`.

Route evidence (merged `/tmp/merged_entry.hako`, quick binary,
temporary eprintln instrumentation — removed):

- Outer label unchanged: `MapLifecycleConsumerMissing`.
- Array-literal failures eliminated: residual uncovered entries dropped
  from 4 sites to 2 — `Body(1),Initializer(0) EntryValue(0)` and
  `Body(2),Value EntryValue(2)`, both the non-scalar-local class
  (`locals.observe` returns no leaf classification for them). Zero
  `[tmp/map-elem]` failures observed — every array literal on the
  observed path now completes.
- Call-argument-position maps (`to_json(%{"functions"=>[main]})`) are
  outside the walked positions; their MapLiteral rows still demand
  flow rows at install loop1 — unchanged fail-closed behavior.

Red classification: two pin updates were current-change expectation
updates (`[]` and nested-child `[]` are admitted now); no known
baseline or new failures.

Next bounded slices: non-scalar-local entry ownership (the last
uncovered entry class on the merged route), then the C5 per-owner arm.
