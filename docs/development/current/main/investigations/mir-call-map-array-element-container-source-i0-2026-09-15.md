Task: MIR-CALL-MAP-ARRAY-ELEMENT-CONTAINER-SOURCE-I0
Parent: mir-call-map-contained-descendant-flow-i0-2026-09-15.md
NextCard: MIR-CALL-MAP-LIFECYCLE-CONSUMER-I0 (C5 per-owner
lifecycle-undertaking contract — the merged route now stops inside
`map_install_owners`, this card's territory)
Status: landed — recursive array-element classification
---

# Map array-element container source I0

## Entry contract

The contained-descendant card landed `ContainedIn` + the interleaved
sweep; every sealed MapLiteral under a walked statement gets one flow
row. The merged residual was `MapCandidateNotCovered` at
`EntryValue(2) -> Element(0)` of
`%{"op"=>"phi", "dst"=>6, "incoming"=>[[4,1],[5,2]]}` — an array entry
whose elements are themselves array literals, which the leaf-only
element contract rejected.

## Decision (worker audit integrated)

1. **Recursive element kind**: `ArrayElementSource` now carries
   `ArrayElementKindV1::{Leaf(MapValueSource), NestedArray(Box<
   [ArrayElementSource]>)}`. Arrays are not MapLiteral rows and need
   no flow row — recursion inside `NestedArray` is the right shape.
   `observe_array_elements` extracts the `Element(ordinal)` relation
   walk and recurses for nested `[...]`.
2. **Map elements deferred**: merged census = zero `[%{}` sites. A
   future admission must observe eagerly inside the classifier (not
   rely on the sweep) so child-subtree transfers mirror into the
   parent's `outer` — recorded as a design constraint, not
   implemented.
3. **Home elements stay rejected**: `[p]` with a live Home is a
   transfer question with no consumer — every physical gate already
   fails closed on `NestedArray` entries; the pin survives.
4. **Depth**: recursion is unbounded (same code path); merged needed
   exactly one level.
5. **Fail-closed for free**: element kinds live under
   `ArrayElementSource`; entry-level `value_source()`/`transfer_home()`
   still return `None` for `NestedArray` — zero new production match
   arms.

## Six-line brief

```text
Decision: recursive element classification inside NestedArray —
ArrayElementKindV1{Leaf, NestedArray}; %{...} elements and live-Home
elements stay MapCandidateNotCovered.
Source authority + canonical issuer: observe_map entry loop +
observe_array_elements in home_map_flow.rs.
Non-authority: no physical array ABI, no element flow rows, no
emission.
Fail-fast boundary: unclassifiable elements stay MapCandidateNotCovered;
NestedArray entries still fail every physical gate via
value_source()/transfer_home()==None.
Smallest next slice: C5 per-owner lifecycle undertaking — the merged
route now stops inside map_install_owners.
Non-claims: physicalization, element transfers, map-element linking,
C5 per-owner arm, production switch.
```

## Acceptance evidence

- Focused: `contained_map_with_nested_array_entry_completes` — the
  exact merged shape `Element -> Argument -> Element` with
  `"incoming" => [[4,1],[5,2]]` completes; `nested_elements()` exposes
  the recursive classification; leaf values verified.
- Pin updates (new contract): `array_entry_rejects_home_and_container_
  elements` dropped the `[[]]` arm (now admitted) and keeps `[p]`/
  `[%{}]`; `return_boundary_map_rejects_uncovered_entry_value_classes`
  uses `[p]`; `nested_map_child_failure_marks_both_rows_unavailable`
  triggers on `[p]`.
- Package scope: 208/208 quick-profile green.
- Merged route advanced past loop1 entirely: **zero**
  `[tmp/preflight-loop1]` failures — every sealed MapLiteral row
  completes. First failure is now `map_install_owners` (`Err(())`) —
  the per-owner install contract that admits only scalar entry
  installs plus AppMain identity. Outer label
  `MapLifecycleConsumerMissing` unchanged.
- Non-claims: no physical array ABI, no map-element linking, no
  element transfers, no C5 arm, no production switch.
