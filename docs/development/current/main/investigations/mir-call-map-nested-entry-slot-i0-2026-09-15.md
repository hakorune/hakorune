Task: MIR-CALL-MAP-NESTED-ENTRY-SLOT-I0
Parent: mir-call-map-entry-value-source-i0-2026-09-15.md
NextCard: array-literal entry source class; then non-scalar-local
ownership; then per-owner lifecycle undertaking (C5)
Implementation permission: pending six-line brief + worker audit
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

## Open design questions (worker audit — pending)

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

## Six-line brief

```text
Decision: pending worker audit integration — likely add
`MapDestinationV1::EntrySlot` so a nested `%{...}` child site issues
its own flow row whose destination is the parent's EntryValue child
site; the parent's entry records the nested site as its value evidence
without claiming physical construction.
Source authority + canonical issuer: observe_map / home_map_flow own
entry classification and nested row issuance; the entry loop owns
linking parent entry -> child site.
Non-authority: pending — no physical nested construction, no runtime
layout reads, no emission claim.
Fail-fast boundary: pending — arrays, non-scalar locals and
Home/Map-backed handles stay Unavailable; membership/path checks
unchanged.
Smallest next slice: nested-map destination + focused positive/negative
tests only.
Non-claims: recursive physical Map construction, Map return ABI, C5
per-owner gate, array literals, prefix coverage for `new ArrayBox()`.
```

## Acceptance

Pending: a nested `%{...}` entry produces a Complete child flow row
with exact EntrySlot destination evidence and a Complete parent entry;
malformed/foreign nested sites stay Unavailable; merged route's
`EntryValue` failures drop to array/non-scalar-local classes only.
