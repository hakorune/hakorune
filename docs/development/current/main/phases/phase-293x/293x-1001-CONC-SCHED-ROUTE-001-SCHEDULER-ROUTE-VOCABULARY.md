# 293x-1001 CONC-SCHED-ROUTE-001 Scheduler Route Vocabulary

Status: landed-code
Date: 2026-06-13

## Decision

Pin runtime scheduler route names and report fields before any source-level
parallel surface is designed.

This row adds descriptor vocabulary only. It does not select `WorkerPool` as a
default route and does not change `nowait` semantics.

## Routes

```text
inline_resolved_future:
  Phase-0 Future path; expression may run before FutureNew

cooperative_task:
  queued task route under cooperative scheduler polling

worker_pool_task:
  runtime worker-pool route; source semantics remain unchanged
```

Code owner:

```text
src/runtime/scheduler_route.rs
```

## Report Fields

```text
scheduler_route_inline_resolved_future_descriptor_present=1
scheduler_route_cooperative_task_descriptor_present=1
scheduler_route_worker_pool_task_descriptor_present=1

scheduler_route_worker_pool_default_enabled=0
worker_pool_source_route_enabled=0
source_level_thread_syntax=0
nowait_os_thread_spawn=0
```

These fields are deliberately separate from `ThreadCapabilityDescriptor`. A
scheduler route answers "how a task may execute"; send/share/thread-root
capabilities answer "what may safely cross that route".

## Stop Lines

- No behavior change.
- No default worker-pool activation.
- No source-level `worker_scope` / `parallel` / raw `thread {}` syntax.
- No `nowait` semantic change.
- No value movement across workers.
- No capability enforcement in this row.

## Next Row

```text
CONC-CAP-INVENTORY-001:
  inventory HakoSend / HakoShare / ThreadRoot gaps before cross-worker
  value movement or source-level worker design.
```

## Evidence

```bash
cargo test -q --lib runtime::scheduler_route
rg -n "HakoSchedulerRoute|scheduler_route_report_fields|scheduler_route_activation_report_fields" \
  src/runtime
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
