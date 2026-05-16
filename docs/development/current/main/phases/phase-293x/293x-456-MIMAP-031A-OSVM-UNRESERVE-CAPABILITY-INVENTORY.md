# 293x-456 MIMAP-031A OSVM Unreserve Capability Inventory

Status: landed
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

Inventory guard:

```text
bash tools/checks/k2_wide_mimalloc_osvm_unreserve_inventory_guard.sh
```

## Return Condition

This row closes when the unreserve capability boundary is documented and one
next row is selected without opening unreserve/recommit/provider behavior.

## Inventory Result

Live OSVM route surface:

```text
OsVmCoreBox.page_size_i64()
OsVmCoreBox.reserve_bytes_i64(len_bytes)
OsVmCoreBox.commit_bytes_i64(base, len_bytes)
OsVmCoreBox.decommit_bytes_i64(base, len_bytes)
```

Live page-source owner surface:

```text
HakoAllocPageSourcePolicy.reservePage(bytes)
HakoAllocPageSourcePolicy.commitPage(base, bytes)
HakoAllocPageSourcePolicy.decommitPage(base, bytes)
```

Inactive surface verified by guard:

```text
hako_osvm_unreserve*
hako_osvm_release*
unreserve_bytes
release_bytes
unreservePage
releasePage
```

## Selection Result

`MIMAP-031A` selects `MIMAP-032A`.

```text
row:
  MIMAP-032A OSVM unreserve substrate route
owner:
  lang/src/runtime/substrate/osvm/osvm_core_box.hako
proof app:
  apps/mimalloc-osvm-unreserve-proof/main.hako
guard:
  tools/checks/k2_wide_mimalloc_osvm_unreserve_exe_guard.sh
reused owners:
  OsVmCoreBox
  HakoAllocPageSourcePolicy remains unchanged in MIMAP-032A
primary proof:
  add a substrate route for unreserve only; do not route allocator owners
stop lines:
  no page-source/facade owner adoption
  no recommit
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

Closeout:

```text
current blocker moves to MIMAP-032A.
```
