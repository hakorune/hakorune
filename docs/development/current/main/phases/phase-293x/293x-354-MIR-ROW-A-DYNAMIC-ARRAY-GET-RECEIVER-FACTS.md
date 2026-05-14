# 293x-354 MIR-ROW-A Dynamic Array Get Receiver Facts

Status: landed
Date: 2026-05-15

## Decision

`MIR-ROW-A-FIX` keeps dynamic object-loop acceptance in MIR-owned semantic
metadata. Local `ArrayBox` / `MapBox` write observations publish collection
element origin facts, and `ArrayBox.get(i)` can recover the returned user-box
origin for later receiver method routing.

## Scope

- Keep source `birth` direct calls forbidden; construction remains `new`.
- Keep the proof shape minimal: dynamic `pages.get(i)`, guarded scalar
  selection, and `page.freeCount()` only.
- Do not reintroduce helper calls, nullable selected-object fields, dense proof
  reads, OSVM, provider activation, hooks, or host allocator replacement.
- Keep VM diagnostic-only for this object-heavy family; LLVM/EXE is the
  acceptance backend.

## Acceptance

- Proof app path: `apps/mimalloc-object-loop-row-a-proof/main.hako`.
- Guard path:
  `tools/checks/k2_wide_mimap012_object_loop_row_a_exe_guard.sh`.
- Guard output:
  - `[mimap012-row-a-mir-json] ok`
  - `[k2-wide-mimap012-object-loop-row-a-exe] ok`
- Expected EXE proof output includes:
  - `selected=1`
  - `selected_id=20`
  - `shape=4`
  - `summary=ok`

## Follow-up

Primary next row remains `MIMAP-013` facade composition over the object-backed
lifecycle queue. If the dynamic object-loop acceptance sidecar continues first,
the next row is `MIR-ROW-B`: add `considerPage(page)` helper-call flow while
keeping selected state scalar.
