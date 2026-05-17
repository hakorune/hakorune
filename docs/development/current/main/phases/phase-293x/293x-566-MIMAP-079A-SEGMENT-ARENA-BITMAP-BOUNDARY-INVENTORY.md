# 293x-566 MIMAP-079A Segment Arena Bitmap Boundary Inventory

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-079A` is the allocator inventory row selected by `MIMAP-078A`.

The row should add one scalar `.hako` owner that names the segment / arena /
bitmap boundary needed by later mimalloc rows. It should classify tiny
proof-only scalar facts while keeping raw pointer residence, atomic bitmap
claim, OSVM execution, provider activation, and host replacement closed.

## Scope

- Add a read-only scalar inventory owner for segment / arena / bitmap readiness.
- Report accepted scalar facts for a tiny segment/arena/mask shape.
- Report explicit blocked reasons for raw pointer, atomic bitmap, OSVM
  execution, provider, and invalid-shape requests.
- Add a proof app, manifest row, focused guard, and docs index entry.

## Stop Lines

- No allocator allocation/free behavior.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No raw pointer residence.
- No atomic bitmap execution.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `079A.1` | Add boundary SSOT and owner. | owner reports scalar ready/blocked rows only. | no bitmap execution |
| `079A.2` | Add proof app and manifest entry. | VM/MIR/EXE proof locks reason vocabulary. | no raw pointer / OSVM |
| `079A.3` | Add focused guard and docs index entry. | guard checks stop lines and proof output. | no provider / matcher |
| `079A.4` | Update current pointers. | current pointer guard passes. | no broad cleanup |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Implementation Result

`MIMAP-079A` added:

```text
HakoAllocSegmentArenaBitmapInventoryReport
HakoAllocSegmentArenaBitmapInventory.classifyBoundary(...)
apps/hako-alloc-segment-arena-bitmap-inventory-proof/
tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_guard.sh
docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md
```

The route accepts one tiny proof-only scalar segment/arena/mask shape and
rejects invalid, raw-pointer, atomic-bitmap, OSVM, and provider rows with
stable scalar reasons.

Proof output shape:

```text
ready=1,0,10,2,16,8,3,1
rejects=1,2,3,4,5
inactive=0,0,0,0,0,0
counts=6,1,5,1,1,1,1,1,14,5
summary=ok
```

Next row:

```text
MIMAP-080A segment arena bitmap inventory closeout guard
```
