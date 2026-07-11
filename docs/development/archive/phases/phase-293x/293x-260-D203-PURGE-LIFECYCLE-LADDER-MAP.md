# 293x-260 D203 Purge Lifecycle Ladder Map

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

Add a thin navigation map before M213 so future agents do not bypass the
M192-M212 purge/lifecycle seams.

This is a docs-only structure card.
It does not consume the current blocker:

```text
M213 abandoned/reclaim inventory
```

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/purge-lifecycle-ladder-map-ssot.md` | Navigation map for M192-M213 purge, recommit, lifecycle, scheduler, and reclaim inventory rows. |

## Fixed decisions

M213 remains inventory-only unless a later card explicitly splits out execution.

Stop lines:

```text
no thread scheduling
no atomics expansion
no reclaim execution
no page-source calls
no unreserve
no OS release
no provider activation
no hooks
no process allocator replacement
```

## Closeout note

After M213 lands, prefer a docs-only closeout row before opening any reclaim
execution path.

