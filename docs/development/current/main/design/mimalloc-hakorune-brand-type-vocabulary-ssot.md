---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-005A Hakorune brand/type vocabulary for mimalloc-shaped allocator blueprint.
Related:
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Hakorune Brand/Type Vocabulary SSOT

## Decision

Allocator identity values are branded. Measurement units are type aliases unless
mixing them would hide a lifecycle bug.

```text
brand:
  identity / generation / owner boundary

type:
  units and readable scalar aliases
```

This keeps the surface small while preventing common allocator scalar mixups.

## Canonical Type Aliases

```hako
type Bytes = usize
type Count = usize
type Index = usize
type Offset = usize
type Alignment = usize
type SliceCount = usize
type BlockCount = usize
type PageCount = usize
type BinCount = usize
```

Rules:

```text
Bytes:
  byte sizes and usable/requested sizes

Count:
  generic non-negative counts when a more specific alias is not useful

Index / Offset:
  array/table offsets; prefer Index for container slots and Offset for byte/slice offsets

Alignment:
  power-of-two alignment values; verifier row should later assert this

SliceCount / BlockCount / PageCount / BinCount:
  readable aliases for allocator cardinalities
```

## Canonical Brands

```hako
brand HeapId: i64
brand SegmentId: i64
brand PageId: i64
brand BlockId: i64
brand ArenaId: i64
brand SizeClassId: i64
brand Generation: i64
brand ThreadId: i64
```

Rules:

| Brand | Meaning | First use |
| --- | --- | --- |
| `HeapId` | Explicit heap owner identity. | Avoid default/TLS heap in first model. |
| `SegmentId` | Segment identity independent of raw pointer address. | Segment lifecycle blueprint. |
| `PageId` | Page identity independent of segment slice pointer. | Page lifecycle and stats. |
| `BlockId` | Block identity independent of raw pointer address. | Local free-count and release model. |
| `ArenaId` | Arena/source identity for future OS/arena substrate. | Gap ledger / later OSVM rows. |
| `SizeClassId` | Size-class/bin identity. | Size-class table and page queue selection. |
| `Generation` | Stale handle/lifecycle reuse discriminator. | release/reclaim/reactivate guards. |
| `ThreadId` | Thread owner identity. | Parked until TLS/capability decision. |

## Boundary Constructors

The blueprint may show explicit construction at trusted boundaries:

```hako
local page_id = PageId(1)
local segment_id = SegmentId(1)
local generation = Generation(0)
```

Policy:

```text
constructor use:
  allowed at model setup / trusted source boundaries

brand mixing:
  rejected by Stage1 brand semantics

unwrap:
  later explicit policy only; do not rely on implicit i64 coercion
```

## Non-Goals

```text
no pointer-as-i64 shortcut
no implicit conversion between PageId / BlockId / SegmentId
no ThreadId behavior before TLS capability decision
no ArenaId behavior before OSVM/arena substrate rows
no branding of every count; use aliases unless identity matters
```

## Blueprint Seed

```hako
type Bytes = usize
type Count = usize
type Index = usize
type Offset = usize
type Alignment = usize
type SliceCount = usize
type BlockCount = usize
type PageCount = usize
type BinCount = usize

brand HeapId: i64
brand SegmentId: i64
brand PageId: i64
brand BlockId: i64
brand ArenaId: i64
brand SizeClassId: i64
brand Generation: i64
brand ThreadId: i64
```

## Next Row

`MIMAP-005B` should define record vocabulary using these names. Records should
stay identity-free and must not model raw pointer residence yet.
