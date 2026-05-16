# 293x-467 MIMAP-037A Facade Huge Backing-Set Helper

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-037A` is the BoxShape cleanup selected by `MIMAP-036B`.

It extracts the duplicate/stale unreserve backing-range set from
`HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute` into a small helper:

```text
HakoAllocObjectLifecycleFacadeHugeBackingSet
```

This row does not add allocator behavior. It keeps the existing MIMAP-035A
proof semantics: duplicate/stale unreserve attempts reject before a second
page-source unreserve adapter call.

## Scope

- Add `object_lifecycle_facade_huge_backing_set_box.hako`.
- Replace the route-local `unreserved_bases` / `unreserved_bytes` parallel
  arrays with the helper.
- Update the MIMAP-035A guard to validate the helper-owned field shape.
- Add a focused static guard for this cleanup.
- Keep provider activation, hooks, host allocator replacement, and
  `#[global_allocator]` inactive.

## Stop Lines

- Do not add allocator behavior.
- Do not add new fail-fast reason vocabulary.
- Do not add recommit, purge scheduler, remote-free, TLS cache, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `037A.1` | Add helper SSOT and helper box. | Helper owns backing pair set operations. | no allocator behavior |
| `037A.2` | Route fail-fast state through helper. | MIMAP-035A proof still passes. | no new diagnostics |
| `037A.3` | Add guard and docs entries. | Guard pins helper export and route no longer owns parallel arrays. | no backend matcher |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_backing_set_helper_guard.sh
bash tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

```text
helper:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_backing_set_box.hako
consumer:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_failfast_box.hako
guard:
  tools/checks/k2_wide_mimalloc_facade_huge_backing_set_helper_guard.sh
```

Closeout:

```text
current blocker moves to MIMAP-037B post-backing-set-helper row selection.
```
