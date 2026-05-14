---
Status: Active
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-002 source concept inventory for mimalloc blueprint work.
Related:
  - docs/development/current/main/investigations/mimalloc-upstream-pin.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
  - docs/development/current/main/phases/phase-293x/293x-332-MIMAP-002-SOURCE-CONCEPT-INVENTORY.md
---

# mimalloc Source Concept Inventory

## Purpose

Classify upstream mimalloc concepts before any Hakorune implementation starts.
This is not a translation plan. It is a map from C concepts to Hakorune-owned
meaning.

## Source Evidence Window

Pinned source tree:

```text
.external/upstream/mimalloc/
```

Files inspected for this inventory:

```text
include/mimalloc/types.h
include/mimalloc/internal.h
include/mimalloc/atomic.h
include/mimalloc/prim.h
src/alloc.c
src/free.c
src/heap.c
src/page.c
src/page-queue.c
src/segment.c
src/segment-map.c
src/arena.c
src/arena-abandon.c
src/bitmap.c
src/bitmap.h
src/os.c
src/options.c
src/stats.c
src/init.c
```

## Classification Legend

| Class | Meaning |
| --- | --- |
| `near-transcription` | Algorithmic shape can be kept close to upstream once types are explicit. |
| `lifecycle-rewrite` | Algorithmic goal is similar, but ownership/state transitions must be explicit. |
| `substrate-gap` | Requires host/runtime capability such as OSVM, atomic, TLS, or raw memory. |
| `representation-gap` | Requires storage representation not yet fully modeled in Hakorune. |
| `deferred-unsafe` | Must stay parked until a capability/fail-fast gate exists. |

## Concept Map

| Concept | Upstream anchors | Observed meaning | Hakorune mapping | Class |
| --- | --- | --- | --- | --- |
| `mi_block_t` | `include/mimalloc/types.h`, `src/free.c` | Allocation unit linked through encoded free-list next pointers. | `BlockId` / `BlockRef` records first; raw pointer residence later behind `uses rawbuf`. | `representation-gap` |
| Page free lists | `include/mimalloc/types.h`, `src/alloc.c`, `src/free.c`, `src/page.c` | Pages carry available, local-deferred, and cross-thread free lists. | Explicit page state plus separate local/free/thread-free views; no implicit pointer list ownership. | `lifecycle-rewrite` |
| `mi_page_t` | `include/mimalloc/types.h`, `src/page.c` | Owns block capacity/used counts, free-list heads, heap link, queue links, commitment flags, and retire state. | `box HakoAllocPageModel` with branded ids, counters, lifecycle enum, and verifier invariants. | `lifecycle-rewrite` |
| Page queues / bins | `include/mimalloc/types.h`, `src/page-queue.c`, `src/heap.c`, `src/page.c` | Heap keeps per-size-class queues and direct small-page lookup. | `Array<PageQueue>` / `PackedArray<SizeClassEntry>` candidates; direct lookup is near-transcription after table shape is fixed. | `near-transcription` / `representation-gap` |
| Small allocation fast path | `src/alloc.c`, `include/mimalloc/internal.h` | Pop from current page free list; fallback to generic page finding when empty. | First executable slice can model index/block-id pop without raw memory. | `near-transcription` after lifecycle model |
| Generic allocation path | `src/page.c`, `src/heap.c` | Collect delayed frees, find/extend/fresh page, retry after collection. | Requires explicit `Result<T,E>`, page lifecycle guards, and collection policy. | `lifecycle-rewrite` |
| Local free | `src/free.c`, `src/page.c` | Local frees push to page-local deferred list and may retire/unfull page. | Model as `releaseLocal(BlockId)` with page-state guard and postcondition. | `lifecycle-rewrite` |
| Cross-thread free | `src/free.c`, `include/mimalloc/atomic.h` | Atomic thread-free list with delayed-free flags and owning-heap handoff. | Requires `uses atomic` and likely `uses tls`; not part of first executable slice. | `substrate-gap` |
| Abandoned page/segment reclaim | `src/free.c`, `src/segment.c`, `src/arena-abandon.c`, `include/mimalloc/internal.h` | Pages/segments can lose owning thread and be reclaimed by another heap. | Explicit `Abandoned` / `Reclaimed` lifecycle states, generation checks, and transition guards. | `lifecycle-rewrite` / `substrate-gap` |
| `mi_segment_t` | `include/mimalloc/types.h`, `src/segment.c` | Segment owns slices/pages, commit and purge masks, thread ownership, abandoned counters, arena/os memory id. | `SegmentId`, `SegmentState`, commit/purge records, and explicit OSVM capability gates. | `lifecycle-rewrite` |
| Commit/purge masks | `include/mimalloc/types.h`, `src/segment.c` | Bit masks track committed and scheduled-purge ranges. | `PackedArray` / bitmap model later; no silent boxed fallback for packed residence. | `representation-gap` |
| Segment map | `src/segment-map.c`, `include/mimalloc/internal.h` | Global map checks whether a pointer belongs to a known segment. | Deferred until raw pointer identity and substrate membership are modeled. | `substrate-gap` |
| Arena allocation | `src/arena.c`, `src/arena-abandon.c`, `include/mimalloc/internal.h` | Arena routes OS/arena memory, abandonment metadata, and large allocation backing. | Host substrate capability surface, not first Hakorune model. | `substrate-gap` |
| OS memory primitives | `include/mimalloc/prim.h`, `src/os.c`, `src/prim/*` | Reserve/commit/decommit/reset/reuse/protect and platform memory config. | `uses osvm` required; unsupported backend must fail-fast. | `substrate-gap` |
| Atomics | `include/mimalloc/atomic.h`, `src/bitmap.c`, `src/free.c`, `src/stats.c` | CAS/load/store/fetch operations for thread-free lists, bitmaps, and global stats. | `uses atomic` required; local single-thread pilot should avoid it. | `substrate-gap` |
| TLS / default heap | `include/mimalloc/prim.h`, `src/init.c`, `src/heap.c` | Fast path depends on thread-local default heap and unique thread id. | Defer global/default heap semantics; model explicit heap object first. | `substrate-gap` |
| Bitmap helpers | `src/bitmap.c`, `src/bitmap.h` | Atomic find/claim/unclaim across bitmap fields. | Later `Bitmap`/`PackedArray<usize>` design; first slice can use scalar/array stand-in with fail-fast label. | `representation-gap` |
| Stats counters | `src/stats.c`, `include/mimalloc-stats.h`, `include/mimalloc/types.h` | Counters track malloc bins, page bins, committed/reserved/purge/segment/page events. | Existing lifecycle stats observer work maps well to record snapshots and counter arrays. | `near-transcription` |
| Options | `src/options.c`, `include/mimalloc/prim.h` | Runtime knobs sourced from environment/options. | Keep out of first allocator model; can become explicit config record. | `near-transcription` later |
| Guard pages / protected allocations | `src/alloc.c`, `src/free.c`, `src/os.c` | Uses OS protect/decommit behavior around objects. | Defer behind `uses osvm` + raw view/protect capability. | `deferred-unsafe` |
| Override / replacement hooks | `include/mimalloc-override.h`, `include/mimalloc-new-delete.h`, `src/alloc-override.c`, `src/alloc-posix.c` | Replaces process allocation entry points. | Explicitly inactive. | `deferred-unsafe` |

## Near-Transcription Candidates

These can become early executable slices once the row explicitly selects one:

| Candidate | Why it is small | Required Hakorune surface |
| --- | --- | --- |
| Stats snapshot/counter accumulation | Mostly scalar counters and bins. | `record`, `Array<T>`, `Result`, existing stats observer docs. |
| Size-class/bin lookup | Arithmetic/table-driven and isolated from OSVM. | `static const` table or explicit `Array<usize>` seed. |
| Page local free-count model | Can avoid raw pointers by using `BlockId` and counts. | `brand`, `record`, `enum`, `transition`, `requires`. |
| Page queue selection model | Uses bins and queue metadata without real allocation. | `Array<PageQueue>`, `loop i in 0..count`, no raw memory. |

## Lifecycle-Rewrite Core

Upstream hides several states in pointer ownership, flags, queue membership, and
thread ids. Hakorune should name these states instead of copying the hidden shape.

Initial state vocabulary for MIMAP-003:

```text
PageState:
  Fresh
  Active
  Full
  Retired
  Freeing
  Abandoned
  Reclaimed
  Decommitted

SegmentState:
  Reserved
  Active
  PurgeScheduled
  Purged
  Abandoned
  Reclaimed
  Freed
```

Initial transition families for MIMAP-003:

```text
page fresh -> active by initPage
page active -> full by moveToFull
page full -> active by unfull
page active -> retired by retire
page retired -> freeing by collectRetired
page active -> abandoned by abandonPage
page abandoned -> reclaimed by reclaimPage
page retired/freeing -> decommitted by purgePageMemory

segment reserved -> active by segmentInit
segment active -> purgeScheduled by schedulePurge
segment purgeScheduled -> purged by purgeSegment
segment active -> abandoned by abandonSegment
segment abandoned -> reclaimed by reclaimSegment
segment active/purged -> freed by freeSegment
```

## Gap Ledger Seeds

These should feed MIMAP-004.

| Gap | Evidence family | Needed before faithful port |
| --- | --- | --- |
| Raw pointer/block residence | block free lists and segment pointer math | `RawBuf` / `Span<T>` / no-escape view design |
| Atomic cross-thread free | `xthread_free`, delayed flags, bitmap CAS | `uses atomic` checker and backend gate |
| TLS/default heap | thread-local default heap and unique thread id | explicit `uses tls` or host-thread capability decision |
| OSVM lifecycle | reserve/commit/decommit/reset/reuse/protect | `uses osvm` checker and fail-fast backend support |
| Bitmap/commit-mask storage | segment masks and arena bitmaps | packed scalar storage or explicit bitmap type |
| Global allocator replacement | override/new-delete/posix routes | optional provider ladder, currently inactive |

## Recommended Next Row

`MIMAP-003 lifecycle rewrite blueprint` should turn the state vocabulary above
into a Hakorune-shaped lifecycle blueprint. It should not implement allocator
behavior yet.
