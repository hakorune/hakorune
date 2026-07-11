# 293x-439 MIMAP-024B Post-Huge-Release Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-024B` is a planning-only row. It selects exactly one next allocator
behavior row:

```text
MIMAP-025A facade huge-release fail-fast diagnostics route
```

It selects the next allocator behavior row after the facade can allocate a
huge request through the MIMAP-023A route and retire that same live huge
pointer through the MIMAP-024A metadata release route:

```text
MIMAP-023A facade huge-page model allocation
MIMAP-024A facade huge-release metadata route
```

The selected next durable slice mirrors the already-landed small-release
sequence: after one known live release, prove double-release / stale-pointer
rejection before widening to page-map unregister. Page-map unregister and OS
page return stay later.

## Scope

- Review the post-MIMAP-024A huge-handle lifetime path.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Selected Next Row

```text
row:
  MIMAP-025A facade huge-release fail-fast diagnostics route

owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_failfast_box.hako

reused owners:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_box.hako
  lang/src/hako_alloc/memory/huge_page_model_box.hako
  lang/src/hako_alloc/memory/huge_page_meta_store_box.hako

proof app:
  apps/mimalloc-facade-huge-release-failfast-proof/main.hako

guard:
  tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh
```

MIMAP-025A should allocate and release one huge handle through the MIMAP-024A
route, then prove that a second release of that same pointer and a stale/unknown
huge pointer are rejected through `HakoAllocHugePageModel.markReleased(ptr)`
without changing page-map ownership.

## Stop Lines

- Do not implement allocator behavior in this row.
- Do not add page-map lookup/unregister, OSVM release/unreserve/decommit,
  double-release / stale-pointer fail-fast, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-024A beyond metadata release while selecting the next row.

MIMAP-025A stop lines:

- Do not adopt `HakoAllocHugeReleaseSeam` or call page-map lookup/unregister.
- Do not add OSVM release/unreserve/decommit, purge/reclaim, provider hooks,
  host allocator replacement, or `#[global_allocator]`.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  page-map unregister, or backend `.inc` matcher shortcuts.
- Do not turn double-release rejection into page-map unregister or OS page
  return.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

MIMAP-024B is closed as a docs-only row selection. The active blocker moves to
MIMAP-025A.
