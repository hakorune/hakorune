# 293x-440 MIMAP-025A Facade Huge-Release Fail-Fast Route

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-025A` is the selected post-MIMAP-024A allocator behavior row.

It adds one narrow facade-facing diagnostics route:

```text
MIMAP-024A allocate+release one huge pointer
second release of that same pointer -> reject
stale/unknown huge pointer -> reject
scalar diagnostics only
```

This row is not the M181 page-map unregister seam. The rejected pointers are
classified through the existing huge-page model metadata release path, and the
page map remains untouched.

## Scope

- Add a facade huge-release fail-fast owner:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_failfast_box.hako`.
- Reuse the MIMAP-024A facade huge-release metadata route for the first live
  release.
- Reuse `HakoAllocHugePageModel.markReleased(ptr)` for double-release and
  stale-pointer rejection.
- Publish scalar report fields that prove:
  - first huge allocate+release succeeds through MIMAP-024A
  - second release of the same huge pointer rejects
  - stale/unknown huge pointer rejects
  - live count remains stable after the rejects
  - release-reject counters advance
  - page-map entry/live/register counters are unchanged by the rejects
- Add an EXE/VM proof app:
  `apps/mimalloc-facade-huge-release-failfast-proof/main.hako`.
- Add a guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh`.

## Stop Lines

- Do not adopt `HakoAllocHugeReleaseSeam`.
- Do not call `HakoAllocPageMap.lookup` or `HakoAllocPageMap.unregister`.
- Do not add OSVM release, unreserve, decommit, purge, or reclaim behavior.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `025A.1` | Add the facade huge-release fail-fast owner. | One `.hako` owner composes the MIMAP-024A route and rejects double/stale releases. | no page-map unregister / OS release |
| `025A.2` | Add the proof app and guard. | Guard proves double-release and stale-pointer rejects through scalar fields. | no wider release behavior |
| `025A.3` | Close docs and current pointers. | Current state points to the next row selected after 025A. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

Owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_release_failfast_box.hako
```

Proof app:

```text
apps/mimalloc-facade-huge-release-failfast-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_mimalloc_facade_huge_release_failfast_exe_guard.sh
```

The implementation keeps the behavior to one durable slice: compose the
MIMAP-024A first release, then prove that a second release of the same huge
pointer and one stale/unknown huge pointer are rejected through the M180
metadata release seam. Page-map unregister / OS page return stay outside
MIMAP-025A.

## Closeout

MIMAP-025A is closed. The active blocker moves to MIMAP-025B
post-huge-release-failfast row selection.
