# 293x-442 MIMAP-026A Facade Huge-Unregister Route

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-026A` is the selected post-MIMAP-025A allocator behavior row.

It adds one narrow facade-facing success route:

```text
huge request allocation through MIMAP-023A
release the selected live huge pointer through M181 HakoAllocHugeReleaseSeam
metadata release + page-map unregister
```

This row is not an OS page return row. It may unregister page-map ownership for
the selected huge pointer, but it must stop before OSVM release/unreserve or
decommit.

## Scope

- Add a facade huge-unregister owner:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_box.hako`.
- Reuse the MIMAP-023A facade huge-page model route for allocation.
- Reuse the existing M181 `HakoAllocHugeReleaseSeam` for success-path release
  and page-map unregister.
- Publish scalar report fields that prove:
  - huge allocation succeeds through the MIMAP-023A route
  - M181 release succeeds for the selected pointer
  - huge-model live count transitions from 1 to 0
  - page-map live count transitions from 1 to 0
  - page-map entry/register counts remain stable
  - no OS release path is touched
- Add an EXE/VM proof app:
  `apps/mimalloc-facade-huge-unregister-proof/main.hako`.
- Add a guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh`.

## Stop Lines

- Do not add OSVM release, unreserve, decommit, purge, or reclaim behavior.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add broader huge release reject diagnostics in this row.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `026A.1` | Add the facade huge-unregister owner. | One `.hako` owner composes MIMAP-023A allocation with M181 success release/unregister. | no OS page return |
| `026A.2` | Add the proof app and guard. | Guard proves huge-model and page-map live-count transitions. | no reject diagnostics widening |
| `026A.3` | Close docs and current pointers. | Current state points to the next row selected after 026A. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_unregister_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
