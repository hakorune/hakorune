---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-005C non-executable enum/transition lifecycle skeleton for mimalloc-shaped Hakorune blueprint.
Related:
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-brand-type-vocabulary-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-record-vocabulary-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Hakorune Lifecycle Skeleton SSOT

## Decision

The blueprint skeleton uses ordinary `enum` values plus `transition` metadata.
It does not introduce a `state` keyword and does not implement allocator
behavior.

## Page and Segment Enums

```hako
enum PageState {
    Fresh
    Active
    Full
    Retired
    Abandoned
    Reclaimed
    PurgeScheduled
    Decommitted
    Freed
}

enum SegmentState {
    Reserved
    Active
    PurgeScheduled
    Purged
    Abandoned
    Reclaimed
    Freed
}

enum BlockState {
    Free
    Allocated
    LocalReleased
    ThreadReleased
    Reclaimed
    Invalid
}
```

## Error Enum

```hako
enum AllocLifecycleError {
    UnknownPointer
    StaleGeneration
    DoubleRelease
    WrongOwner
    DecommittedPage
    AbandonedPage
    UnsupportedCapability
    InvariantViolation
}
```

## Page Owner Skeleton

```hako
box HakoAllocPageModel {
    page_id: PageId = PageId(0)
    segment_id: SegmentId = SegmentId(0)
    heap_id: HeapId = HeapId(0)
    generation: Generation = Generation(0)
    state: PageState = PageState::Fresh
    capacity: BlockCount = 0
    used: BlockCount = 0
    local_free_count: BlockCount = 0
    thread_free_count: BlockCount = 0
    committed: Bytes = 0
    reserved: Bytes = 0

    invariant used <= capacity
    invariant committed <= reserved

    transition PageState::Fresh -> PageState::Active by initPage
    transition PageState::Active -> PageState::Full by moveToFull
    transition PageState::Full -> PageState::Active by unfull
    transition PageState::Active -> PageState::Retired by retire
    transition PageState::Retired -> PageState::Active by reuseRetired
    transition PageState::Retired -> PageState::Freed by collectRetired
    transition PageState::Active -> PageState::Abandoned by abandonPage
    transition PageState::Abandoned -> PageState::Reclaimed by reclaimPage
    transition PageState::Reclaimed -> PageState::Active by attachReclaimed
    transition PageState::Retired -> PageState::PurgeScheduled by schedulePagePurge
    transition PageState::PurgeScheduled -> PageState::Decommitted by purgePageMemory
    transition PageState::Decommitted -> PageState::Active by reactivatePage
}
```

## Segment Owner Skeleton

```hako
box HakoAllocSegmentModel {
    segment_id: SegmentId = SegmentId(0)
    arena_id: ArenaId = ArenaId(0)
    heap_id: HeapId = HeapId(0)
    generation: Generation = Generation(0)
    state: SegmentState = SegmentState::Reserved
    used_pages: PageCount = 0
    abandoned_pages: PageCount = 0
    committed: Bytes = 0
    reserved: Bytes = 0
    purge_scheduled: Bytes = 0

    invariant abandoned_pages <= used_pages
    invariant committed <= reserved
    invariant purge_scheduled <= committed

    transition SegmentState::Reserved -> SegmentState::Active by initSegment
    transition SegmentState::Active -> SegmentState::PurgeScheduled by schedulePurge
    transition SegmentState::PurgeScheduled -> SegmentState::Purged by purgeSegment
    transition SegmentState::Purged -> SegmentState::Active by reusePurged
    transition SegmentState::Active -> SegmentState::Abandoned by abandonSegment
    transition SegmentState::Abandoned -> SegmentState::Reclaimed by reclaimSegment
    transition SegmentState::Reclaimed -> SegmentState::Active by attachSegment
    transition SegmentState::Active -> SegmentState::Freed by freeSegment
    transition SegmentState::Purged -> SegmentState::Freed by freePurgedSegment
}
```

## Method Contract Seeds

These are contract shapes only; method bodies are intentionally omitted.

```hako
releaseLocal(block_ref: PageBlockRef): Result<void, AllocLifecycleError>
    requires state == PageState::Active
    ensures used <= capacity

retire(): Result<void, AllocLifecycleError>
    requires state == PageState::Active
    ensures state == PageState::Retired

reactivatePage(): Result<void, AllocLifecycleError>
    uses osvm
    requires state == PageState::Decommitted
    ensures state == PageState::Active
```

## Skeleton Rules

```text
non-executable:
  this is not allocator behavior yet

metadata-only:
  transition declarations are verifier input, not Stage0 semantics

capability-visible:
  methods that require OSVM/atomic/TLS/rawbuf must say so
```

## Next Row

`MIMAP-005D` should define the capability surface used by this skeleton.
