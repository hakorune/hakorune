# 293x-912 CHECKS-ALLOCATOR-GATE-FAMILY-SPLIT Allocator Gate Family Inventory Split

Status: landed
Date: 2026-05-20

## Decision

Split the `allocator-wide` gate inventory into family step files while keeping
the stable public entry unchanged.

## Context

`tools/checks/allocator/k2_wide_allocator_gate.steps` had grown into one large
mixed list covering mimalloc substrate, hako_alloc production facade, allocator
hooks, and provider activation proofs. The execution model was already
centralized through `tools/checks/allocator/lib/guard_group.sh`, so this cleanup
only changes the inventory shape.

## Scope

- Keep `tools/checks/k2_wide_allocator_gate.sh` and
  `tools/checks/allocator/k2_wide_allocator_gate.sh` as stable public entries.
- Add `@include` support to `tools/checks/allocator/lib/guard_group.sh`.
- Split the allocator-wide steps into:
  - `tools/checks/allocator/families/mimalloc.steps`
  - `tools/checks/allocator/families/hako_alloc_production.steps`
  - `tools/checks/allocator/families/hooks.steps`
  - `tools/checks/allocator/families/provider.steps`
- Preserve exact script order and script-level `--list` output.

## Non-Goals

- Do not migrate allocator-wide to `manifest_runner`.
- Do not change which guards allocator-wide runs.
- Do not change validation levels or row profiles.
- Do not run the heavy allocator-wide gate as part of this cleanup.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
bash tools/checks/k2_wide_allocator_gate.sh --list
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
