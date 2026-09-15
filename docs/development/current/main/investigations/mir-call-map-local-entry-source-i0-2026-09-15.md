Task: MIR-CALL-MAP-LOCAL-ENTRY-SOURCE-I0
Parent: mir-call-map-array-entry-source-i0-2026-09-15.md
NextCard: per-owner lifecycle undertaking (C5)
Implementation permission: pending six-line brief + worker audit
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

```text
Decision: pending worker audit.
Source authority + canonical issuer: `observe_map`'s entry loop /
`PrefixLocalFlow` in resolved_semantics.
Non-authority: no physical construction/emission claim.
Fail-fast boundary: pending — live Home/Map-backed locals stay on the
transfer/unavailable paths.
Smallest next slice: pending — one non-scalar-local entry class +
focused positive/negative tests.
Non-claims: physical emission, element-level non-scalar locals, Map
return ABI, C5 per-owner gate.
```

## Acceptance

Pending: merged route's `EntryValue` failures reach zero uncovered
classes (or only explicitly-excluded ones); the residual blocker is
then the C5 per-owner lifecycle arm.
