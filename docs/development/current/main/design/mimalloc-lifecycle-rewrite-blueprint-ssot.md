---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-003 lifecycle rewrite blueprint for mimalloc-shaped Hakorune allocator work.
Related:
  - docs/development/current/main/investigations/mimalloc-upstream-pin.md
  - docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Lifecycle Rewrite Blueprint SSOT

## Decision

Do not copy mimalloc's pointer/flag/queue lifecycle directly. Hakorune owns the
lifecycle as explicit enum states, transitions, guard contracts, and fail-fast
capability boundaries.

```text
upstream C:
  state is spread across flags, queue membership, thread id, commit masks, and pointer ownership

Hakorune:
  state is explicit and verifier-readable
```

## Non-Goals

```text
no executable allocator behavior in MIMAP-003
no provider activation
no host allocator replacement
no hooks / global allocator replacement
no raw pointer model
no OSVM or atomic implementation
```

## Canonical Surface

Use existing or planned canonical Hakorune surface only:

```text
enum       lifecycle state values
transition legal state moves
requires   caller-side guard contract
ensures    post-state contract
invariant  object consistency
uses       explicit low-level capability declaration
Result     fail-fast return surface
```

No `state` keyword is introduced. Lifecycle values are ordinary enums.

## Page Lifecycle

### State Vocabulary

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
```

| State | Meaning | Notes |
| --- | --- | --- |
| `Fresh` | Page metadata exists but free-list/capacity setup is not complete. | Construction-only state. |
| `Active` | Page belongs to a heap and can allocate/free local blocks. | Normal local model state. |
| `Full` | Page has no immediate local availability and is in the full route. | Can become active again after free/unfull. |
| `Retired` | Page is fully free but kept briefly for reuse. | Retire counters are explicit. |
| `Abandoned` | Owning heap/thread is gone or no longer owns the page. | Reclaim requires ownership transfer. |
| `Reclaimed` | Previously abandoned page is now attached to a live heap. | Must refresh generation/owner facts. |
| `PurgeScheduled` | Memory may be reset/decommitted after a delay. | Requires segment/OSVM context. |
| `Decommitted` | Backing memory is not currently usable. | Reuse requires recommit/reuse guard. |
| `Freed` | Page metadata is no longer owned by the heap model. | Terminal for the local model. |

### Transition Table

| From | To | By | Required guard | Capability |
| --- | --- | --- | --- | --- |
| `Fresh` | `Active` | `initPage` | capacity/reserved fields are initialized | none |
| `Active` | `Full` | `moveToFull` | no immediate free block and not expandable | none |
| `Full` | `Active` | `unfull` | a block becomes locally available | none |
| `Active` | `Retired` | `retire` | all blocks are free and page is not the only reusable page | none |
| `Retired` | `Active` | `reuseRetired` | page is still committed and generation matches | none |
| `Retired` | `Freed` | `collectRetired` | retire counter expired or force collection | none / segment owner |
| `Active` | `Abandoned` | `abandonPage` | owning heap is ending and live blocks may remain | `atomic` later |
| `Abandoned` | `Reclaimed` | `reclaimPage` | claimant heap owns segment/subprocess route | `atomic` / `tls` later |
| `Reclaimed` | `Active` | `attachReclaimed` | owner/generation refreshed | none |
| `Retired` | `PurgeScheduled` | `schedulePagePurge` | page is fully free | `osvm` later |
| `PurgeScheduled` | `Decommitted` | `purgePageMemory` | purge delay expired or force purge | `osvm` |
| `Decommitted` | `Active` | `reactivatePage` | recommit/reuse succeeds and generation advances | `osvm` |

### Page Invariants

```text
used <= capacity
committed <= reserved
state == Active or Full implies owner_heap_id is present
state == Abandoned implies owner_heap_id is absent or stale by generation
state == Decommitted implies no visible block access is allowed
state == Freed is terminal
block generation must match page generation on release
```

### Page Guard Points

| Operation | Guard |
| --- | --- |
| `allocateLocal` | page is `Active`, generation matches, and immediate/extendable capacity exists. |
| `releaseLocal` | page is `Active` or `Full`, block is allocated, owner/generation matches, and page is not decommitted. |
| `retire` | all blocks are free, page is not already abandoned/decommitted/freed. |
| `reclaimPage` | page is `Abandoned`, segment owner route allows claim, and generation is refreshed. |
| `reactivatePage` | page is `Decommitted`, OSVM recommit/reuse succeeds, and generation advances. |

## Block Lifecycle

```hako
enum BlockState {
    Free
    Allocated
    LocalReleased
    ThreadReleased
    Reclaimed
    Invalid
}
```

| From | To | By | Notes |
| --- | --- | --- | --- |
| `Free` | `Allocated` | `allocateLocal` | Pop from page-local availability. |
| `Allocated` | `LocalReleased` | `releaseLocal` | Same owner/heap route. |
| `Allocated` | `ThreadReleased` | `releaseFromOtherThread` | Deferred until atomic/TLS capability exists. |
| `LocalReleased` | `Free` | `collectLocalFree` | Makes block immediately allocatable. |
| `ThreadReleased` | `Reclaimed` | `collectThreadFree` | Requires atomic delayed-free semantics. |
| `Reclaimed` | `Free` | `attachReclaimedBlock` | Owner/generation normalized. |
| any non-`Allocated` | `Invalid` | `release*` | Double release / stale pointer fail-fast. |

First executable models should use `BlockId` and counters. Raw pointer residence
is a later `uses rawbuf` row.

## Segment Lifecycle

```hako
enum SegmentState {
    Reserved
    Active
    PurgeScheduled
    Purged
    Abandoned
    Reclaimed
    Freed
}
```

| From | To | By | Required guard | Capability |
| --- | --- | --- | --- | --- |
| `Reserved` | `Active` | `initSegment` | slices/pages metadata initialized | `osvm` later |
| `Active` | `PurgeScheduled` | `schedulePurge` | purge mask non-empty and purging allowed | `osvm` later |
| `PurgeScheduled` | `Purged` | `purgeSegment` | purge delay expired or force purge | `osvm` |
| `Purged` | `Active` | `reusePurged` | reuse/recommit succeeds | `osvm` |
| `Active` | `Abandoned` | `abandonSegment` | owning thread/heap exits with live pages | `atomic` / `tls` later |
| `Abandoned` | `Reclaimed` | `reclaimSegment` | claimant can clear abandoned route | `atomic` / `tls` later |
| `Reclaimed` | `Active` | `attachSegment` | owner and tld facts refreshed | none |
| `Active` | `Freed` | `freeSegment` | used pages are zero and no abandoned pages remain | `osvm` later |
| `Purged` | `Freed` | `freePurgedSegment` | no live pages remain | `osvm` later |

### Segment Invariants

```text
abandoned <= used
purge_mask subset_of commit_mask
state == Freed is terminal
state == Abandoned implies thread owner is absent/stale
state == Purged implies pages in purged range cannot be used until reuse/recommit
```

## Heap Lifecycle Boundary

Heap/default-thread behavior is intentionally not the first model.

```text
first model:
  explicit HakoAllocHeapModel object passed to operations

later substrate:
  default heap / TLS / thread id capability
```

This prevents global allocator replacement from leaking into the blueprint.

## Error Vocabulary Seed

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

## Capability Boundaries

| Capability | Enables | Stop line |
| --- | --- | --- |
| none | local page/block/count model, stats, size-class tables | no raw memory claim |
| `uses osvm` | reserve/commit/decommit/reset/reuse/protect | unsupported backend fail-fast |
| `uses atomic` | cross-thread free, abandoned reclaim, bitmap claim | no silent single-thread fallback |
| `uses tls` | default heap and thread id fast path | explicit capability decision required |
| `uses rawbuf` | block pointer residence and bounded views | no escape outside view scope |

## First Executable Slice Guidance

MIMAP-006 should prefer one of:

```text
size-class/bin lookup
stats snapshot/counter accumulation
local page free-count model
page queue selection model
```

It should avoid:

```text
cross-thread free
raw pointer block lists
OSVM commit/decommit
TLS default heap
global allocator replacement
```

## Next Row

`MIMAP-004 substrate and representation gap ledger` should convert the capability
boundaries and representation gaps into explicit missing-feature rows.
