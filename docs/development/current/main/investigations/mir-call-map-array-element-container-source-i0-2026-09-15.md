Task: MIR-CALL-MAP-ARRAY-ELEMENT-CONTAINER-SOURCE-I0
Parent: mir-call-map-contained-descendant-flow-i0-2026-09-15.md
NextCard: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0 (C5 per-owner contract)
Implementation permission: pending six-line brief + worker audit
---

# Map array-element container source I0

## Entry contract

The contained-descendant card landed `ContainedIn` + the interleaved
sweep; every sealed MapLiteral under a walked statement now gets one
flow row. Merged-route loop1 evidence (2026-09-15) shows the row is
issued but Unavailable at:

```text
[tmp/sweep] site=[Body(0), Initializer(0), Element(3), Argument(1),
Element(0)] dest=ContainedIn{parent=[...Argument(1)], role=Element(0)}
issue=MapCandidateNotCovered([...Element(0), EntryValue(2), Element(0)])
```

`emit_if_merge_local_return_var` (merged L14775+):

```hako
local blocks = [
  ...,
  me._block(3, [
    %{"op"=> "phi", "dst"=> 6, "incoming"=> [[4, 1], [5, 2]]},
    ...
  ])
]
```

`"incoming" => [[4, 1], [5, 2]]` — an array entry whose elements are
themselves array literals. The array-entry card (I0) deliberately
restricted elements to leaf sources; `[4, 1]` is a non-leaf container
element → `MapCandidateNotCovered` → the whole ContainedIn map stays
Unavailable.

`Census boundary: merged entry program -> array-literal element sites
under map EntryValue positions whose element class is not a leaf
(Integer/Bool/String/TrivialLocal/BorrowedHandle/MapLocal); includes
nested `[...]` and `%{...}` elements; excludes elements already
covered as leaves.`

## Open design questions (worker audit — pending)

1. Nested-array element: does `[4, 1]` inside `"incoming"` become a
   recursive `NestedArray` entry (element list per element), or a
   first-class destination row of its own? Arrays are not MapLiteral
   rows — they have no separate flow row, so recursion inside
   `MapEntryOwnership::NestedArray` is the likely shape.
2. Map-as-array-element: `%{}` inside `[...]` — the map is a sealed
   MapLiteral and already gets a ContainedIn row from the sweep; does
   the parent entry need a `NestedMap`-style element kind linking it,
   or does the leaf contract stay map-blind and rely on the row?
3. Transfer elements: `[p]` where `p` is a live Home — is a Home
   element a transfer (consumed) or rejected? The existing pin
   `array_entry_rejects_home_and_container_elements` expects `[p]` to
   keep the outer map Unavailable — decide whether that pin survives
   or is rescoped.
4. Depth bound: is recursion unbounded or capped at one level? The
   merged residual needs exactly one extra level.

## Six-line brief

```text
Decision: pending worker audit — likely recursive NestedArray element
classification inside map entry arrays.
Source authority + canonical issuer: `observe_map` entry loop +
`map_value_leaf`-family element classifier.
Non-authority: no physical array ABI, no emission claim.
Fail-fast boundary: unclassifiable elements stay MapCandidateNotCovered.
Smallest next slice: pending audit — one level of nested-container
elements (the merged residual shape).
Non-claims: physicalization, C5 per-owner arm, production switch.
```

## Acceptance

Pending: merged loop1's first failure advances past the
`EntryValue(2) -> Element(0)` nested-array element — or the pin
documents a deliberate bound with the next honest terminal recorded.
