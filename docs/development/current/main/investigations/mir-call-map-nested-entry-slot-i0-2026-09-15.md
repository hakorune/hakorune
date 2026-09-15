Task: MIR-CALL-MAP-NESTED-ENTRY-SLOT-I0
Parent: mir-call-map-entry-value-source-i0-2026-09-15.md
NextCard: mir-call-map-array-entry-source-i0-2026-09-15.md (array
literal entry class); then non-scalar-local ownership; then per-owner
lifecycle undertaking (C5)
Implementation permission: landed
---

# Nested Map entry-slot destination I0

## Entry contract

The entry-value source card landed Facts-level admission for String
literals and self-rooted parameter handles. Merged route evidence:
the first failing entry advanced from `EntryValue(0)` to
`EntryValue(2)`; every remaining uncovered entry is a card-listed
exclusion — nested `%{...}`, array literal, or non-scalar local.

The first failing merged site (`make_const`,
`"value"=>%{ "type"=>"i64", "value"=>val }`) is a nested map. Per the
prior worker audit, a nested `%{...}` carries **two** authorities: its
own `MapLiteral` body-shape row (so preflight loop1 demands a Complete
flow row for the nested site) AND an `EntryValue` relation on the
parent (so the parent's entry loop wants a value source for the same
child site). A nested map is therefore not a value-source leaf: it
needs a `MapDestinationV1::EntrySlot(parent_site, entry_ordinal)` so the
child's construction flows into the parent's entry, plus recursive
observation and outer-fault plumbing.

`Census boundary: merged entry program -> nested `%{` EntryValue
children; includes every parent map site; excludes array literals and
non-scalar-local entry values (sibling cards).`

## Open design questions (worker audit — answered)

1. `MapDestinationV1::EntrySlot` shape: which site pair exactly
   identifies the parent entry (parent map site + entry ordinal, or the
   EntryValue child site itself)?
2. Observation order: the child map must be observed with the parent's
   prefix state — does `observe_map` recurse in-place or does the walk
   order sites so the child resolves first?
3. Completion rule: is a parent entry `Value` complete only when the
   nested child's flow row is Complete? Where does the child's
   Unavailable surface on the parent row?
4. `used`/`compatible`/`remaining`: a nested map's own entries may
   consume TransferHomes — how do parent and child share one used-set
   without double-consumption?
5. Downstream: `scalar_kind()`/emission stays fail-closed — does the
   EntrySlot destination need a fail-closed guard at install anyway
   (nested physical construction is out of scope)?

## Worker audit (f7dc0b99, code-verified)

1. The walk visits top-level statements only; nested `%{...}` sites get
   zero `observe_map` calls. Recursion must happen inside
   `observe_map`'s entry loop — the sole existing access point.
2. "child is a MapLiteral site" is detectable in-loop via
   `map_literal_keys(input, child)` — no new facts needed.
3. `used`/`remaining`/`outer`/`locals` must be shared across the
   nested tree: a home consumed inside a child subtree is marked in
   the child's `outer` (child-relative index) AND the parent's `outer`
   (parent entry index), so `outer_after_installs` stays exact.
   `locals.consume_home` moves to the transfer point (`&mut`).
4. `MapDestinationV1::EntrySlot { parent_map, ordinal }` identifies
   the parent entry exactly; `local_binding()` -> `None`.
5. `MapEntryOwnership::NestedMap` makes `transfer_home`/
   `value_source`/`binding` all `None` — every downstream
   `scalar_kind`/transfer gate fails closed with zero extra code.
6. Consumer audit: only `local_binding()` + the outward dispatch need
   code; preflight loop1, `map_install_owners`, `begin_map_emission`,
   `validate_map_emission`, `selected/map.rs::emit`,
   `map_demands_consumed` all stay fail-closed unchanged.
7. `SourcePathSegmentV1::EntryValue(ordinal)` relations map the child
   node back to its `MapLiteral` row directly.

## Six-line brief

```text
Decision: a nested `%{...}` EntryValue child issues its own flow row
with `MapDestinationV1::EntrySlot { parent_map, ordinal }`; the parent
entry records `MapEntryOwnership::NestedMap` (no value source, no
binding). Recursion lives inside `observe_map`; `locals` becomes `&mut`
with `consume_home` at the transfer point, `used`/`remaining`/
`nested_out` thread through, and child-subtree transfers mark the
parent's `outer` at the parent entry index.
Source authority + canonical issuer: `observe_map`/`home_map_flow` own
entry classification and nested row issuance; `map_control.rs` gains
`map_entry_outward` for exact child membership/path verification.
Non-authority: no physical nested construction, no emission claim, no
runtime layout reads.
Fail-fast boundary: arrays, non-scalar locals and Home/Map-backed
handles stay Unavailable; EntrySlot rows have `local_binding() ==
None`, `value_source() == None`, so install/emission stays fail-closed.
Smallest next slice: EntrySlot destination + recursion + focused
positive/negative tests.
Non-claims: recursive physical Map construction, Map return ABI, C5
per-owner gate, array literals, prefix coverage for `new ArrayBox()`.
```

## Acceptance

Pending: a nested `%{...}` entry produces a Complete child flow row
with exact EntrySlot destination evidence and a Complete parent entry;
malformed/foreign nested sites stay Unavailable; merged route's
`EntryValue` failures drop to array/non-scalar-local classes only.

## Implementation receipt (2026-09-15, landed)

Code:

- `map_control.rs`: `map_entry_outward` — owner/membership (child AND
  parent are `MapLiteral` rows), exact path tail `parent.node +
  EntryValue(ordinal)`, exact sealed relation, scope/target tail shared
  with `map_source_outward`. Exported via `resolved_control_flow`.
- `home_map_flow.rs`: `MapDestinationV1::EntrySlot { parent_map,
  ordinal }` (`local_binding()` -> `None`) and
  `MapEntryOwnership::NestedMap` (`transfer_home`/`value_source`/
  `binding` -> `None`). `observe_map` recurses on
  `map_literal_keys(input, child)`; `locals` is `&mut` with
  `consume_home` at the transfer point; `used`/`nested_out` are shared
  across the nested tree; child-subtree transfers mark the parent's
  `outer` at the parent's entry index; child failure pushes the child's
  Unavailable row and propagates its issue.
- `home_new_prefix.rs`: both `observe_map` call sites create `used`/
  `nested` and extend `maps` with nested rows (parent first, then
  children); caller-side `consume_home` loops deleted (consumption now
  at the transfer point).

Focused tests (`map_home_flow_tests.rs`, +4; quick profile, 77/77):

- `nested_map_entry_issues_entry_slot_child_row`
- `nested_map_entry_in_local_position_and_deeper_recursion`
- `nested_map_child_failure_marks_both_rows_unavailable`
- `nested_map_transfer_marks_parent_outer_at_parent_entry_index`
- Updated pins: nested case removed from
  `return_boundary_map_rejects_uncovered_entry_value_classes` and
  `map_candidate_alias_reuse_..._are_not_transfer_evidence` (renamed
  `..._and_fresh_...`) — nested `%{...}` is admitted, still never
  transfer evidence.

Route evidence (merged `/tmp/merged_entry.hako`, quick binary,
temporary eprintln instrumentation — removed):

- Outer label unchanged: `MapLifecycleConsumerMissing`.
- Nested-map failures eliminated: both prior `Body(0),Value
  EntryValue(2)` nested-map sites (`make_const` shape) now complete.
- Residual uncovered: 4 sites, all array-literal or non-scalar-local
  classes — `Body(0),Value EntryValue(2)` (the `"locals" => []` shape),
  `Body(2),Value EntryValue(2)`, `Initializer(0) EntryValue(0)/(1)`.
- First install failure moved to an initializer-position map
  (`Body(0), Initializer(0)`); return-position maps now pass loop1.

Red classification: the single pre-update failure
(`map_candidate_alias_reuse_fresh_and_nested_...`) was a current-change
expectation update — nested is admitted but remains non-transfer
evidence, so the case moved to the positive nested tests.

Next bounded slices: array-literal entry source class, then
non-scalar-local ownership, then the C5 per-owner arm.
