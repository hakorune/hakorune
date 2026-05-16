# 293x-548 MIMAP-061A Reclaim Scalar Lane Closeout Guard

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-061A` is the closeout guard row selected by `MIMAP-060A`.

The row should lock the landed scalar reclaim lane through:

```text
MIMAP-051A owner-transfer contract
MIMAP-054A atomic-claim contract
MIMAP-055A owner-transfer first execution route
MIMAP-056A remote-free drain contract
MIMAP-057A remote-free drain first execution route
MIMAP-058A post-drain owner-transfer integration route
MIMAP-060A reclaim completion marker route
```

It must not add new allocator behavior. It is a BoxShape/closeout row for
proof and docs synchronization before broader reclaim behavior is considered.

## Scope

- Add a scalar reclaim lane closeout SSOT.
- Add a focused guard that checks the landed reclaim proof app manifest rows
  and stop lines.
- Keep provider activation, thread scheduling, page-source calls, OSVM
  unreserve/release, and host allocator replacement closed.
- Select the next row after closeout.

## Stop Lines

- No new `.hako` allocator owner behavior.
- No page-source call.
- No OSVM unreserve / release.
- No thread scheduling.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `061A.1` | Write scalar reclaim lane closeout SSOT. | landed row set and inactive surfaces fixed. | no behavior |
| `061A.2` | Add closeout guard. | manifest, proof apps, and stop lines verified. | no provider |
| `061A.3` | Update docs index and current pointers. | current pointer guard passes. | no bundle |
| `061A.4` | Select follow-up row. | next row has one narrow owner. | no feature mixing |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-061A` added the scalar reclaim lane closeout SSOT and guard.

The guard locks the landed reclaim scalar row set from `MIMAP-051A` through
`MIMAP-060A`, including proof manifest entries, focused guard index entries,
owner exports, memory README owner descriptions, `.inc` no-growth, and inactive
page-source/OSVM/scheduling/provider replacement stop lines.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scalar_lane_closeout_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Selection Result

`MIMAP-061A` selects `MIMAP-062A`.

```text
row:
  MIMAP-062A post-reclaim-scalar-closeout row selection

classification:
  planning row

why now:
  scalar reclaim is closed through proof and guard. The next step should decide
  one narrow follow-up before opening broader reclaim, scheduler, page-source,
  compiler/language, or lane-switch work.

stop lines:
  no new allocator behavior
  no thread scheduling
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no cleanup bundle
```
