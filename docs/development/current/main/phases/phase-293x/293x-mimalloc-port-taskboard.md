---
Status: Active
Date: 2026-05-14
Lane: phase-293x mimalloc blueprint / port preparation
Canonical SSOT:
  - docs/development/current/main/design/mimalloc-hakorune-blueprint-task-breakdown-ssot.md
---

# Phase 293x mimalloc Port Taskboard

## Current Status

This board is parked until the current language-minimal blocker set reaches a
safe handoff point.

Preferred handoff point:

```text
LOOP-003C/D complete
PACKED-003/004 complete
```

Blueprint and inventory rows are now the active lane entry. Current row: `MIMAP-001 upstream source pin`.

## Active Source Policy

Upstream mimalloc source is local-only:

```text
.external/upstream/mimalloc/
```

Tracked output is docs only.

## Rows

### Source and Inventory

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-001` | active | Upstream source pin: URL, commit/tag, license, inspected files, local path. | 1 commit |
| `MIMAP-002` | ready | Source concept inventory: segment/page/block/heap/free-list/size-class/os/stats. | 1-2 commits |
| `MIMAP-003` | ready | Lifecycle rewrite blueprint: enum states, transitions, guard points. | 1-2 commits |
| `MIMAP-004` | ready | Substrate/representation gap ledger from source evidence. | 1-2 commits |

### Hakorune Blueprint

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-005A` | parked | Define brand/type vocabulary: `Bytes`, `PageId`, `BlockId`, `SegmentId`, `Generation`. | 1 commit |
| `MIMAP-005B` | parked | Define record vocabulary for page/block refs, size-class entries, stats snapshots. | 1 commit |
| `MIMAP-005C` | parked | Define enum/transition lifecycle blueprint for page/segment state. | 1 commit |
| `MIMAP-005D` | parked | Define capability surface: `uses osvm`, `uses atomic`, `uses rawbuf` inventory. | 1 commit |

### First Executable Slices

| Row | Status | Purpose | Expected size |
| --- | --- | --- | --- |
| `MIMAP-006` | blocked by inventory | Select first near-transcription executable slice. | 1 commit |
| `MIMAP-007` | blocked by MIMAP-006 | Size-class / bin map executable pilot. | 2-3 commits |
| `MIMAP-008` | blocked by MIMAP-006 | Page/free-list model pilot with explicit lifecycle state. | 2-4 commits |
| `MIMAP-009` | blocked by MIMAP-008 | Decommit/recommit/reuse lifecycle integration pilot. | 2-4 commits |

## Readiness Checklist

- [ ] `.external/` is ignored before upstream source is cloned.
- [ ] Upstream pin doc records commit/tag and license.
- [ ] Source concepts are classified as `near-transcription`, `lifecycle-rewrite`, `substrate-gap`, `representation-gap`, or `deferred-unsafe`.
- [ ] Blueprint uses Hakorune canonical surface only.
- [ ] Provisional syntax is clearly marked and does not become implementation by accident.
- [ ] Executable slices have proof apps or guards.

## Stop Lines

```text
no vendored mimalloc source
no full port as the first row
no OSVM/provider/global allocator activation
no hooks / #[global_allocator]
no untracked design decision in implementation
```

