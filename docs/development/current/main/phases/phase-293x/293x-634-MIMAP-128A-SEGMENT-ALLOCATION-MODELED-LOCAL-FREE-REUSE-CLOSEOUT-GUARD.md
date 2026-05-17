# 293x-634 MIMAP-128A Segment Allocation Modeled Local-Free Reuse Closeout Guard

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-128A` is a closeout guard row for `MIMAP-126A`.

It freezes the modeled local-free reuse owner, proof app, guard entry, module
export, memory README owner note, current planning handoff, and inactive
stop-line set.

## Scope

Closeout guard:

```text
tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh
```

Implementation body:

```text
tools/checks/impl/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh
```

The guard may be manifest-backed through `tools/checks/guard_rows.toml`.

## Must Check

- `MIMAP-126A` card is landed.
- `MIMAP-127A` row-selection card is landed.
- `MIMAP-126A` proof app remains in `proof_apps.toml`.
- `MIMAP-126A` owner remains exported from `hako_module.toml`.
- Memory README names the `MIMAP-126A` owner.
- Check-script index lists the route guard and closeout guard.
- Owner still composes `integrateLocalFree` and `page.acquire`.
- Owner still observes `local_free_collect_count`.
- Stop-line leak checks remain closed for raw pointers, segment maps, arena,
  atomics, page-source/OSVM, scheduling, providers, host replacement, and
  backend `.inc` matchers.

## Stop Lines

- No allocator behavior.
- No compiler behavior.
- No source syntax change.
- No broad guard bundle.
- No dev gate / allocator-wide gate growth.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/run_row_guard.sh --only hako-alloc-segment-allocation-modeled-local-free-reuse-closeout
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_local_free_reuse_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`MIMAP-128A` added the local-free reuse closeout SSOT, manifest-backed closeout
guard, thin wrapper, and check-script index entry.

It selects `MIMAP-129A post-local-free-reuse-closeout row selection`.
