# 293x-913 CHECKS-DEV-GATE-ALLOCATOR-LIST-DEDUP Dev Gate Allocator List Dedup

Status: landed
Date: 2026-05-20

## Decision

Deduplicate the `allocator-wide` profile listing in `tools/checks/dev_gate.sh`
by delegating its detailed guard inventory to
`tools/checks/k2_wide_allocator_gate.sh --list`.

## Context

`allocator-wide` execution already calls the allocator gate group as a stable
public entry. After `293x-912`, that group owns the family inventories under
`tools/checks/allocator/families/*.steps`.

The remaining debt was list-only: `dev_gate.sh --list` still carried a large
copy of the allocator guard list. That duplicated the allocator gate inventory
and made future allocator guard additions easy to miss in one of the two
places.

## Scope

- Keep `tools/checks/dev_gate.sh allocator-wide` execution unchanged.
- Keep `tools/checks/k2_wide_allocator_gate.sh` as the allocator-wide guard
  inventory owner.
- Make `tools/checks/dev_gate.sh --list` call
  `tools/checks/k2_wide_allocator_gate.sh --list` for allocator details.
- Keep `tools/checks/k2_wide_metal_keep_inventory_guard.sh` visible as the
  extra allocator-wide tail guard owned by `dev_gate.sh`.
- Update the check scripts index to document the list ownership.

## Non-Goals

- Do not migrate allocator-wide to `manifest_runner`.
- Do not change which guards allocator-wide runs.
- Do not change validation profiles or row cadence.
- Do not run the heavy allocator-wide gate as part of this cleanup.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
bash -n tools/checks/dev_gate.sh
bash tools/checks/dev_gate.sh --list
bash tools/checks/k2_wide_allocator_gate.sh --list
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
