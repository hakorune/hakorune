---
Status: Provisional SSOT
Decision: provisional
Date: 2026-05-12
Scope: `record` surface semantics, ordinary `box` boundary, and the implementation order for packed/local aggregate lowering.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/box-identity-view-allocation-design-note.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako
---

# Record And Packed Array Lowering SSOT

## Goal

- keep `.hako` source readable for metadata / small-record workloads
- define `record` as the explicit source-level name for the aggregate/value lane
- allow local scalarization and `ArrayBox` packed storage without weakening ordinary `box` identity rules
- give allocator metadata a clean migration path away from hand-written parallel scalar arrays

## Current Reading

Current allocator work already shows the pressure point:

- `M178` needed parallel scalar arrays for aligned-small metadata because an `ArrayBox` of ordinary user boxes was not the right keep lane for that slice
- that workaround is acceptable as an implementation stop-gap
- it is **not** the long-term surface we want users to write

Desired end state:

- user writes a small typed record once
- compiler/MIR/runtime choose scalar tuple or packed-column residence internally
- materialization back to an object happens only at the boundaries that really need object semantics

## Planning Decisions

Decision status note: the source word `record`, the ordinary `box` stop-line,
and the C201-C205 order are fixed for planning. Parser/runtime semantics stay
provisional until the individual C202-C204 rows land.

### 1. `record` is the source-level name for an aggregate value contract

`record` does **not** mean "fast-path-only box" or "ordinary box with a lucky optimizer".
It means the program is explicitly declaring an identity-free aggregate contract with these semantics:

- no mandatory stable identity
- copyable value semantics
- fixed typed fields
- layout known at compile time
- eligible for local scalar replacement
- eligible for packed / columnar `ArrayBox` residence
- materialization to a stable object is allowed only when a real boundary demands it

This is a language/runtime contract first.
Fast paths are a consequence of that contract, not the definition.

Internal reading:

- source word: `record`
- compiler/runtime lane: aggregate/value lane (`agg_local`, packed columns, scalar replacement)

### 2. Ordinary `box` keeps object-capable semantics

Ordinary `box` continues to mean:

- identity-capable object semantics
- alias-sensitive field mutation
- compatibility with reflection/dynamic/object-world behaviors
- no silent flattening just because fields happen to be scalar-friendly

Ordinary user-box fast paths may remove lookup/downcast overhead, but they must not erase object semantics.

### 3. `record` is the explicit source-level aggregate keep lane

Within `lifecycle-typed-value-language-ssot.md`, `record` is the first intended language-level candidate for the `agg_local` lane.

Read it as:

- inside hot local flow: aggregate value
- when stored in packed containers: column-friendly aggregate payload
- at host / identity / shared-alias boundary: materialize to stable object if required

### 4. No blanket user-box flattening

The repo stop-line stays fixed:

- do not flatten all ordinary user boxes
- do not infer `record` legality from current typed field fast paths
- do not treat `ArrayBox` typed-slot residence as proof that object identity can be removed
- do not use allocator-specific needs to smuggle in a generic identity rewrite

`record` is an explicit semantic lane or it does not exist.

### 5. First cut prefers replace-style updates

The first MVP should prefer value replacement:

```hako
local meta = metas.get(i)
local updated = new HakoAllocAlignedSmallMeta(
    meta.ptr,
    meta.alignment,
    meta.requested_size,
    new_size
)
metas.set(i, updated)
```

Not first-cut MVP:

- write-through element field mutation such as `metas[i].usable_size = new_size`
- hidden alias semantics for array elements

This keeps the initial contract readable and avoids partial object/reference semantics sneaking back in.

## Ordinary `box` vs `record`

| Surface | Ordinary `box` | `record` |
| --- | --- | --- |
| Identity | object-capable; may be observed | not part of the contract |
| Aliasing | alias-sensitive | value-copy semantics |
| Layout | runtime object layout may vary internally | fixed typed layout required |
| Local lowering | typed field fast path only | scalar tuple / aggregate lane allowed |
| `ArrayBox` residence | generic boxed or future specialized object path | packed/column residence allowed |
| Materialization | default object world | only when a real boundary demands it |

## MVP Restrictions

Initial `record` support should stay narrow:

- typed fields required
- no weak fields
- no `fini`
- no dynamic field creation
- no reflection-dependent semantics in the keep lane
- no promise of write-through field mutation for `ArrayBox` elements

Open policy choice later:

- equality may start as structural equality or explicit "no identity equality" only

## Implementation Order

`C198-C200` are already reserved by the proof-surface lane.
This lane therefore starts at `C201`.

| Row | Goal | Required before | Stop line |
| --- | --- | --- | --- |
| `C201 ordinary user-box field-index fast path` | keep ordinary `box` semantics but lower typed fields as `layout_id + field_index` and typed slots where legal | `record` surface work | no identity erasure, no packed container rewrite, no new syntax |
| `C202 record surface and semantics` | add docs/parser/semantic lock for `record` as the explicit source-level aggregate lane | local scalar replacement and packed storage | no blanket rewrite of ordinary `box`, no reflection/weak/fini support |
| `C203 record local scalar replacement` | keep non-escaping local `record` values as scalar tuple / aggregate carriers | packed `ArrayBox` storage | no host-boundary publication rewrite, no alias-sensitive mutation |
| `C204 ArrayBox inline-record storage` | add `ArrayBox` residence for packed `record` payloads (record columns) | allocator metadata migration | no change to ordinary boxed/mixed/string authority; `ArrayBox` stays the runtime owner |
| `C205 allocator metadata record migration` | replace hand-written scalar metadata arrays with `record` surface over packed storage | broader allocator/table cleanup | no allocator-specific DSL, no huge/native/provider coupling |

## Target Runtime Shape

The intended packed shape is columnar, for example:

```text
ArrayStorage::InlineRecord {
  layout_id,
  len,
  columns: [
    I64(...),
    I64(...),
    I64(...),
    I64(...),
  ]
}
```

Representative mapping:

```text
record HakoAllocAlignedSmallMeta {
  ptr: i64
  alignment: i64
  requested_size: i64
  usable_size: i64
}
```

may lower to:

```text
ptrs            -> column 0 (i64)
alignments      -> column 1 (i64)
requested_sizes -> column 2 (i64)
usable_sizes    -> column 3 (i64)
```

Important:

- this is a runtime/private lowering shape
- it does not change the public ABI by itself
- `ArrayBox` remains the authority for choosing when a lane must promote back to boxed storage

## Allocator Migration Reading

For allocator metadata, the preferred reading is:

1. keep `M178` scalar columns as the current truthful implementation
2. land `C201-C204` as compiler/runtime prerequisites
3. move allocator metadata surface to `record` only at `C205`

That preserves today's working implementation while making the long-term `.hako` surface cleaner again.

## Non-Goals

- flattening all user-defined boxes
- changing object identity rules for ordinary `box`
- moving `ArrayBox` authority into generic `NyashValue::Array`
- allocator-specific syntax or DSL
- packed storage for arbitrary reflection-heavy objects
- using `record` as a synonym for "faster if lucky"
