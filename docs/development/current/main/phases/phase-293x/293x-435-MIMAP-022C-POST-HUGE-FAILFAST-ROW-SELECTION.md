# 293x-435 MIMAP-022C Post-Huge-Failfast Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-022C` is a planning-only row.

It selects the next allocator behavior row after the facade page-source path
has a huge-request fail-fast boundary:

```text
MIMAP-021B fresh page attach
MIMAP-021C allocation-miss fallback
MIMAP-022B huge request fail-fast
```

The selected next row is:

```text
MIMAP-023A facade huge-page model route
```

This keeps the allocator lane on the huge-request seam opened by MIMAP-022B.
The row should reuse the already-existing M180 huge-page model from the
object-lifecycle facade side, without reopening provider hooks, host allocator
replacement, or a broad page-map route.

It must not implement allocator behavior in this planning card.

## Selected Row

Next card:

```text
docs/development/current/main/phases/phase-293x/293x-436-MIMAP-023A-FACADE-HUGE-PAGE-MODEL-ROUTE.md
```

Planned owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
```

Existing collaborators:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_failfast_box.hako
lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako
lang/src/hako_alloc/memory/huge_page_model_box.hako
lang/src/hako_alloc/memory/page_map_box.hako
lang/src/hako_alloc/memory/size_class_box.hako
```

Proof and guard:

```text
apps/mimalloc-facade-huge-page-model-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_huge_page_model_exe_guard.sh
```

Selected scope:

- Classify request size through the existing size-class / MIMAP-022B route
  threshold.
- Route huge requests into the existing `HakoAllocHugePageModel` owner.
- Preserve MIMAP-022B / MIMAP-021C small-request forwarding.
- Expose scalar proof fields for huge allocation status, page id, pointer,
  requested size, committed size, small forwarding, and final route reason.

## Scope

- Review the post-MIMAP-022B facade/page-source path.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Stop Lines

- Do not implement allocator behavior in this row.
- Do not add a huge page model, page-map route, provider hook, host allocator
  replacement, or backend `.inc` matcher shortcut.
- Do not widen release/realloc/alignment/purge/remote-free/TLS/atomic behavior.
- Do not add huge release, unregister, unreserve, decommit, or page-map lookup
  behavior to the selected next row.

## Required Evidence

```text
bash tools/checks/k2_wide_mimap022c_next_row_selection_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

MIMAP-022C selected `MIMAP-023A facade huge-page model route` as the next
durable allocator behavior row. The next row is narrow: it should connect
facade huge requests to the existing M180 `HakoAllocHugePageModel` through a
facade-facing adapter and scalar proof app.

The selection deliberately keeps the following inactive:

```text
huge release / unregister / unreserve / decommit
page-map lookup route
provider hooks
host allocator replacement
#[global_allocator]
backend .inc matcher shortcuts
```
