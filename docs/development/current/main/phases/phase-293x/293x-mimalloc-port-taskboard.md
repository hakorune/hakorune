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

Blueprint and inventory rows are now the active lane entry. Current primary row:
`MIMAP-013 facade composition over object-backed lifecycle queue`.

Latest sidecar closeout:

```text
MIR-ROW-A-FIX:
  landed
  dynamic ArrayBox.get(i) now recovers local collection element origin facts
  LLVM/EXE guard is green for pages.get(i) -> page.freeCount()
```

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

### Construction / Lifecycle Policy Rows

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `LIFECYCLE-BIRTH-001` | ready | Lock `birth` as a constructor hook fired only by `new`; direct receiver calls stay forbidden. | before parser widening |
| `PARSER-BIRTH-001` | ready | Add a negative parser fixture for `obj.birth(...)` so constructor policy does not regress. | after LIFECYCLE-BIRTH-001 |
| `PARSER-BIRTH-002` | ready | Improve direct-`birth` diagnostic with a `use new Box(...)` hint. | after PARSER-BIRTH-001 |
| `NEW-NAMED-ARGS-001` | parked | Design named constructor arguments for `new Box(field: value, ...)`. | later; not a MIMAP-013 blocker |
| `REUSE-LIFECYCLE-001` | ready | Keep reuse as explicit methods such as `reset`, `reactivate`, `configure`, `clear`, and `attach` with contracts/transitions. | sidecar with allocator lifecycle rows |

Policy SSOT:

```text
docs/development/current/main/design/constructor-birth-new-lifecycle-ssot.md
```

Decision for this board:

```text
Do not fix constructor failures by accepting source-level obj.birth(...).
Use new Box(...) for construction and explicit lifecycle methods for reuse.
```

### MIR Object-Loop Acceptance Follow-up Rows

| Row | Status | Purpose | Ordering |
| --- | --- | --- | --- |
| `MIR-INV-MIMAP012` | ready | Pin the MIMAP-012 heavy loop/object shape investigation and minimized hypotheses. | before broadening MIMAP-012 shape |
| `MIR-ROW-A` | landed | Minimal fixture for `loop + if guard + pages.get(i)` with scalar result only; MIR JSON and LLVM/EXE pass. | after MIR-INV-MIMAP012 |
| `MIR-ROW-B` | ready | Add `considerPage(page)` helper call while selected state remains scalar; prove both MIR JSON and LLVM/EXE acceptance. | after MIR-ROW-A |
| `MIR-ROW-C` | parked | Accept nullable object field selection, e.g. `me.last_selected_page = page`, and return it; prove both MIR JSON and LLVM/EXE acceptance. | after MIR-ROW-B |
| `MIR-ROW-D` | parked | Reintroduce dense queue field-read proof after object selection is green; prove both MIR JSON and LLVM/EXE acceptance. | after MIR-ROW-C |
| `MIR-ROW-A-FIX` | landed | Preserve or recover typed user-box receiver facts after dynamic `ArrayBox.get(i)` so `page.freeCount()` lowers as `HakoAllocPageModel.freeCount/0`, not `RuntimeDataBox.freeCount`. | before MIR-ROW-A closeout |

MIMAP-013 may proceed with the bounded-slot object queue from MIMAP-012. Do
not reintroduce dynamic scan, helper call, nullable object field selection, and
dense proof reads in one row.

Acceptance split for every `MIR-ROW-*`:

```text
MIR JSON:
  parser / Stage1 / planner / emit can produce JSON for the shape

LLVM/EXE:
  the emitted route compiles and executes with the expected proof output

VM:
  diagnostic smoke only; VM object-heavy timeout is not a MIMAP blocker
```

Guard policy:

```text
each implemented MIR-ROW-* must add a k2_wide_*.sh guard
the guard must fail if MIR JSON generation fails
the guard must fail if LLVM/EXE execution fails
the guard must not treat VM timeout as success for the EXE route
```

Current MIR-ROW-A evidence:

```text
tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh

MIR JSON:
  passes

LLVM/EXE:
  passes
  pages.get(i) result recovers HakoAllocPageModel origin
  page.freeCount() routes as a user-box method rather than RuntimeDataBox.freeCount

Guard:
  bash tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh
  [mimap012-row-a-mir-json] ok
  [k2-wide-mimap012-object-loop-row-a-exe] ok
```

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
no source-level receiver.birth(...) as lifecycle workaround
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
