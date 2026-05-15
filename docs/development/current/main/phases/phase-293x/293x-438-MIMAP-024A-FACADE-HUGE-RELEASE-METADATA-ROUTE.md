# 293x-438 MIMAP-024A Facade Huge-Release Metadata Route

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-024A` is the selected post-MIMAP-023A allocator behavior row.

It adds one narrow facade-facing route for huge-handle lifetime:

```text
huge request allocation through MIMAP-023A
release that same live huge pointer through HakoAllocHugePageModel.markReleased
publish scalar proof fields
```

This row is not the full M181 huge release seam. M181 includes page-map lookup
and unregister behavior; MIMAP-024A intentionally stops before that boundary so
the facade lifetime step stays small and testable.

## Scope

- Add a facade huge-release metadata owner:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_box.hako`.
- Reuse the MIMAP-023A facade huge-page model route for allocation.
- Reuse `HakoAllocHugePageModel.markReleased(ptr)` for metadata release.
- Publish scalar report fields that prove:
  - the huge request routed through the MIMAP-023A route
  - the selected pointer/page metadata came from the huge model
  - live count transitions from 1 to 0 for the released handle
  - model/facade release counters advance once
  - non-huge requests still forward through the existing small fallback path
- Add an EXE/VM proof app:
  `apps/mimalloc-facade-huge-release-proof/main.hako`.
- Add a guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh`.

## Stop Lines

- Do not adopt `HakoAllocHugeReleaseSeam` as the facade owner in this row.
- Do not call `HakoAllocPageMap.lookup` or `HakoAllocPageMap.unregister`.
- Do not add OSVM release, unreserve, decommit, purge, or reclaim behavior.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add double-release / stale-pointer facade fail-fast behavior.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `024A.1` | Add the facade huge-release metadata owner. | One `.hako` owner reuses the MIMAP-023A route and existing huge model metadata release. | no page-map unregister / OS release |
| `024A.2` | Add the proof app and guard. | Guard proves huge allocate+release metadata fields and small forwarding. | no double-release fail-fast |
| `024A.3` | Close docs and current pointers. | Current state points to the next row selected after 024A. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

Owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_box.hako
```

Proof app:

```text
apps/mimalloc-facade-huge-release-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_mimalloc_facade_huge_release_exe_guard.sh
```

The implementation keeps the behavior to one durable slice: a huge request is
allocated through the MIMAP-023A facade route, then that same live huge pointer
is retired through `HakoAllocHugePageModel.markReleased(ptr)`. The page-map
entry remains registered in this row; page-map unregister / OS page return stay
outside MIMAP-024A.

## Closeout

MIMAP-024A is closed. The active blocker moves to MIMAP-024B
post-huge-release row selection.
