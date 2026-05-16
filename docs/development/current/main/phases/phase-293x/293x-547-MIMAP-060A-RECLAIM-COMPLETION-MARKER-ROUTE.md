# 293x-547 MIMAP-060A Reclaim Completion Marker Route

Status: landed
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

## Implementation Result

`MIMAP-060A` added `HakoAllocReclaimCompletionMarker`, a scalar completion
owner that composes `HakoAllocReclaimPostDrainOwnerTransfer`.

The route marks completion only when post-drain owner-transfer succeeds. It
preserves the integration reason on blocked rows and keeps page-source calls,
OSVM release/unreserve, scheduling, provider activation, and host allocator
replacement inactive.

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 target/debug/hakorune --backend vm apps/hako-alloc-reclaim-completion-marker-proof/main.hako
bash tools/checks/k2_wide_hako_alloc_reclaim_completion_marker_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Selection Result

`MIMAP-060A` selects `MIMAP-061A`.

```text
row:
  MIMAP-061A reclaim scalar lane closeout guard

classification:
  closeout / guard row

why now:
  reclaim has landed owner-transfer, drain, post-drain integration, and
  completion-marker rows. Before opening scheduling, page-source release, or
  provider behavior, lock this scalar reclaim lane with one closeout guard.

stop lines:
  no new allocator behavior
  no thread scheduling
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no cleanup bundle
```
