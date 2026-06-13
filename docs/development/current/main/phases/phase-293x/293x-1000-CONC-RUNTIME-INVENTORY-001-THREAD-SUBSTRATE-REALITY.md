# 293x-1000 CONC-RUNTIME-INVENTORY-001 Thread Substrate Reality

Status: landed-docs
Date: 2026-06-13

## Decision

The current repository already has runtime thread substrate below the `.hako`
source surface:

```text
ThreadApi:
  sleep / yield_now / current_thread_id / spawn / join / detach

Scheduler:
  SingleThreadScheduler
  WorkerPoolScheduler

Future ownership:
  FutureBox
  TaskGroupBox
```

This row fixes the inventory before source-level parallel design resumes. It
does not activate true parallel `.hako` semantics.

## Reading

```text
ThreadApi substrate present=1
WorkerPoolScheduler present=1
FutureBox present=1
TaskGroupBox present=1

nowait_os_thread_spawn=0
source_level_thread_syntax=0
worker_pool_source_route_enabled=0
lock_t_canonical_surface=0
sync_box_canonical_surface=1
```

The implemented worker pool is runtime substrate only. It may become an
execution route later, but `nowait` remains Future/task semantics and must not
be redefined as OS-thread spawn.

## Scope

- Update the runtime substrate reading in:
  - `docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md`
  - `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
  - `docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md`
- Record the side-lane pointer in `CURRENT_TASK.md`.
- Keep `CURRENT_STATE.toml` unchanged because the active lane remains the
  MIM-PORT-FMEM row.

## Stop Lines

- No behavior change.
- No source-level `thread {}` / `worker_scope` / `parallel` syntax.
- No `nowait` semantic change.
- No default worker-pool activation.
- No `lock<T>` promotion; `sync box` remains the canonical shared-mutable
  surface.
- No moving `.hako` values across worker threads before send/share/thread-root
  safety is pinned.

## Next Row

```text
CONC-SCHED-ROUTE-001:
  pin report/check vocabulary for inline_resolved_future,
  cooperative_task, and worker_pool_task runtime routes.
```

That row should remain report/check-only unless a later implementation card
explicitly opens worker-pool route selection.

## Evidence

```bash
rg -n "trait ThreadApi|fn yield_now|fn current_thread_id|fn spawn|fn join|fn detach" \
  src/runtime/ring0/traits.rs src/runtime/ring0/std_impls.rs

rg -n "struct SingleThreadScheduler|struct WorkerPoolScheduler|impl Scheduler for WorkerPoolScheduler" \
  src/runtime/scheduler.rs

rg -n "struct FutureBox|struct TaskGroupBox" \
  src/boxes/future src/boxes/task_group_box.rs

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
