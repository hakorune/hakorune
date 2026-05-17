---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-080A segment / arena / bitmap inventory closeout guard.
Related:
  - docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-566-MIMAP-079A-SEGMENT-ARENA-BITMAP-BOUNDARY-INVENTORY.md
  - docs/development/current/main/phases/phase-293x/293x-567-MIMAP-080A-SEGMENT-ARENA-BITMAP-INVENTORY-CLOSEOUT-GUARD.md
  - tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh
---

# Hako Alloc Segment Arena Bitmap Inventory Closeout SSOT

## Decision

`MIMAP-080A` is a guard-only closeout for the scalar segment / arena /
bitmap boundary inventory added by `MIMAP-079A`.

It does not add allocator behavior. It locks the inventory owner, proof app,
proof manifest, module export, guard, and inactive stop lines before the lane
selects broader allocator behavior, real bitmap/OSVM substrate work, or
Hakorune language work.

## Locked Rows

| Row | Status | Locked surface |
| --- | --- | --- |
| `MIMAP-079A` | landed | segment / arena / bitmap boundary inventory owner, proof app, guard, manifest, module export, README entry |
| `MIMAP-080A` | landed by this closeout | local-run closeout guard and docs index entry |
| `MIMAP-081A` | selected next | post-segment-arena-bitmap-inventory row selection |

## Required Locked Files

```text
docs/development/current/main/design/hako-alloc-segment-arena-bitmap-inventory-ssot.md
lang/src/hako_alloc/memory/segment_arena_bitmap_inventory_box.hako
apps/hako-alloc-segment-arena-bitmap-inventory-proof/main.hako
apps/hako-alloc-segment-arena-bitmap-inventory-proof/test.sh
tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_guard.sh
tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh
tools/checks/proof_apps.toml
lang/src/hako_alloc/hako_module.toml
lang/src/hako_alloc/memory/README.md
docs/tools/check-scripts-index.md
```

## Inactive Stop Lines

The closeout guard must keep these inactive:

```text
allocation/free behavior
real thread scheduling
worker spawning
source-level concurrency semantics
raw pointer residence
atomic bitmap execution
page-source call
OSVM unreserve / release
provider activation
hooks
host allocator replacement
backend app/name matcher
```

## Next Row

```text
MIMAP-081A post-segment-arena-bitmap-inventory row selection
```

