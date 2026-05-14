---
Status: Active
Date: 2026-05-14
Lane: phase-293x mimalloc blueprint / port preparation
Canonical SSOT:
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
---

# Phase 293x mimalloc Port Taskboard

## Current Status

This board is active. The language-minimal prerequisite set has reached the
safe handoff point for mimalloc blueprint work.

Preferred handoff point:

```text
LOOP-003C/D complete
PACKED-003/004 complete
```

Blueprint and inventory rows are now the active lane entry. Current row:
`MIMAP-011 allocator facade lifecycle route pilot`.

## Active Source Policy

Upstream mimalloc source is local-only:

```text
.external/upstream/mimalloc/
```

Tracked output is docs only.

## Collection / Automata Dependency Cut

Map/Set/FST work is tracked in:

```text
docs/development/current/main/design/collection-set-map-fst-task-breakdown-ssot.md
```

Decision for this board:

```text
Set:
  not a prerequisite for MIMAP-011

Map:
  existing MapBox / MapCoreBox is enough if a later row needs dynamic lookup

FST:
  not a mimalloc prerequisite
```

## Rows

### Source and Inventory

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-001` | landed | Upstream source pin: URL, commit/tag, license, inspected files, local path. | 1 commit |
| `MIMAP-002` | landed | Source concept inventory: segment/page/block/heap/free-list/size-class/os/stats. | 1-2 commits |
| `MIMAP-003` | landed | Lifecycle rewrite blueprint: enum states, transitions, guard points. | 1-2 commits |
| `MIMAP-004` | landed | Substrate/representation gap ledger from source evidence. | 1-2 commits |

### Hakorune Blueprint

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-005A` | landed | Define brand/type vocabulary: `Bytes`, `PageId`, `BlockId`, `SegmentId`, `Generation`. | 1 commit |
| `MIMAP-005B` | landed | Define record vocabulary for page/block refs, size-class entries, stats snapshots. | 1 commit |
| `MIMAP-005C` | landed | Define enum/transition lifecycle blueprint for page/segment state. | 1 commit |
| `MIMAP-005D` | landed | Define capability surface: `uses osvm`, `uses atomic`, `uses rawbuf` inventory. | 1 commit |

### First Executable Slices

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-006` | landed | Select first near-transcription executable slice. | 1 commit |
| `MIMAP-007` | landed | Size-class / bin map executable pilot. | 2-3 commits |
| `MIMAP-008` | landed | Page/free-list model pilot with direct executable proof and guard. | 1 commit |
| `MIMAP-009` | landed | Decommit/recommit/reuse lifecycle integration pilot. | 1 commit |
| `MIMAP-010` | landed | Page queue lifecycle selection pilot that skips decommitted pages and selects reusable pages explicitly. | 1 commit |
| `MIMAP-B001` | landed | Backend acceptance policy: VM scalar reference, LLVM/EXE MIMAP-011+ primary, VM timeout required. | 1 commit |
| `MIMAP-011` | landed | Allocator facade lifecycle route pilot using lifecycle-aware page selection; LLVM/EXE primary. | 1 commit |
| `MIMAP-012` | landed | Object-backed lifecycle queue LLVM route pilot; `ArrayBox` retains page objects and returns selected page. | 1 commit |
| `MIMAP-013` | ready | Facade composition over object-backed lifecycle queue. | after MIMAP-012 |

### Collection / Automata Sidecar Rows

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `COLL-001` | ready | Map/Set/HashMap naming and placement docs. | sidecar; not blocking MIMAP-011 |
| `COLL-002` | parked | Set semantic wrapper over Map. | after MIMAP-011 unless Set becomes the blocker |
| `COLL-003` | parked | Set proof app and guard. | after COLL-002 |
| `AUTO-001` | ready | FST placement SSOT. | sidecar; not mimalloc prerequisite |
| `AUTO-002` | parked | FST record vocabulary. | after evidence |
| `AUTO-003` | parked | Compiler keyword-table FST pilot. | compiler evidence only |

## Readiness Checklist

- [x] `.external/` is ignored before upstream source is cloned.
- [x] Upstream pin doc records commit/tag and license.
- [x] Source concepts are classified as `near-transcription`, `lifecycle-rewrite`, `substrate-gap`, `representation-gap`, or `deferred-unsafe`.
- [x] Blueprint uses Hakorune canonical surface only.
- [x] Provisional syntax is clearly marked and does not become implementation by accident.
- [ ] Executable slices have proof apps or guards.

## Stop Lines

```text
no vendored mimalloc source
no full port as the first row
no OSVM/provider/global allocator activation
no hooks / #[global_allocator]
no untracked design decision in implementation
```


## Active cleanup sidecar

| Row | Status | Scope | Notes |
| --- | --- | --- | --- |
| `CLEAN-WHILE-001` | landed | While deletion readiness inventory. | BoxShape cleanup; do not mix with MIMAP-012. |
| `CLEAN-WHILE-002` | landed | Delete `ASTNode::While` after inventory. | Parser `while` stays canonical Loop. |

## Remaining cleanup sidecar

| Row | Status | Scope | Notes |
| --- | --- | --- | --- |
| `CLEAN-FOR-001` | landed | Decide legacy `parse_for_range_stage3` fate. | Legacy `for` quarantined; canonical source is `loop i in`. |
| `CLEAN-DEAD-001` | landed | Audit first `#[allow(dead_code)]` cluster. | `numeric_substrate` and `type_registry` classified as intentional staging. |
