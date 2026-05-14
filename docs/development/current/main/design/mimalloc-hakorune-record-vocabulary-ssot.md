---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-005B Hakorune record vocabulary for mimalloc-shaped allocator blueprint.
Related:
  - docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Hakorune Record Vocabulary SSOT

## Decision

Records are identity-free allocator facts. They describe references, table rows,
state snapshots, and counter snapshots. They do not own behavior, raw memory, or
lifecycle transitions.

```text
record:
  value aggregate / facts / table row / snapshot

box:
  owner / behavior / lifecycle mutation
```

## Stop Lines

```text
no record methods in this blueprint
no delegate on records
no raw pointer fields
no implicit identity
no ordinary box pretending to be a record
```

## Reference Records

```hako
record PageBlockRef {
    page_id: PageId
    block_id: BlockId
    generation: Generation
}

record SegmentPageRef {
    segment_id: SegmentId
    page_id: PageId
    generation: Generation
}

record HeapPageRef {
    heap_id: HeapId
    page_id: PageId
    generation: Generation
}
```

Purpose:

```text
PageBlockRef:
  release/check handle for local block operations

SegmentPageRef:
  segment/page membership without raw pointer math

HeapPageRef:
  explicit heap ownership without TLS/default heap
```

## Size-Class Records

```hako
record SizeClassEntry {
    size_class_id: SizeClassId
    block_size: Bytes
    usable_size: Bytes
    page_size: Bytes
    blocks_per_page: BlockCount
    bin_index: Index
}

record PageQueueRef {
    heap_id: HeapId
    size_class_id: SizeClassId
    first_page_id: PageId
    last_page_id: PageId
    page_count: PageCount
}
```

Purpose:

```text
SizeClassEntry:
  table row for near-transcription size/bin lookup

PageQueueRef:
  queue summary, not a linked-list pointer owner
```

`PageQueueRef` should be replaced by a box-owned queue model when mutation is
implemented.

## Page/Segment Snapshot Records

```hako
record PageSnapshot {
    page_id: PageId
    segment_id: SegmentId
    heap_id: HeapId
    generation: Generation
    capacity: BlockCount
    used: BlockCount
    local_free_count: BlockCount
    thread_free_count: BlockCount
    committed: Bytes
    reserved: Bytes
}

record SegmentSnapshot {
    segment_id: SegmentId
    arena_id: ArenaId
    generation: Generation
    used_pages: PageCount
    abandoned_pages: PageCount
    committed: Bytes
    reserved: Bytes
    purge_scheduled: Bytes
}
```

Purpose:

```text
PageSnapshot:
  verifier/proof/stat surface for page lifecycle

SegmentSnapshot:
  verifier/proof/stat surface for segment lifecycle and gap-ledger rows
```

## Lifecycle Counter Records

```hako
record PageLifecycleStats {
    allocated: Count
    released_local: Count
    retired: Count
    reclaimed: Count
    decommitted: Count
    reactivated: Count
    failed: Count
}

record SegmentLifecycleStats {
    reserved: Count
    committed: Count
    purged: Count
    abandoned: Count
    reclaimed: Count
    freed: Count
    failed: Count
}
```

Purpose:

```text
stats observer:
  read-only lifecycle event stats surface

proof apps:
  compare expected transition counts without mutating allocator state
```

## Gap-Summary Records

```hako
record CapabilityGap {
    name: String
    class_name: String
    required_for: String
    first_safe_substitute: String
}

record PackedMetadataCandidate {
    record_name: String
    field_count: Count
    scalar_field_count: Count
    requires_backend: bool
}
```

These are documentation/diagnostic records, not allocator runtime records.

## Materialization Policy

```text
normal Array<T>:
  acceptable for first docs/proof slices

PackedArray<T>:
  allowed only when a row requires packed residence and backend support is ready

record get/set:
  value replacement style; no field write-through in arrays for MVP
```

## Next Row

`MIMAP-005C` should bind these records to enum/transition lifecycle vocabulary
without implementing behavior.
