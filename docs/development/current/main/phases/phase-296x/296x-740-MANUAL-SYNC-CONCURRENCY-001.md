---
Status: Landed
Date: 2026-06-15
Task: MANUAL-SYNC-CONCURRENCY-001
Scope: Synchronize user-facing concurrency/thread manual entry points with the
  current co/nowait/worker-scope/thread-substrate decisions.
Related:
  - docs/reference/concurrency/semantics.md
  - docs/reference/concurrency/boundary-model.md
  - docs/reference/runtime/threading.md
  - docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
---

# MANUAL-SYNC-CONCURRENCY-001

## Result

```text
output_contract=hako-manual-sync-concurrency-v0
source_evidence=CONC-SOURCE-PARALLEL-001
co_canonical_surface=1
task_scope_compat_surface=1
nowait_os_thread_spawn=0
readme_async_sample_uses_co=1
worker_scope_design_reserved=1
worker_scope_workers_is_upper_bound=1
worker_scope_exact_thread_count_promise=0
raw_thread_parser_enabled=0
threadapi_substrate_not_source_syntax=1
concurrency_reference_nav_linked=1
summary=ok
```

## Decision

The manual should teach:

```text
current source:
  co / nowait / await

reserved structured parallel:
  worker_scope workers=N { parallel ... }
  design-only until safety gates

closed source:
  raw thread {}

runtime substrate:
  ThreadApi / WorkerPoolScheduler
```

`workers=N` is a scheduler budget hint / upper bound, not an exact OS thread
count promise.

## Stop Line

```text
do not imply nowait spawns OS threads
do not expose raw thread syntax
do not present WorkerPoolScheduler as source syntax
do not open worker_scope parser/lowering before safety gates
```
