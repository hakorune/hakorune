# 293x-454 MIMAP-030A Facade Huge Decommit Fail-Fast Diagnostics

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-030A` is the behavior row selected by `MIMAP-029B`.

It adds a narrow facade-facing fail-fast diagnostics owner for duplicate/stale
huge decommit after a successful MIMAP-029A same-backed unregister + decommit.

The row must prove:

```text
first path:
  MIMAP-029A succeeds
  same backed huge allocation is unregistered
  same backing range is decommitted once

second path:
  duplicate/stale decommit attempt is rejected by allocator-side state
  no second page-source decommit adapter call occurs
```

This is not an OSVM unreserve, recommit, provider activation, hook, or host
allocator replacement row.

## Scope

- Add:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_failfast_box.hako`.
- Reuse MIMAP-029A:
  `HakoAllocObjectLifecycleFacadeHugeDecommitRoute`.
- Reuse MIMAP-028A / M181 / M196 through the MIMAP-029A owner; do not bypass
  the existing success route.
- Add allocator-side duplicate/stale decommit state in the new owner.
- Reject a duplicate/stale decommit attempt before calling
  `HakoAllocPageSourceDecommitAdapter.decommitPage` again.
- Add proof app:
  `apps/mimalloc-facade-huge-decommit-failfast-proof/main.hako`.
- Add guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh`.

## Required Scalar Proof

The proof app should publish a compact scalar report that shows:

```text
success_status == 1
success_decommit_ok == 1
duplicate_attempted == 1
duplicate_rejected == 1
adapter_calls_before_duplicate == adapter_calls_after_duplicate
no_unreserve == 1
no_recommit == 1
no_provider == 1
```

The exact field names may follow local owner style, but the guard must prove
the same facts.

## Stop Lines

- Do not add OSVM unreserve, OS release, recommit, purge, or reclaim behavior.
- Do not implement general page lifecycle verifier changes in this row.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.
- Do not rely on OSVM/page-source decommit itself to reject duplicates.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `030A.1` | Add the facade huge decommit fail-fast owner. | Owner composes MIMAP-029A success and records allocator-side decommit state. | no unreserve/recommit |
| `030A.2` | Add duplicate/stale decommit rejection path. | Duplicate attempt rejects before a second adapter call. | no page-source fallback |
| `030A.3` | Add proof app and guard. | EXE guard proves scalar report and stop lines. | no backend matcher |
| `030A.4` | Close docs and current pointers. | Current state moves to post-failfast row selection. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Return Condition

This row closes when duplicate/stale huge decommit is rejected by allocator-side
state before a second page-source decommit call, with provider/host allocator
replacement still inactive.
