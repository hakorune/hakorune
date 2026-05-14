---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: Map / Set / HashMap naming, FST placement, and ordering relative to mimalloc port work.
Related:
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - lang/src/runtime/collections/README.md
---

# Collection Set/Map/FST Task Breakdown SSOT

## Decision

Keep collection and automata surface small.

```text
Map:
  canonical user-visible collection name

HashMap:
  implementation detail, not canonical source surface

Set:
  ring1 semantic wrapper over Map, not Stage0 and not a raw substrate first

Finite State Transducer:
  library / compiler-tooling data structure, not language core
```

## Current Evidence

`lang/src/runtime/collections/README.md` names ring1 collection core as the
visible owner frontier for `ArrayBox` and `MapBox` semantics. `MapCoreBox` owns
visible `MapBox.{set,get,has,size/len/length}` orchestration and delegates raw
storage/probe details to `RawMapCoreBox` / substrate routes.

No current canonical Set owner or FST owner is established.

## Mimalloc Ordering Cut

Do not move Set or FST ahead of current mimalloc work by default.

```text
MIMAP-008 page/free-list model pilot:
  does not require Set
  does not require FST
  can use counters, records, Array, and existing SizeClassBox

MIMAP-009 lifecycle integration pilot:
  does not require FST
  may use existing MapBox if a lookup table is needed
  should not introduce Set unless membership semantics become the blocker
```

If a mimalloc row needs dynamic membership, prefer this order:

```text
1. use explicit Array/counter/record model if bounded and local
2. use existing MapBox if key/value lookup is genuinely needed
3. open COLL-002/COLL-003 only if unique-membership Set semantics are the blocker
4. do not open AUTO rows for mimalloc unless a static dictionary/route matcher is proven necessary
```

## Canonical Naming

```text
canonical:
  Array<T>
  Map<K,V>
  Set<T>
  PackedArray<T>

not canonical:
  HashMap
  HashSet
  Dict
  Vec
  List
```

`HashMap` and `HashSet` may appear in Rust implementation notes, but not as
Hakorune source-level canonical names.

## Task Rows

### Collection Rows

| Row | Status | Scope | Ordering |
| --- | --- | --- | --- |
| `COLL-001` | ready | Map / Set / HashMap naming and placement docs. | may land anytime; docs-only |
| `COLL-002` | parked | `Set<T>` semantic wrapper over `Map<T,i64>`; no raw Set substrate. | after MIMAP-008 unless Set is a blocker |
| `COLL-003` | parked | Set proof app and guard for `add/has/remove/size/clear`. | after COLL-002 |
| `COLL-004` | parked | Key capability inventory: String, i64, brand-over-i64 supported; unsupported keys fail-fast. | before generic Set/Map widening |
| `COLL-005` | deferred | Hash/equality interface or where-constraint story. | after interface/where rows |

### Automata Rows

| Row | Status | Scope | Ordering |
| --- | --- | --- | --- |
| `AUTO-001` | ready | FST placement SSOT: compiler/std automata, not language core. | docs-only, not mimalloc prerequisite |
| `AUTO-002` | parked | `FstState` / `FstTransition` record vocabulary, Array/PackedArray-backed. | after PackedArray backend route is useful |
| `AUTO-003` | parked | Compiler keyword-table FST pilot. | only if parser/compiler dictionary evidence appears |
| `AUTO-004` | deferred | Public std automata package. | after compiler pilot or app evidence |

## Set MVP Shape

`Set<T>` starts as visible membership semantics over Map storage.

```hako
box Set<T> {
    map: Map<T, i64> = new Map<T, i64>()

    add(value: T): i64
    has(value: T): i64
    remove(value: T): i64
    size(): usize
    clear()
}
```

MVP return policy:

```text
add:
  1 if inserted, 0 if already present

remove:
  1 if removed, 0 if absent

has:
  1 if present, 0 if absent
```

Use `i64` 0/1 for the first proof-oriented row. A later Bool-facing API can be
added only if it does not create duplicate canonical behavior.

## FST Placement

FST is not a language keyword or builtin syntax.

```text
compiler use:
  lang/src/compiler/automata/... if keyword/route lookup needs it

stdlib use:
  lang/src/std/automata/... only after compiler/app evidence

substrate:
  raw transition tables may use Array/PackedArray; no RawFst substrate first
```

FST is not a Map replacement:

```text
Map:
  dynamic key/value collection

FST:
  static or mostly immutable byte/string dictionary and route matcher
```

## Stop Lines

```text
no Stage0 Set
no Stage0 FST
no HashMap as canonical source name
no RawSet substrate before Set semantics
no FST language keyword
no FST as MapBox backend without evidence
no wildcard generic key support without fail-fast key capability checks
```
