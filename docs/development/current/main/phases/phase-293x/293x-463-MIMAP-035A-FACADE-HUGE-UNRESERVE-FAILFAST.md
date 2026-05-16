# 293x-463 MIMAP-035A Facade Huge Unreserve Fail-Fast

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-035A` is the behavior row selected by `MIMAP-034B`.

It adds facade-level duplicate/stale huge unreserve diagnostics by composing the
landed MIMAP-034A success route:

```text
HakoAllocObjectLifecycleFacadeHugeUnreserveRoute
  -> HakoAllocPageSourceUnreserveAdapter
```

This row records the first successful unreserved backing range in
allocator-side state and rejects duplicate/stale unreserve attempts before a
second page-source unreserve adapter call.

## Scope

- Add `HakoAllocObjectLifecycleFacadeHugeUnreserveFailfastRoute`.
- Add a scalar report capsule for first success, duplicate reject, stale reject,
  route counters, and stop-line sentinels.
- Reuse `HakoAllocObjectLifecycleFacadeHugeUnreserveRoute` for the first success
  path.
- Reject duplicate/stale unreserve attempts by consulting the fail-fast owner's
  recorded unreserved backing ranges; do not call the adapter on reject paths.
- Add proof app:
  `apps/mimalloc-facade-huge-unreserve-failfast-proof/main.hako`.
- Add focused EXE guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh`.

## Stop Lines

- Do not call `HakoAllocPageSourcePolicy` or `OsVmCoreBox` directly from the
  fail-fast owner; route the first success through MIMAP-034A.
- Do not add recommit, purge scheduler, remote-free, TLS cache, provider
  activation, hooks, host allocator replacement, or `#[global_allocator]`.
- Do not change page-source / OSVM substrate semantics.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `035A.1` | Add fail-fast route/report owner. | Duplicate/stale reject paths observe local unreserved range state. | no direct page-source/OSVM |
| `035A.2` | Add proof app. | Proof shows one adapter call for first success and no extra adapter calls for duplicate/stale reject. | no recommit/provider |
| `035A.3` | Add guard. | Guard checks owner delegation, MIR route metadata, EXE output, and no `.inc` matcher leak. | no backend shortcut |
| `035A.4` | Close current pointers. | Current state moves to the next selected row. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_unreserve_failfast_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when facade huge unreserve duplicate/stale diagnostics are live
and proven through existing owner seams, while recommit, provider activation,
and host allocator replacement remain inactive.
