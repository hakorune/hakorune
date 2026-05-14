---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: MIMAP-004 substrate and representation gap ledger for mimalloc-shaped Hakorune allocator work.
Related:
  - docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# mimalloc Substrate and Representation Gap Ledger SSOT

## Decision

Treat missing mimalloc port requirements as explicit gap rows. Do not hide them
behind fallback behavior or C-shaped shortcuts.

```text
unsupported substrate:
  fail-fast

unsupported representation:
  fail-fast or choose a smaller executable slice

unsafe process integration:
  parked until optional provider ladder reopens
```

## Gap Classes

| Class | Meaning |
| --- | --- |
| `capability-gap` | Needs an explicit `uses` capability and backend support. |
| `representation-gap` | Needs a data representation that current Hakorune cannot faithfully model yet. |
| `semantic-gap` | Needs verifier/type semantics before it is safe. |
| `deferred-unsafe` | Intentionally parked until a separate optional ladder. |

## Ledger

| Gap | Class | Upstream concept | Hakorune requirement | Blocks | First safe substitute |
| --- | --- | --- | --- | --- | --- |
| `GAP-OSVM` | `capability-gap` | reserve/commit/decommit/reset/reuse/protect | `uses osvm` checker plus backend fail-fast support | real segment purge, guard pages, decommit/recommit | local page count model without OS memory |
| `GAP-ATOMIC` | `capability-gap` | CAS/load/store/fetch-add, delayed free flags, bitmap claim | `uses atomic` checker and memory-order policy | cross-thread free, abandoned reclaim, atomic bitmap | single-owner local page model |
| `GAP-TLS` | `capability-gap` | default heap, thread id, thread done hooks | explicit `uses tls` or host-thread capability decision | default heap fast path, thread-local heaps | pass heap object explicitly |
| `GAP-RAWBUF` | `capability-gap` / `representation-gap` | raw block pointer residence and segment pointer math | `RawBuf` / `Span<T>` / `view` no-escape model | real block memory, pointer validation | `BlockId` records and counters |
| `GAP-BITMAP` | `representation-gap` | commit masks, purge masks, arena bitmaps | explicit bitmap type or `PackedArray<usize>` bitfield policy | segment masks and arena claims | scalar mask record for tiny pilots |
| `GAP-CONST-EVAL` | `semantic-gap` | size-class/bin constants and generated tables | const table policy; later `const fn` / `const assert` | full size-class table generation | hand-written static const table |
| `GAP-RANDOM` | `capability-gap` | free-list encoding keys, secure randomization | entropy/random capability decision | encoded free lists, guarded security modes | deterministic model keys for proof-only docs |
| `GAP-PROTECT` | `capability-gap` | guard pages and protected memory | `uses osvm` protect route | guarded allocations | no guard page in first slice |
| `GAP-STATS-ATOMIC` | `capability-gap` | global stats merge and abandoned-page counters | local vs global stats ownership and atomic route | process-wide stats accuracy | local stats snapshot/counter model |
| `GAP-OVERRIDE` | `deferred-unsafe` | malloc/new-delete/posix replacement hooks | optional provider/global allocator ladder | host allocator replacement | explicit allocator object APIs only |
| `GAP-ABANDON-RECLAIM` | `semantic-gap` / `capability-gap` | abandoned segment/page reclaim | lifecycle transition checker plus atomic/TLS gates | cross-thread reclaim | same-owner reclaim simulation only |
| `GAP-PACKED-METADATA` | `representation-gap` | compact page/bin/segment metadata | source `PackedArray<T>` planning and backend lowerer | dense allocator metadata | normal `Array<T>` or record snapshots |

## Required Fail-Fast Policy

```text
PackedArray<T> unsupported:
  fail-fast, no boxed fallback

uses osvm unsupported:
  fail-fast, no simulated commit/decommit

uses atomic unsupported:
  fail-fast, no single-thread fallback for cross-thread APIs

uses rawbuf unsupported:
  fail-fast, no pointer-as-i64 shortcut

host allocator replacement inactive:
  fail-fast / not available, no hidden hook path
```

## Rows to Extract Later

These are not automatically active. They are row seeds for task planning.

| Seed | Purpose | Preferred lane |
| --- | --- | --- |
| `USES-002A` | capability checker for method-level `uses` metadata | language semantics |
| `OSVM-001` | reserve/commit/decommit/reset/reuse host capability contract | substrate |
| `ATOMIC-001` | atomic operation surface and memory-order policy | substrate |
| `TLS-001` | decide whether TLS is a language capability or host protocol only | substrate/design |
| `RAWBUF-001` | bounded raw buffer / span / view no-escape model | language semantics |
| `BITMAP-001` | bitmap representation policy for commit masks and arena claims | representation |
| `CONST-001` | const table generation / const assert plan | language semantics |
| `RANDOM-001` | entropy source capability for encoded free-list keys | substrate |
| `PACKED-005` | packed metadata backend lowering beyond metadata consumption | backend |
| `PROVIDER-001` | optional host allocator replacement ladder | parked optional lane |

## Executable Slice Filter

A first executable mimalloc-shaped row is acceptable only if it avoids all hard
gaps or marks them as explicit non-goals.

Allowed early slices:

```text
size-class/bin lookup with static const table
local stats snapshot/counter accumulation
local page free-count model with BlockId and PageId
page queue selection model without raw memory
```

Blocked early slices:

```text
real OS commit/decommit
cross-thread free
abandoned reclaim across heaps
raw pointer validation
atomic bitmap claim
global allocator replacement
```

## Next Row

`MIMAP-005A` should start the Hakorune blueprint skeleton by defining the
brand/type vocabulary that these gap rows need.
