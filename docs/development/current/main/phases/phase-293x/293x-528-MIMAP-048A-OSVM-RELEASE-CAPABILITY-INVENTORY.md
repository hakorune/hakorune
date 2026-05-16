# 293x-528 MIMAP-048A OSVM Release Capability Inventory

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-048A` is the row selected by `MIMAP-047B`.

It is an inventory / proposal row for OSVM release vocabulary after the
OSVM-backed fast-path unreserve surface closed through success and fail-fast
diagnostics.

This row must not implement release behavior. It only fixes the boundary
between the landed unreserve surface and any future release route.

## Scope

- Inventory the landed unreserve surface:
  `OsVmCoreBox.unreserve_bytes_*`, `HakoAllocPageSourcePolicy.unreservePage`,
  and `HakoAllocOsVmFastPathUnreserveRoute`.
- Record that OS release remains inactive:
  `hako_osvm_release*`, `release_bytes*`, and `releasePage`.
- Add a focused guard that fails if release symbols or route behavior leak into
  active allocator/runtime/backend code.
- Select the next single row after the inventory closes.

## Non-Goals

- Do not add `hako_osvm_release*`.
- Do not add `release_bytes_*` or `releasePage`.
- Do not call OS release from the fast-path route, page-source owner, or proof
  apps.
- Do not change unreserve/recommit/reuse behavior.
- Do not add provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.
- Do not add backend `.inc` app/box-name matchers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `048A.1` | Write release capability inventory SSOT. | release/unreserve boundary is explicit. | no behavior |
| `048A.2` | Add release inventory guard. | release symbols remain absent from active routes. | no extern route |
| `048A.3` | Update taskboard/current pointers. | current pointer guard passes. | no multi-row bundle |
| `048A.4` | Select follow-up planning row. | one next blocker token is named. | no release implementation |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_osvm_release_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Return Condition

This row closes when the release capability boundary is documented and guarded
without opening release behavior.

## Inventory Result

`MIMAP-048A` freezes this boundary:

```text
landed unreserve surface:
  OsVmCoreBox.unreserve_bytes_i64 / unreserve_bytes_usize
  HakoAllocPageSourcePolicy.unreservePage
  HakoAllocPageSourceUnreserveAdapter.unreservePage
  HakoAllocOsVmFastPathUnreserveRoute.unreservePurgedPage
  HakoAllocOsVmFastPathUnreserveFailFastRoute diagnostics

inactive release surface:
  hako_osvm_release*
  release_bytes_i64 / release_bytes_usize
  releasePage
  fast-path release route
```

The row also clarifies active owner comments so unreserve owners do not describe
themselves as release rows.

## Evidence

```text
bash tools/checks/k2_wide_mimalloc_osvm_release_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-048A` selects `MIMAP-048B`.

```text
row:
  MIMAP-048B post-release-inventory row selection
classification:
  planning-only row
why now:
  release remains inactive after MIMAP-048A; the lane needs a single-row
  selection step before any allocator/compiler/language implementation resumes.
stop lines:
  no release implementation
  no provider activation
  no host allocator replacement
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-048B.
```
