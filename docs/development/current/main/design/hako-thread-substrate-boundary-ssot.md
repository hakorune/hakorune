# Hako Thread Substrate Boundary SSOT

Status: SSOT
Decision: accepted
Date: 2026-06-05
Scope: `.hako` source-level concurrency semantics, runtime OS-thread substrate,
allocator threading evidence, and benchmark report claims.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`
- `docs/development/current/main/workstreams/mimalloc-current.md`
- `src/runtime/scheduler.rs`
- `src/runtime/ring0/traits.rs`

## Summary

Hakorune keeps source-level concurrency semantics separate from OS-thread
substrate and allocator benchmark evidence.

Fixed decisions:

```text
nowait_os_thread_spawn=0
hako_source_owns_raw_os_thread=0
c_pthread_benchmark_hako_thread_support_claim=0
worker_local_is_allocator_substrate=1
scoped_context_is_task_local=1
type_abi_hot_path_thread_lookup=0
```

`nowait` creates a Future/task semantic boundary. It must not become a source
promise to create one OS thread. A runtime may later execute eligible tasks on a
worker pool, but that is an execution route, not source meaning.

C pthread allocator benchmarks are allocator execution evidence only. They do
not prove `.hako` `nowait`, `co`, `task_scope`, `sync box`, `context`, or true
parallel language semantics.

## Layer Boundary

Recommended stack:

```text
.hako source
  nowait / await / co / task_scope / sync box / context
        |
        v
Future / Task / TaskGroup ownership semantics
        |
        v
Scheduler
  inline_resolved_future
  cooperative_task
  worker_pool_task   # future execution route
        |
        v
ThreadApi
  sleep / yield_now / current_thread_id / spawn / join
        |
        v
platform threads
  pthread / std::thread / platform thread APIs
```

Responsibilities:

```text
source semantics:
  Future creation, structured ownership, await/failure/cancel observation

Scheduler:
  task execution policy and route selection
  may understand Future/TaskGroup/context snapshots

ThreadApi:
  OS-thread substrate only
  must not know `.hako` source semantics

allocator replacement front:
  malloc/free ABI and native thread-local/remote-free execution evidence
  must not claim `.hako` source thread support
```

## Route Vocabulary

Use these route names in docs and reports:

```text
inline_resolved_future:
  Phase-0 Future path; expression may run sequentially before FutureNew

cooperative_task:
  queued/polled task under SingleThreadScheduler-style execution

worker_pool_task:
  future runtime worker-pool execution route; source semantics unchanged

os_thread_substrate_task:
  explicit runtime substrate task owned by ThreadApi/ThreadRegistry

detached_task:
  reserved for a future explicit advanced source/runtime surface
```

Rules:

- `nowait` may select `inline_resolved_future`, `cooperative_task`, or a future
  `worker_pool_task` route.
- `nowait` must not directly mean `os_thread_substrate_task`.
- `detached_task` remains closed until an explicit source/runtime decision opens
  it.

## Benchmark Claim Fields

Allocator reports that involve threads should use these fields:

```text
concurrency_surface_claim=phase0_future_only|cooperative_task|worker_pool_task|os_thread_surface|not_applicable
benchmark_thread_origin=none|c_pthread|std_thread|hako_scheduler|hako_worker_scope
hako_source_thread_support_claim=0|1
allocator_threading_evidence=none|c_side|hako_runtime|hako_source
nowait_os_thread_spawn=0|1
c_pthread_benchmark_hako_thread_support_claim=0|1
worker_local_is_allocator_substrate=0|1
scoped_context_is_task_local=0|1
```

Current replacement-front pthread benchmark reading:

```text
benchmark_thread_origin=c_pthread
concurrency_surface_claim=not_applicable
hako_source_thread_support_claim=0
allocator_threading_evidence=c_side
nowait_os_thread_spawn=0
c_pthread_benchmark_hako_thread_support_claim=0
```

## ThreadApi Minimal Shape

The code-side `ThreadApi` may grow as substrate, not as source syntax.

Preferred order:

```text
v0:
  sleep
  yield_now
  current_thread_id

v1:
  spawn(spec, closure) -> ThreadHandle
  join(handle) -> Result<ThreadExit>

v2:
  ThreadRegistry registration/unregistration
  thread-root cleanup
  worker id binding
```

`ThreadExit` should remain narrow at first:

```text
Ok
Panic(String)
```

Do not return `NyashBox` values across threads until send/share capability,
thread roots, and handle safety are pinned.

## Direct std::thread Cleanup Inventory

The current tree has direct host-thread calls that should be cleaned behind
ThreadApi/substrate rows before opening worker-pool execution:

```text
src/runtime/global_hooks.rs:
  std::thread::yield_now
  std::thread::spawn in spawn_task_after fallback

src/boxes/task_group_box.rs:
  std::thread::yield_now in join_pending_with_timeout

src/runtime/plugin_loader_unified.rs:
  std::thread::yield_now

src/runtime/plugin_loader_v2/enabled/extern_functions.rs:
  std::thread::yield_now

crates/nyash_kernel/src/plugin/future.rs:
  std::thread::spawn

src/boxes/p2p_box.rs:
  std::thread::spawn

crates/nyash_kernel/src/exports/mem.rs:
  std::thread::spawn for native/kernel stress

crates/nyash_kernel/src/tests/mimalloc_parallel_stress.rs:
  std::thread::spawn in tests only
```

This inventory is not a request to rewrite all callers in one change. Start
with scheduler/global-hooks/task-group surfaces where OS-thread dependency leaks
into runtime policy.

## Context, Worker Local, and Sync Boundaries

Context:

```text
context/scoped:
  task-local/request-local dynamic context
  immutable or copy-on-write snapshot
  structured child tasks inherit at creation time
  raw ThreadApi spawn does not inherit implicitly
```

Worker local:

```text
worker_local:
  allocator/cache substrate
  may map to TLS/per-worker slots
  not inherited by tasks
  not request/trace context
```

Sync/shared state:

```text
sync box:
  serialized method boundary when runtime rows land
  await/nowait/yield/blocking calls remain forbidden inside sync methods
  does not make returned handles or external resources automatically thread-safe
```

## Implementation Task Order

### THREAD-BOUNDARY-001: docs/report-only boundary

Status: landed as the first docs/report-only boundary cut.

Scope:

```text
nowait_os_thread_spawn=0
c_pthread_benchmark_hako_thread_support_claim=0
benchmark_thread_origin=c_pthread for replacement-front pthread reports
hako_source_thread_support_claim=0
worker_local_is_allocator_substrate=1
scoped_context_is_task_local=1
```

No behavior change. No source syntax expansion.

### THREAD-API-001: yield/current-id substrate

Status: landed.

Scope:

```text
ThreadApi::yield_now
ThreadApi::current_thread_id
replace runtime-policy direct std::thread::yield_now calls with ThreadApi
```

No worker pool. No `spawn`/`join` yet.

Landed behavior:

```text
HostThreadId=u64
StdThread::yield_now wraps std::thread::yield_now
StdThread::current_thread_id hashes std::thread::ThreadId into an opaque diagnostic/registry id
runtime_policy_direct_yield_now_count=0 outside Ring0 StdThread
```

### THREAD-API-002: spawn/join substrate

Status: landed as spawn-inventory/report-only. Substrate implementation remains
THREAD-API-003.

Direct `std::thread::spawn` classification:

| Path | Classification | Next owner |
| --- | --- | --- |
| `src/runtime/global_hooks.rs` `spawn_task_after` fallback | runtime substrate leak | landed in `THREAD-REG-001`: `ThreadApi::spawn` + `detach` |
| `crates/nyash_kernel/src/plugin/future.rs` `nyash_future_delay_i64` | runtime/plugin delayed future substrate | `THREAD-API-003` or future timer scheduler row |
| `src/boxes/p2p_box.rs` async reply helpers | box-specific async workaround | later P2P/task route cleanup; not generic ThreadApi proof |
| `crates/nyash_kernel/src/exports/mem.rs` thread-safe mem test | kernel native stress/test | keep as native execution evidence |
| `crates/nyash_kernel/src/tests/mimalloc_parallel_stress.rs` | allocator native stress/test | keep as native execution evidence |

Report fields:

```text
direct_std_thread_spawn_total=6
runtime_substrate_spawn_candidate_count=2
box_specific_spawn_workaround_count=2
kernel_native_stress_spawn_count=2
hako_source_thread_support_claim=0
```

### THREAD-API-003: spawn/join substrate

Status: landed as ThreadApi substrate only.

Scope:

```text
ThreadHandle opaque id
ThreadExit::Ok | ThreadExit::Panic(String)
ThreadRegistry for handles
spawn/join not exposed to `.hako` source
```

Do not move `NyashBox` values across threads in this row.

Landed behavior:

```text
ThreadHandle=u64 opaque id
ThreadSpawnSpec.name=optional
ThreadExit=Ok|Panic(String)
ThreadApi::spawn stores JoinHandle in the ThreadApi registry
ThreadApi::join removes and joins the registered handle
source_syntax_exposure=0
nowait_os_thread_spawn=0
worker_pool_enabled=0
```

### THREAD-REG-001: detached runtime delayed fallback cleanup

Status: landed.

Scope:

```text
ThreadApi::detach(handle)
spawn_task_after fallback uses ThreadApi::spawn
spawn_task_after fallback detaches fire-and-forget handle
source_syntax_exposure=0
nowait_os_thread_spawn=0
```

Landed behavior:

```text
ThreadApi::detach removes handle from the registry without joining
spawn_task_after fallback creates ThreadExit::Ok after delayed closure returns
thread_spawn_failed_tag=[freeze:contract][thread/spawn_failed]
thread_detach_failed_tag=[freeze:contract][thread/detach_failed]
direct_std_thread_spawn_total_after=5
runtime_substrate_spawn_candidate_count_after=1
```

### THREAD-SCHED-001: WorkerPoolScheduler route

Scope:

```text
Scheduler implementation only
source `nowait` semantics unchanged
route decision report for inline_resolved_future/cooperative_task/worker_pool_task
```

Requires capture/thread-root safety before routing user tasks to workers by
default.

### THREAD-SAFETY-001: send/share/root capability

Scope:

```text
move-capable Box
share-capable Box
thread root registration
thread exit cleanup
```

Required before source-level worker/parallel surfaces.

### THREAD-SOURCE-001: structured worker source surface

Reserved future row. Prefer structured surfaces such as `worker_scope` /
`parallel` before raw `thread { ... }` or detached tasks.

## Stop Lines

- Do not reinterpret `nowait` as OS thread spawn.
- Do not add raw `thread { ... }` source syntax before capability/root safety.
- Do not use C pthread allocator benchmarks as `.hako` thread support evidence.
- Do not use `worker_local` as request/task context.
- Do not make Type ABI a hot-path thread dispatcher.
- Do not open product allocator replacement, hook install, or global allocator
  claim from these rows.
