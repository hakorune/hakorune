Task: MIR-CALL-MAP-LOCAL-ENTRY-SOURCE-I0
Parent: mir-call-map-array-entry-source-i0-2026-09-15.md
NextCard: per-owner lifecycle undertaking (C5)
Implementation permission: landed
---

# Map non-scalar-local entry source I0

## Entry contract

The array-entry card landed: `[...]` entry values record `NestedArray`
ownership with leaf-classified `Element(ordinal)` children. Merged
route evidence shows the residual uncovered entries are now exactly
one class — non-scalar locals:

```text
Body(1), Initializer(0) EntryValue(0)   "callee" => callee   (param
                                        holding a call result)
Body(2), Value         EntryValue(2)    e.g. "blocks" => blocks
                                        (param/local holding an array
                                        or map value)
```

These locals hold non-scalar values (maps, arrays, call results) and
are neither `TrivialLocal`, self-rooted `Handle`, `Home`, nor `Map` in
`StoredLocal` — `locals.observe` returns `None` for them today, so they
fall to `MapCandidateNotCovered`.

`Census boundary: merged entry program -> non-scalar-local EntryValue
children; includes parameter bindings and prefix locals whose value is
a non-scalar; excludes array-element non-scalar locals (element class
boundary is the array card's fail-fast edge — promoting elements is a
separate slice) and live Home/Map locals (transfer path).`

## Open design questions (worker audit — pending)

1. What `StoredLocal` state do these non-scalar locals actually carry?
   `observe` returns `None` for `Consumed`/`Uninitialized`/`Handle` with
   a consumed root — which case are `callee`/`blocks`/`arg_ids` in
   merged? Are they `Uninitialized` (never tracked) or a missing
   `StoredLocal` class (e.g. call-result, map-installed, opaque)?
2. Is a non-scalar local entry a borrow (`MapValueSource`-level leaf
   carrying the binding + an opaque-kind marker) or an ownership
   question (transfer/loan of the held object)? What does the
   merged-route consumer (`to_json`) actually need — a deep structure
   or just the value reference?
3. If the local holds a `%{...}`/`[...]` constructed earlier, does the
   entry need the construction's flow row as evidence (destination
   relations), or is a binding-level opaque borrow enough for Facts?
4. Downstream fail-closed: which variant shape keeps every physical
   gate closed — `MapValueSource` opaque-local leaf (needs
   `scalar_kind()==None` + the `selected/map.rs` arm) or an ownership
   variant returning `None` everywhere?
5. Does admission interact with `install_map`'s `StoredLocal::Map`
   marking or `consume_home` — does recording a non-scalar local as an
   entry value change what happens to that local afterwards?

## Six-line brief

Worker audit (2026-09-15) located the residual class precisely: the
merged route's remaining `MapCandidateNotCovered` sites are
`StoredLocal::Map` locals (`local callee = %{...}` then
`"callee" => callee`) plus the chained `payload` case — a binding whose
own map-install failed and was therefore never inserted. A `Map` local
is a live *owner* (its `Map::End` still issues via `outer`), so an
entry reference is a non-consuming borrow — `MapValueSource` leaf, not
a transfer, not a `NestedMap` construction.

```text
Decision: admit live map-installed locals as `MapValueSource::MapLocal`
— `locals.observe` yields `Handle(root)` and `is_map_local(root)`
(`StoredLocal::Map` at root) proves the borrow; no consume, no
used/compatible interaction. Aliases (`Handle(root)` of a Map root)
resolve to the same root.
Source authority + canonical issuer: `map_value_leaf` in
`home_map_flow.rs` + `PrefixLocalFlow::is_map_local` in
`home_prefix_local_flow.rs`.
Non-authority: no physical emission (borrowed-object install is
unbuilt), no ownership transfer, no ABI claim.
Fail-fast boundary: `Uninitialized`/`Consumed`/absent bindings and
Home/Handle-of-Home locals stay uncovered (transfer questions, never
borrows); `scalar_kind()==None` keeps begin_map_emission,
map_install_owners and selected/map.rs fail-closed.
Smallest next slice: the variant + `is_map_local` + the `selected/map.rs`
exhaustive arm + focused positive/negative tests.
Non-claims: physical emission, call-arg-position map observation, Map
return ABI, C5 per-owner gate.
```

## Acceptance

Met: merged route's `MapCandidateNotCovered` entry failures reached
zero — residual blocker is the C5 per-owner arm plus un-walked
map-literal positions (call-arg `%{...}` rows demand flow rows at
install loop1 but are never observed).

## Implementation receipt (2026-09-15, landed)

Code:

- `home_prefix_local_flow.rs`: `is_map_local(root)` —
  `locals[root] == StoredLocal::Map`.
- `home_map_flow.rs`: `MapValueSource::MapLocal(root)`;
  `scalar_kind()` -> `None`; `binding()` -> `Some(root)` (consistent
  with `BorrowedHandle`); `map_value_leaf` arm after
  `is_self_rooted_handle`. Shared by entry values and array elements —
  one leaf classification, both positions.
- `selected/map.rs`: `MapLocal` joins the
  `map-value-consumer-missing` exhaustive arm (unreachable —
  `scalar_kind()==None` returns first; the arm exists for exhaustiveness).

Focused tests (`map_home_flow_tests.rs`, +2; quick profile, 82/82):

- `map_local_entry_is_a_non_consuming_borrow` — `MapLocal` source,
  `binding()` exposes the root, `transfer_home()==None`, `m` stays in
  the parent's outer after all installs.
- `map_local_alias_and_array_element_borrow_the_same_root` — alias and
  `[m, a]` elements all borrow the same map root.
- Pin updates: `local m = %{} local a = m` moved out of
  `return_boundary_map_rejects_uncovered_entry_value_classes`
  (replaced by `local u` uninitialized); `[m]` moved out of
  `array_entry_rejects_home_and_container_elements` (map-local
  elements are borrows, still fail-closed physically).

Route evidence (merged `/tmp/merged_entry.hako`, quick binary,
temporary eprintln instrumentation — removed):

- Outer label unchanged: `MapLifecycleConsumerMissing`.
- **Zero `[tmp/map-entry]` failures** — every walked map literal's
  entry coverage now completes, including the `callee`/`payload` chain
  (`callee` admitted as `MapLocal` -> `payload` installs as `Map` ->
  the return map's `"mir_call" => payload` resolves as `MapLocal`).
- Residual `MapLifecycleConsumerMissing` sources: un-walked map
  positions (call-arg `%{...}` literals produce `MapLiteral` rows with
  no flow row -> loop1 missing-row) and the owner gate — the C5 arm.

Red classification: two pin moves were current-change expectation
updates; the single transient failure
(`map_local_alias_and_array_element_...` row-count assertion) was a
current-change test bug fixed in-flight. No baseline debt touched.

Next bounded slice: the C5 per-owner lifecycle arm — the remaining
merged-route blocker (AppMain identity / `map_install_owners` for the
non-AppMain merged library compile, plus call-arg map positions).
