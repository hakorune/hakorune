# 293x-437 MIMAP-023B Post-Huge-Page-Model Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-023B` is a planning-only row. It selects exactly one next allocator
behavior row:

```text
MIMAP-024A facade huge-release metadata route
```

MIMAP-023A proves that the facade huge-request path can allocate through the
existing M180 huge-page model:

```text
MIMAP-022B huge request fail-fast
MIMAP-023A facade huge-page model route
```

The next narrow lifetime step is to release one live huge handle through the
existing huge-page model metadata owner. Existing M181
`HakoAllocHugeReleaseSeam` is intentionally not adopted as-is here because it
also owns page-map lookup/unregister behavior. MIMAP-024A keeps the facade row
smaller: metadata release only, no page-map unregister, no OS release, and no
provider / host allocator activation.

## Scope

- Review the post-MIMAP-023A facade huge-page path.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Selected Next Row

```text
row:
  MIMAP-024A facade huge-release metadata route

owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_box.hako

reused owners:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_page_model_box.hako
  lang/src/hako_alloc/memory/huge_page_model_box.hako
  lang/src/hako_alloc/memory/huge_page_meta_store_box.hako

proof app:
  apps/mimalloc-facade-huge-release-proof/main.hako

guard:
  tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh
```

MIMAP-024A should allocate one huge handle through the MIMAP-023A facade route,
then release that same live huge pointer through
`HakoAllocHugePageModel.markReleased(ptr)`. The proof should expose scalar
report fields for the selected pointer, page id, requested/committed sizes,
live-count transition, model release count, and facade release result.

## Stop Lines

- Do not implement allocator behavior in this row.
- Do not add huge release/unregister/unreserve/decommit behavior, page-map
  lookup route, provider hook, host allocator replacement, or backend `.inc`
  matcher shortcut.
- Do not widen release/realloc/alignment/purge/remote-free/TLS/atomic behavior.

MIMAP-024A stop lines:

- Do not use `HakoAllocHugeReleaseSeam` as the facade route owner yet; it is a
  wider M181 seam with page-map lookup/unregister behavior.
- Do not call `HakoAllocPageMap.lookup`, `HakoAllocPageMap.unregister`,
  OSVM release/unreserve/decommit, or provider hooks.
- Do not add small release/free, realloc, alignment, purge/reclaim,
  remote-free, TLS, atomic, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add double-release / stale-pointer facade fail-fast handling in
  MIMAP-024A; that is a separate row if it becomes the next blocker.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

MIMAP-023B is closed as a docs-only row selection. The active blocker moves to
MIMAP-024A.
