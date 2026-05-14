# 293x-342 MIMAP-009 Lifecycle Integration Pilot

Status: landed.
Decision: accepted.

## Goal

Connect the page/free-list pilot to explicit decommit, recommit, and reuse state
transitions while keeping OSVM and provider activation inactive.

## Landing shape

`HakoAllocPageModel` now owns page-local lifecycle state:

- `decommitted`
- `decommit_count`
- `recommit_count`
- `reuse_count`
- `lifecycle_reject_count`

The landed methods are:

- `decommit()`
- `recommit()`
- `canReuse()`
- `reuse()`

`reactivate()` now rejects decommitted pages directly.

## Guard

```bash
bash tools/checks/k2_wide_mimalloc_lifecycle_integration_pilot_guard.sh
```

## Explicit non-goals

- no OSVM
- no segment source
- no provider activation
- no allocator hooks
- no host allocator replacement

Next selected row: `MIMAP-010 page queue lifecycle selection pilot`.
