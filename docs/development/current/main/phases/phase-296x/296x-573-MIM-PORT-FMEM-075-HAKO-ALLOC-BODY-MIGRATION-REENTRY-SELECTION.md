---
Status: Done
Date: 2026-06-07
Scope: MIM-PORT-FMEM-075.
Related:
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - src/mir/fastmem_access_plan.rs
  - tools/hako_check/fastmem_check_smoke.sh
---

# 296x-573 MIM-PORT-FMEM-075 Hako Alloc Body Migration Reentry Selection

## Purpose

Return from the FastMemory BoxShape cleanup mini-series to the mimalloc port
mainline. Select the next `.hako` hako_alloc body slice that should use landed
FastMemory substrate, without opening a new lowering or product activation row
in the same commit.

## Required Boundaries

```text
selection/report-only row
no new MemOp kind
no new lowering
no report/check behavior change unless the selected row explicitly requires it
no product activation / hook / global allocator claim / winner claim changes
```

## Selection Questions

```text
Which landed substrate is ready to consume next?
Which hako_alloc body still has the highest C-bridge duplication pressure?
Can the next slice be one durable semantic row with fixture/gate coverage?
Which existing checks prove the selected boundary stays non-activating?
```

## Acceptance Sketch

```text
next body migration row is named
explicit non-goals and closed claims are recorded
required fixture/gate evidence for the next row is listed
current FastMemory smoke remains green
git diff --check passes
```

## Non-goals

```text
implementing the selected body slice
retiring the diagnostic Python-template C bridge
changing product activation or allocator claim behavior
```

## Decision

Select `MIM-PORT-FMEM-076` as the next implementation row:

```text
Page-local alloc route CFG preflight
```

The next `.hako hako_alloc` body migration should return to the allocation
route selected in MIM-PORT-FMEM-020/MIM-PORT-FMEM-021:

```text
if local_free is non-empty:
  local_free_alloc
else if free_head is non-empty:
  free_head_alloc
else:
  refill_then_free_head_alloc
```

## Why This Slice

The current FastMemory substrate has landed verified straight-line allocation
body pilots:

```text
local_free_alloc
free_head_alloc
refill_then_free_head_alloc
```

It has also landed branch-CFG producer evidence for the same/remote free body
path. The allocation route itself still remains a report-only surface:

```text
page_local_alloc_route_branch_claim=0
page_local_alloc_route_cfg_lowering_enabled=0
```

That makes page-local allocation route CFG the highest remaining C-bridge
duplication pressure on the hako_alloc body path: the three straight-line
allocation bodies are represented, but the route that chooses between them is
not yet a source-truth FastMemory body.

Rejected for this row:

```text
product activation / hook / global allocator / winner claim:
  already have producer evidence and should not be reopened while selecting a
  body migration row.

remote-owner free expansion:
  free-side route CFG evidence exists; allocation route CFG is the missing
  body selection surface.

multi-block refill:
  still broader than the single-route allocation wrapper and would mix policy
  transfer with branch route proof.
```

## Required Evidence For MIM-PORT-FMEM-076

```text
new source body:
  lang/src/hako_alloc/memory/page_meta_page_local_alloc_route_cfg_preflight_box.hako

report/check shape:
  replacement_front_selected_route=page_local_alloc_route_cfg_preflight
  replacement_front_selected_memop_family=page_local_alloc_route_cfg
  replacement_front_next_producer_slice=page_local_alloc_route_cfg_producer_pilot
  page_local_alloc_route_cfg_selected=1
  page_local_alloc_route_cfg_lowering_enabled=0
  page_local_alloc_route_branch_claim=0

existing substrate that must remain visible:
  page_local_alloc_route_candidate in
    local_free_alloc | free_head_alloc | refill_then_free_head_alloc
  fastmem_branch_cfg_open already proven on the landed branch-CFG producer path
```

## Boundaries For MIM-PORT-FMEM-076

```text
preflight/report-only row
no new MemOp kind
no new lowering
no path-sensitive allocation branch execution yet
no multi-block refill
no remote-owner free expansion
no TLS backing transfer change
no owner slot reuse / abandoned reclaim change
no product activation / hook / global allocator claim / winner claim change
```

## Verification

```bash
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/hako_check/fastmem_source_syntax_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
