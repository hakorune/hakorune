---
Status: Complete
Date: 2026-05-11
Scope: docs-only purpose realignment for the mimalloc port after M103.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/stage2-selfhost-and-hako-alloc-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - docs/development/current/main/design/allocator-provider-post-m101-implementation-ladder-ssot.md
---

# 293x-160 Mimalloc Hako Port Purpose Realignment

## Goal

Fix the durable reading of the current mimalloc lane before more implementation
work lands.

Mimalloc is being ported to improve Hakorune completeness by implementing the
allocator in `.hako` / `hako_alloc`. The allocator-provider activation ladder is
future optional host replacement support, not the default current
implementation path.

## Updated Contract

- The current mimalloc implementation target is `.hako` / `hako_alloc`.
- Hakorune core does not replace its process allocator in the current lane.
- M104 remains the next row only inside the optional allocator-provider ladder.
- The default next implementation work should return to `.hako` allocator
  slices and capability-backed proof apps.
- Existing provider activation stop lines remain active.

## Proof

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
