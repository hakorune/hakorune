# 293x-441 MIMAP-025B Post-Huge-Release-Failfast Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-025B` is a planning-only row. It selects exactly one next allocator
behavior row:

```text
MIMAP-026A facade huge-release page-map unregister route
```

It selects the next allocator behavior row after the facade huge-release path
has both:

```text
MIMAP-024A first live huge metadata release
MIMAP-025A double-release / stale-pointer fail-fast diagnostics
```

The selected next durable slice adopts the existing M181 huge release seam from
the facade side for the success path only:

```text
facade huge allocation
M181 HakoAllocHugeReleaseSeam.releaseHugePtr(ptr)
metadata release + page-map unregister
```

OS page return, unreserve/decommit, and provider activation remain later.

## Scope

- Review the post-MIMAP-025A huge-release fail-fast path.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Selected Next Row

```text
row:
  MIMAP-026A facade huge-release page-map unregister route

owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_box.hako

reused owners:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
  lang/src/hako_alloc/memory/huge_release_seam_box.hako
  lang/src/hako_alloc/memory/huge_page_model_box.hako
  lang/src/hako_alloc/memory/page_map_box.hako

proof app:
  apps/mimalloc-facade-huge-unregister-proof/main.hako

guard:
  tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh
```

MIMAP-026A should allocate one huge handle through the MIMAP-023A facade route,
then release that same live huge pointer through the existing M181
`HakoAllocHugeReleaseSeam`. The proof should show huge-model live count and
page-map live count both transition from 1 to 0, while page-map entry count and
register count remain stable.

## Stop Lines

- Do not implement allocator behavior in this row.
- Do not add page-map lookup/unregister, OSVM release/unreserve/decommit,
  small release/free, realloc, alignment, purge/reclaim, remote-free, TLS,
  atomic, provider hook, host allocator replacement, or backend `.inc` matcher
  shortcut.
- Do not widen MIMAP-025A while selecting the next row.

MIMAP-026A stop lines:

- Do not add OSVM release/unreserve/decommit, purge/reclaim, provider hooks,
  host allocator replacement, or `#[global_allocator]`.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  or backend `.inc` matcher shortcuts.
- Do not add huge release reject diagnostics beyond the existing M181 success
  path in this row.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

MIMAP-025B is closed as a docs-only row selection. The active blocker moves to
MIMAP-026A.
