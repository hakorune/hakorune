---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-005D capability surface for mimalloc-shaped Hakorune allocator blueprint.
Related:
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-lifecycle-skeleton-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Hakorune Capability Surface SSOT

## Decision

Low-level allocator effects are explicit capabilities. They are not hidden behind
`unsafe`, backend route selection, or silent fallback.

```text
canonical:
  uses osvm
  uses atomic
  uses rawbuf

provisional capability decisions:
  uses tls
  uses random

not a capability surface:
  provider activation
  hooks
  global allocator replacement
```

## Capability Table

| Capability | Enables | First blocked concepts | Fail-fast rule |
| --- | --- | --- | --- |
| `uses osvm` | reserve, commit, decommit, reset, reuse, protect, release OS memory | segment purge, page decommit/reactivate, guard pages | unsupported backend rejects; no simulated OSVM |
| `uses atomic` | CAS/load/store/fetch operations and memory-order-sensitive state | cross-thread free, abandoned reclaim, atomic bitmap, global stats | unsupported backend rejects; no single-thread fallback for atomic APIs |
| `uses rawbuf` | bounded raw memory residence and views | real block payload, pointer validation, span access | unsupported backend rejects; no pointer-as-i64 shortcut |
| `uses tls` | default heap and thread id fast path | thread-local default heap, thread done hooks | provisional; explicit decision row required before implementation |
| `uses random` | entropy for encoded free-list keys and secure modes | encoded free lists, guarded security modes | provisional; deterministic test keys only in proof-only models |

## Explicitly Inactive Surfaces

```text
provider activation:
  inactive

host allocator replacement:
  inactive

hooks / #[global_allocator]:
  inactive

malloc/new-delete/posix override:
  deferred-unsafe
```

These must not be modeled as `uses` in the first blueprint. They belong to a
separate optional provider ladder if reopened.

## Method Seeds

```hako
reserveSegment(size: Bytes): Result<SegmentId, AllocLifecycleError>
    uses osvm

commitSegment(segment_id: SegmentId, offset: Offset, size: Bytes): Result<void, AllocLifecycleError>
    uses osvm

decommitPage(page_id: PageId): Result<void, AllocLifecycleError>
    uses osvm

releaseFromOtherThread(block_ref: PageBlockRef): Result<void, AllocLifecycleError>
    uses atomic

claimBitmapRange(start: Index, count: Count): Result<void, AllocLifecycleError>
    uses atomic

writeBlockBytes(block_ref: PageBlockRef, offset: Offset, len: Bytes): Result<void, AllocLifecycleError>
    uses rawbuf
```

`uses tls` and `uses random` are intentionally not given executable method seeds
until their policy rows are accepted.

## No-Fallback Contract

```text
osvm unavailable:
  fail-fast before lowering OS memory behavior

atomic unavailable:
  fail-fast before lowering cross-thread or atomic bitmap behavior

rawbuf unavailable:
  fail-fast before exposing block payload residence

tls undecided:
  use explicit heap object, not default heap

random undecided:
  do not implement encoded free-list security behavior
```

## First Executable Slice Rule

MIMAP-006 may choose a slice that uses no capability or only a capability already
accepted by an implementation row.

Preferred no-capability candidates:

```text
size-class/bin lookup
stats snapshot/counter accumulation
local page free-count model
page queue selection model
```

## Next Row

`MIMAP-006` should select one executable near-transcription slice and explicitly
state which capabilities are not used.
