# 293x-551 MIMAP-064A Reclaim Scheduler Request Marker Contract

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-064A` is the allocator-local scheduler request marker contract selected
by `MIMAP-063A`.

The row may add a small `.hako` owner that classifies whether a completed
scalar reclaim result would request modeled scheduler handoff or stay local /
suppressed. It must not execute real scheduling and must not expose
source-level concurrency semantics.

## Scope

- Add a scheduler request marker SSOT.
- Add a `.hako` marker contract owner if it stays scalar and local.
- Report request/suppress reason vocabulary.
- Add proof app and focused guard.
- Select the next row after the marker contract lands.

## Stop Lines

- No real thread scheduling.
- No source-level `nowait`, `Channel`, `task_scope`, `co`, `sync box`,
  `context`, or `worker_local` semantics.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `064A.1` | Write scheduler request marker SSOT. | reason vocabulary fixed. | no scheduling |
| `064A.2` | Add `.hako` marker contract owner. | scalar request/suppress report only. | no source concurrency |
| `064A.3` | Add proof app and focused guard. | VM / MIR JSON / pure-first EXE proof passes if owner exists. | no backend matcher |
| `064A.4` | Select follow-up row. | next row has one narrow owner. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Implementation Result

`MIMAP-064A` added `HakoAllocReclaimSchedulerRequestMarker`, a scalar marker
contract that composes `HakoAllocReclaimCompletionMarker`.

The route sets a request marker only when reclaim completion succeeds and the
modeled scheduler request flag is enabled. Completion-blocked and
scheduler-disabled cases remain scalar suppressions. Real scheduling,
source-level concurrency, page-source/OSVM release, provider activation, and
host allocator replacement remain closed.

## Evidence

```text
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 timeout 30 target/debug/hakorune --backend vm apps/hako-alloc-reclaim-scheduler-request-marker-proof/main.hako
bash tools/checks/k2_wide_hako_alloc_reclaim_scheduler_request_marker_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

## Selection Result

`MIMAP-064A` selects `MIMAP-065A`.

```text
row:
  MIMAP-065A reclaim scheduler marker closeout guard

classification:
  closeout / guard row

why now:
  the scheduler boundary and request marker are guarded. Before considering
  real scheduling or broader reclaim behavior, close this marker lane and keep
  source-level concurrency/provider stop lines explicit.

stop lines:
  no new allocator behavior
  no real thread scheduling
  no source-level concurrency feature change
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no cleanup bundle
```
