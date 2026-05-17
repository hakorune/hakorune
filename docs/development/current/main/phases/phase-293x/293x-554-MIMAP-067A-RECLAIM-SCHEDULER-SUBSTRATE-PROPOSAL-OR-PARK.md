# 293x-554 MIMAP-067A Reclaim Scheduler Substrate Proposal Or Park

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-067A` is the allocator substrate planning row selected by `MIMAP-066A`.

The row decides whether to open a narrow allocator-internal scheduler substrate
implementation lane, park real scheduling for now, or switch to a concrete
Hakorune language/compiler prerequisite.

## Scope

- Read the closed scalar reclaim and scheduler marker evidence.
- Compare next options:
  - allocator-internal scheduler substrate
  - next scalar allocator behavior without real scheduling
  - Hakorune language feature prerequisite
  - compiler acceptance sidecar
- Select exactly one next row with stop lines.

## Stop Lines

- No allocator behavior in this row.
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
| `067A.1` | Read closed reclaim/scheduler evidence. | blocker set is accurate. | no code |
| `067A.2` | Decide open vs park for real scheduler substrate. | one row is selected. | no bundle |
| `067A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence Review

The scalar reclaim lane is closed through:

```text
MIMAP-051A owner-transfer contract inventory
MIMAP-054A reclaim atomic-claim contract
MIMAP-055A owner-transfer first execution route
MIMAP-056A remote-free drain contract inventory
MIMAP-057A remote-free drain first execution route
MIMAP-058A post-drain owner-transfer integration route
MIMAP-060A reclaim completion marker route
MIMAP-061A scalar reclaim lane closeout guard
```

The scheduler boundary/request-marker slice is closed through:

```text
MIMAP-063A scheduler boundary inventory
MIMAP-064A scheduler request marker contract
MIMAP-065A scheduler marker closeout guard
```

`MIMAP-064A` proves that a completed scalar reclaim row can set a scheduler
request marker while keeping all production-facing scheduling surfaces inactive:

```text
would_schedule_thread = 0
would_spawn_worker = 0
would_touch_source_concurrency = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
would_host_allocator_swap = 0
```

Opening real scheduling now would cross a larger substrate boundary:

```text
worker handoff / run queue semantics
progress and retry policy
native wake/sleep or thread execution
failure and timeout diagnostics
```

None of those are required to continue allocator-first evidence. The next
small row should instead keep the scheduler request executor-local and record
one pending request in an allocator-owned scalar ledger.

## Selection Result

`MIMAP-067A` selects `MIMAP-068A`.

```text
row:
  MIMAP-068A reclaim scheduler request ledger route

classification:
  allocator behavior / modeled scheduler request ledger

decision:
  park real scheduler substrate for now. Continue with a scalar allocator-owned
  request ledger that composes the MIMAP-064A request marker and records one
  pending modeled scheduler request without executing scheduling.

why not real scheduler substrate:
  real scheduling requires worker handoff, progress, wake/run semantics, and
  timeout diagnostics. The current allocator evidence only needs a durable
  request record after completion succeeds.

why not language feature:
  no current allocator proof is blocked on source-level co / nowait / Channel /
  sync box / context / worker_local semantics.

why not compiler sidecar:
  no current proof app exposes a concrete compiler acceptance failure.

stop lines:
  no real thread scheduling
  no worker spawn
  no source-level concurrency feature change
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no host allocator replacement
  no backend matcher
```

Closeout:

```text
current blocker moves to MIMAP-068A.
```
