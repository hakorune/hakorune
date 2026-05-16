# 293x-444 MIMAP-027A Facade Huge-Unregister Failfast Route

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-027A` is the selected post-MIMAP-026B allocator behavior row.

It adds one narrow facade-facing diagnostics route:

```text
MIMAP-026A allocate + M181 unregister one huge pointer
second release of that same pointer -> M181 lookup-miss reject
stale/unknown huge pointer -> M181 lookup-miss reject
scalar diagnostics only
```

This row is not an OS page return row. It proves the post-unregister reject
contract through the existing M181 seam and keeps OSVM release/unreserve,
decommit, provider activation, and allocator replacement closed.

## Scope

- Add a facade huge-unregister fail-fast owner:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unregister_failfast_box.hako`.
- Reuse the MIMAP-026A facade huge-unregister route for the first live release.
- Reuse the existing M181 `HakoAllocHugeReleaseSeam` for double-unregister and
  stale-pointer rejection.
- Publish scalar report fields that prove:
  - first huge unregister succeeds through MIMAP-026A
  - second release of the same pointer rejects through M181
  - stale/unknown huge pointer rejects through M181
  - page-map live count remains zero after rejects
  - lookup-miss and reject counters advance
- Add an EXE/VM proof app:
  `apps/mimalloc-facade-huge-unregister-failfast-proof/main.hako`.
- Add a guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_unregister_failfast_exe_guard.sh`.

## Stop Lines

- Do not add OSVM release, unreserve, decommit, purge, or reclaim behavior.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not call `HakoAllocPageMap.lookup`, `HakoAllocPageMap.unregister`, or
  `HakoAllocHugePageModel.markReleased(ptr)` directly from the facade
  diagnostics owner.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `027A.1` | Add the facade huge-unregister fail-fast owner. | One `.hako` owner composes MIMAP-026A and rejects double/stale releases through M181. | no direct page-map/model release calls |
| `027A.2` | Add the proof app and guard. | Guard proves lookup-miss/reject counters after page-map unregister. | no OS page return |
| `027A.3` | Close docs and current pointers. | Current state points to the next row selected after 027A. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_unregister_failfast_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
