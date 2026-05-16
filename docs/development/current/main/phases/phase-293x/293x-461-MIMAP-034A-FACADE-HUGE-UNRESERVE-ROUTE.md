# 293x-461 MIMAP-034A Facade Huge Unreserve Route

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-034A` is the behavior row selected by `MIMAP-033B`.

It adds the first facade-level huge unreserve-after-decommit success route by
composing existing owners:

```text
HakoAllocObjectLifecycleFacadeHugeDecommitRoute
  -> HakoAllocPageSourceUnreserveAdapter
  -> HakoAllocPageSourcePolicy.unreservePage
  -> OsVmCoreBox.unreserve_bytes_i64
```

This row proves the success route only. Duplicate/stale unreserve diagnostics
remain a later row.

## Scope

- Add `HakoAllocObjectLifecycleFacadeHugeUnreserveRoute`.
- Compose `HakoAllocObjectLifecycleFacadeHugeDecommitRoute` and
  `HakoAllocPageSourceUnreserveAdapter`.
- Record scalar report fields for backing identity, decommit status, unreserve
  attempted/ok/base/bytes/rc, adapter counters, and final status/reason.
- Add proof app:
  `apps/mimalloc-facade-huge-unreserve-proof/main.hako`.
- Add focused EXE guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_unreserve_exe_guard.sh`.

## Stop Lines

- Do not add duplicate/stale unreserve diagnostics in this row.
- Do not call `OsVmCoreBox` or `HakoAllocPageSourcePolicy` directly from the
  facade owner; use `HakoAllocPageSourceUnreserveAdapter`.
- Do not add recommit, purge scheduler, remote-free, TLS cache, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `034A.1` | Add facade huge unreserve owner. | New owner composes MIMAP-029A and MIMAP-033A only. | no direct page-source/OSVM |
| `034A.2` | Add proof app. | Proof shows same backing range is decommitted then unreserved once. | no duplicate/stale diagnostics |
| `034A.3` | Add guard. | Guard checks owner delegation, MIR route metadata, EXE output, and no `.inc` matcher leak. | no backend shortcut |
| `034A.4` | Close current pointers. | Current state moves to the next selected row. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_unreserve_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

```text
facade route owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_unreserve_box.hako
proof app:
  apps/mimalloc-facade-huge-unreserve-proof/main.hako
guard:
  tools/checks/k2_wide_mimalloc_facade_huge_unreserve_exe_guard.sh
```

The landed row adds `HakoAllocObjectLifecycleFacadeHugeUnreserveRoute`. It
composes `HakoAllocObjectLifecycleFacadeHugeDecommitRoute` and
`HakoAllocPageSourceUnreserveAdapter`, then unreserves the exact backing range
that MIMAP-029A decommitted. The facade owner does not call page-source or OSVM
APIs directly.

Focused proof output includes:

```text
mimalloc-facade-huge-unreserve-proof
decommit=1,1,<base>,<bytes>,0
unreserve=1,1,<same-base>,<same-bytes>,0
adapter=1,1,0
stop=1,1,1
summary=ok
```

## Return Condition

This row closes when facade huge unreserve-after-decommit success is live and
proven through existing owner seams, while duplicate/stale unreserve diagnostics
and provider/host allocator replacement remain inactive.

Closeout:

```text
current blocker moves to MIMAP-034B post-huge-unreserve row selection.
```
