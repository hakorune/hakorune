# 293x-547 MIMAP-060A Reclaim Completion Marker Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-060A` is the scalar reclaim completion route selected by `MIMAP-059A`.

The row may compose the MIMAP-058A post-drain owner-transfer integration route
and set an executor-local reclaim completion marker when that integration
succeeds. It must not call page-source APIs, release/unreserve OSVM memory,
schedule workers, activate providers, or replace the host allocator.

## Scope

- Add an SSOT for scalar reclaim completion marker semantics.
- Add a `.hako` completion owner that composes the post-drain owner-transfer
  route.
- Mark completion only when integration reports transfer success.
- Publish blocked reasons for drain/transfer failures.
- Add proof app and focused guard.
- Select the next row after the proof lands.

## Stop Lines

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
| `060A.1` | Write reclaim completion marker SSOT. | completion and blocked reason vocabulary fixed. | no page-source |
| `060A.2` | Add `.hako` completion owner. | success sets only executor-local completion marker. | no OS release |
| `060A.3` | Add proof app. | proof observes success and blocked cases. | no provider |
| `060A.4` | Add focused guard and docs index row. | VM / MIR JSON / pure-first EXE proof passes. | no backend matcher |
| `060A.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_completion_marker_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
