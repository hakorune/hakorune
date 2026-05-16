# 293x-456 MIMAP-031A OSVM Unreserve Capability Inventory

Status: selected current
Date: 2026-05-16

## Decision

`MIMAP-031A` is the row selected by `MIMAP-030B`.

It is an inventory / planning row for OSVM unreserve capability after the
landed huge decommit success and duplicate/stale fail-fast diagnostics rows.
It must not implement unreserve, recommit, provider activation, hooks, host
allocator replacement, or `#[global_allocator]`.

## Scope

This row must inventory:

- existing OSVM substrate owners:
  `OsVmCoreBox`, `HakoAllocPageSourcePolicy`, and backend route metadata
- existing decommit owners:
  `HakoAllocPageSourceDecommitAdapter`,
  `HakoAllocObjectLifecycleFacadeHugeDecommitRoute`, and
  `HakoAllocObjectLifecycleFacadeHugeDecommitFailfastRoute`
- current stop lines around unreserve/release/recommit/provider activation
- whether the next implementable row should be:
  - an unreserve capability declaration / route inventory
  - a page-source unreserve adapter
  - a verifier/fail-fast contract row
  - or a park row until more substrate evidence exists

## Non-Goals

- Do not add an OSVM unreserve extern route.
- Do not call an OS unreserve/release primitive.
- Do not add recommit, purge, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matchers or app/box-name classifiers.
- Do not change MIMAP-029A / MIMAP-030A behavior.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `031A.1` | Inventory existing OSVM/page-source/decommit owners. | Current ownership and missing unreserve capability are documented. | no implementation |
| `031A.2` | Decide the next row granularity. | One next row has owner/proof/guard/stop lines. | no provider activation |
| `031A.3` | Add any guard needed for inventory no-growth. | Guard, if added, proves unreserve remains closed. | no new route |
| `031A.4` | Close current pointers. | Current state moves to the selected next row. | no unreserve execution |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

If a no-growth inventory guard is added, it must be listed here before closeout.

## Return Condition

This row closes when the unreserve capability boundary is documented and one
next row is selected without opening unreserve/recommit/provider behavior.
