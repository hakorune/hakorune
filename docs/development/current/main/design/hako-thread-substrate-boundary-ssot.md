# Hako Thread Substrate Boundary SSOT

Status: SSOT
Decision: accepted
Date: 2026-06-05
Scope: `.hako` source-level concurrency semantics, runtime OS-thread substrate,
allocator threading evidence, and benchmark report claims.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/reference/runtime/threading.md`
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

Current substrate reading:

```text
ThreadApi sleep/yield_now/current_thread_id/spawn/join/detach=implemented
SingleThreadScheduler=implemented
WorkerPoolScheduler=implemented
ThreadRegistry v0=implemented
source_level_thread_syntax=0
worker_pool_source_route_enabled=0
worker_scope_design_reserved=1
worker_scope_parser_enabled=0
worker_scope_workers_is_upper_bound=1
```

The implemented worker pool is runtime substrate only. It is not evidence that
`.hako` `nowait` means OS-thread spawn, and it is not a reason to add raw
source-level thread syntax before send/share/root safety is pinned.

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

The current tree keeps only non-runtime-substrate direct spawn sites outside
ThreadApi. Runtime policy and delayed Future substrate calls are routed through
ThreadApi before opening worker-pool execution:

```text
crates/nyash_kernel/src/exports/mem.rs:
  std::thread::spawn for native/kernel stress

crates/nyash_kernel/src/tests/mimalloc_parallel_stress.rs:
  std::thread::spawn in tests only
```

This inventory is not a request to rewrite all callers in one change. Runtime
policy and runtime/plugin delayed Future surfaces are closed behind ThreadApi;
P2P async reply helpers need a separate P2P/task-route cleanup, and native
stress/test files stay native evidence.

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

### Pre-selfhost substrate order

Use this order if the concurrency substrate is reopened before selfhost:

| Order | Row | Purpose | Stop line |
| --- | --- | --- | --- |
| 1 | `CONC-RUNTIME-INVENTORY-001` | Sync docs/report inventory with current ThreadApi, WorkerPoolScheduler, FutureBox, and TaskGroupBox reality. | docs/report-only |
| 2 | `CONC-SCHED-ROUTE-001` | Pin runtime scheduler route vocabulary: `inline_resolved_future`, `cooperative_task`, `worker_pool_task`. | no default worker-pool activation |
| 3 | `CONC-CAP-INVENTORY-001` | Inventory send/share/thread-root requirements before cross-worker `.hako` values move. | no enforcement; no value movement |
| 4 | `CONC-SYNCBOX-003` | Add reference serialized entry behavior for canonical `sync box`. | no fairness/reentrancy guarantee |
| 5 | `CONC-CHANNEL-002/003` | Implement the future `Channel<T>` queue runtime separately from legacy P2P `ChannelBox`. | no hidden blocking ordinary calls |
| 6 | `CONC-SOURCE-PARALLEL-001` | Reserve source-level worker/parallel surface after substrate safety rows. | docs-only; `THREAD-SAFETY-001` required before parser/lowering |

Do not use `lock<T>` as the new canonical source surface. `sync box` is the
canonical shared-mutable surface; locks remain runtime/internal or historical
compatibility vocabulary.

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
| `crates/nyash_kernel/src/plugin/future.rs` `nyash_future_delay_i64` | runtime/plugin delayed future substrate | landed in `THREAD-REG-002`: `ThreadApi::spawn` + `detach` |
| `src/boxes/http_server_box.rs` client handler | box-specific server workaround | landed in `THREAD-REG-003`: `ThreadApi::spawn` + `detach` |
| `src/boxes/p2p_box.rs` async reply helpers | box-specific async workaround | landed in `P2P-THREAD-001`: `ThreadApi::spawn` + `detach` |
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
spawn_task_after fallback success returns true
thread_spawn_failed_tag=[freeze:contract][thread/spawn_failed]
thread_detach_failed_tag=[freeze:contract][thread/detach_failed]
direct_std_thread_spawn_total_after=5
runtime_substrate_spawn_candidate_count_after=1
```

### THREAD-REG-002: delayed Future substrate cleanup

Status: landed.

Scope:

```text
nyash_future_delay_i64 uses ensure_global_ring0_initialized
nyash_future_delay_i64 uses ThreadApi::spawn
nyash_future_delay_i64 uses ThreadApi::detach
nyash_future_delay_i64 sleeps through ThreadApi::sleep
source_syntax_exposure=0
nowait_os_thread_spawn=0
```

Landed behavior:

```text
future_delay_spawn_failed_sets_failed_future=1
future_delay_detach_failed_sets_failed_future=1
direct_std_thread_spawn_total_after=5
runtime_substrate_spawn_candidate_count_after=0
box_specific_spawn_workaround_count_after=3
kernel_native_stress_spawn_count_after=2
```

### THREAD-REG-003: HTTP server client handler substrate cleanup

Status: landed.

Scope:

```text
HTTPServerBox client handler uses ThreadApi::spawn
HTTPServerBox client handler uses ThreadApi::detach
HTTPServerBox active connection registry stores connection ids
HTTPServerBox removes active connection id when handler completes
source_syntax_exposure=0
nowait_os_thread_spawn=0
```

Landed behavior:

```text
direct_std_thread_spawn_total_after=4
runtime_substrate_spawn_candidate_count_after=0
box_specific_spawn_workaround_count_after=2
kernel_native_stress_spawn_count_after=2
http_server_active_connections_unbounded_growth=0
```

### P2P-THREAD-001: P2P async reply helper cleanup

Status: landed.

Scope:

```text
P2PBox sys.ping async reply uses ThreadApi::spawn
P2PBox debug async reply uses ThreadApi::spawn
P2PBox async reply sleeps through ThreadApi::sleep
P2PBox async reply detaches fire-and-forget handles
source_syntax_exposure=0
nowait_os_thread_spawn=0
```

Landed behavior:

```text
direct_std_thread_spawn_total_after=2
runtime_substrate_spawn_candidate_count_after=0
box_specific_spawn_workaround_count_after=0
kernel_native_stress_spawn_count_after=2
p2p_async_reply_threadapi_route=1
```

### THREAD-SCHED-001: WorkerPoolScheduler route

Status: landed as runtime substrate only.

Scope:

```text
Scheduler implementation only
source `nowait` semantics unchanged
route decision report for inline_resolved_future/cooperative_task/worker_pool_task
```

Requires capture/thread-root safety before routing user tasks to workers by
default.

Landed behavior:

```text
WorkerPoolScheduler implemented=1
WorkerPoolScheduler default_enabled=0
WorkerPoolScheduler source_route_enabled=0
WorkerPoolScheduler uses ThreadApi::spawn=1
WorkerPoolScheduler joins workers on drop=1
WorkerPoolScheduler spawn_after route=threadapi_single_timer_enqueue
WorkerPoolScheduler delayed tasks occupy worker while waiting=0
WorkerPoolScheduler delayed tasks require external poll=0
WorkerPoolScheduler delayed timer threads per scheduler=1
WorkerPoolScheduler delayed timer threads per delayed task=0
source_syntax_exposure=0
nowait_os_thread_spawn=0
worker_pool_enabled_by_default=0
```

### CONC-SCHED-ROUTE-001: scheduler route vocabulary

Status: landed as report/check vocabulary only.

Scope:

```text
src/runtime/scheduler_route.rs
HakoSchedulerRoute::InlineResolvedFuture
HakoSchedulerRoute::CooperativeTask
HakoSchedulerRoute::WorkerPoolTask
```

Report fields:

```text
scheduler_route_inline_resolved_future_descriptor_present=1
scheduler_route_cooperative_task_descriptor_present=1
scheduler_route_worker_pool_task_descriptor_present=1
scheduler_route_worker_pool_default_enabled=0
worker_pool_source_route_enabled=0
source_level_thread_syntax=0
nowait_os_thread_spawn=0
```

This row does not select a scheduler route. It only freezes the names future
reports and checks must use.

### THREAD-SAFETY-001: send/share/root capability

Status: ThreadRegistry v0 landed. Box send/share capability remains closed.

Scope:

```text
move-capable Box
share-capable Box
thread root registration
thread exit cleanup
```

Required before source-level worker/parallel surfaces.

Subtasks:

```text
THREAD-SAFETY-001A:
  docs/task boundary
  define WorkerId separate from HostThreadId
  define thread registry as runtime substrate
  keep HakoSend/HakoShare as descriptor-only future capability

THREAD-SAFETY-001B:
  implement ThreadRegistry v0
  register/unregister runtime worker threads
  expose snapshot/count for diagnostics and tests
  no GC root set yet
  no Box move/share authorization yet

THREAD-SAFETY-001C:
  connect WorkerPoolScheduler workers to ThreadRegistry
  register on worker entry
  unregister on worker exit/drop shutdown
  source_syntax_exposure=0
  nowait_os_thread_spawn=0

THREAD-SAFETY-001D:
  future capability descriptors
  HakoSend/HakoShare/ThreadRoot remain metadata until object/root safety lands
```

Acceptance fields:

```text
thread_registry_v0=1
worker_id_distinct_from_host_thread_id=1
worker_pool_threads_registered=1
worker_pool_threads_unregistered_on_exit=1
thread_registry_snapshot_available=1
thread_registry_gc_roots_enabled=0
hako_send_share_enforced=0
source_syntax_exposure=0
nowait_os_thread_spawn=0
worker_pool_source_route_enabled=0
```

Landed behavior:

```text
ThreadRegistry module=src/runtime/thread_registry.rs
WorkerId shape=u64_opaque
ThreadRegistryRole=RuntimeWorker|HostThread|Test
WorkerPoolScheduler registers worker thread on entry=1
WorkerPoolScheduler unregisters worker thread on exit=1
WorkerPoolScheduler unregisters worker thread during panic unwind=1
thread_registry_snapshot_available=1
thread_registry_gc_roots_enabled=0
hako_send_share_enforced=0
source_syntax_exposure=0
nowait_os_thread_spawn=0
worker_pool_source_route_enabled=0
```

### THREAD-SAFETY-001D: descriptor-only thread capability vocabulary

Status: landed.

Scope:

```text
define stable descriptor keys for future thread safety capabilities
do not authorize Box movement across threads
do not enable worker-pool source routing
do not expose raw thread syntax
do not make Type ABI a hot-path dispatcher
```

Capability descriptor keys:

```text
hako.thread.send:
  future metadata that a Box value may be moved to another runtime worker

hako.thread.share:
  future metadata that a Box value may be shared by multiple runtime workers

hako.thread.root:
  future metadata that a thread participates in runtime root ownership
```

Landed behavior:

```text
ThreadCapabilityDescriptor module=src/runtime/thread_capability.rs
hako_send_capability_descriptor_present=1
hako_share_capability_descriptor_present=1
hako_thread_root_descriptor_present=1
hako_send_share_enforced=0
thread_registry_gc_roots_enabled=0
worker_pool_source_route_enabled=0
source_syntax_exposure=0
nowait_os_thread_spawn=0
type_abi_hot_path_thread_lookup=0
```

### CONC-CAP-INVENTORY-001: send/share/root gap inventory

Status: landed as report/check vocabulary only.

Scope:

```text
thread_capability_inventory_report_fields()
```

Report fields:

```text
hako_send_candidate_count=0
hako_share_candidate_count=0
hako_thread_root_candidate_count=0
rejected_non_send_count=0
rejected_non_share_count=0
thread_root_required_count=0
cross_worker_value_move_enabled=0
```

This row does not enforce send/share/root capabilities. It fixes the
diagnostic vocabulary needed before a later row can move `.hako` values across
workers or reject non-send/non-share values.

### CONC-SYNCBOX-003: reference serialized entry

Status: landed as reference runtime only.

Scope:

```text
src/runtime/sync_box.rs
SyncState per future sync object instance
SyncState::enter(object_id, method_name)
TLS nested-entry guard
```

Report fields:

```text
sync_box_reference_runtime_enabled=1
sync_box_mir_lowering_enabled=0
sync_box_program_json_enabled=0
sync_box_llvm_enabled=0
sync_box_fairness_guarantee=0
sync_box_reentrancy_guarantee=0
sync_box_lock_order_verifier_enabled=0
sync_box_worker_pool_route_enabled=0
```

Reading:

```text
serialized_truth_owner=runtime_object_instance_side
method_dispatch_role=enter_exit_only_later
program_json_v0_sync_box_support=0
mir_sync_box_lowering=0
llvm_sync_box_lowering=0
```

Reentrant sync method entry is fail-fast in v0:

```text
[syncbox/reentrant-entry]
```

This row does not open normal MIR/Program JSON/LLVM lowering. It proves the
reference runtime boundary and keeps backend fallback forbidden.

### CONC-CHANNEL-002: reference close semantics

Status: landed as reference runtime only.

Scope:

```text
src/runtime/channel_queue.rs
ChannelQueue<T>
ChannelQueue::close()
ChannelQueue::try_recv()
ChannelQueue::recv_blocking_reference()
```

Report fields:

```text
channel_queue_reference_runtime_enabled=1
channel_queue_legacy_p2p_channelbox_reused=0
channel_queue_close_wakes_waiters_reference=1
channel_queue_send_after_close_rejected=1
channel_queue_drain_after_close_enabled=1
channel_queue_double_close_rejected=1
channel_queue_true_parallel_scheduler_required=0
channel_queue_source_blocking_call_enabled=0
```

Reading:

```text
channel_queue_truth_owner=runtime_channel_queue_reference
legacy_p2p_channelbox_is_canonical_channel=0
source_blocking_recv_enabled=0
worker_pool_required_for_close=0
```

`recv_blocking_reference()` exists only to prove that close wakes a waiting
reference receiver. It is not a source-level ordinary blocking call.
`CONC-CHANNEL-003` owns await-visible `send` / `recv` route integration.

### CONC-CHANNEL-003: await-visible route bridge

Status: landed as route vocabulary / fail-fast bridge.

Scope:

```text
src/runtime/channel_route.rs
HakoChannelRoute
ChannelRouteDescriptor
```

Report fields:

```text
channel_route_await_send_descriptor_present=1
channel_route_await_recv_descriptor_present=1
channel_route_await_close_descriptor_present=1
channel_route_try_send_descriptor_present=1
channel_route_try_recv_descriptor_present=1
channel_route_hidden_blocking_ordinary_call_enabled=0
channel_route_mir_lowering_enabled=0
channel_route_program_json_enabled=0
channel_route_llvm_enabled=0
channel_route_legacy_p2p_channelbox_reused=0
```

Reading:

```text
await_visible_channel_route_shape_fixed=1
ordinary_blocking_channel_call_enabled=0
channel_route_lowering_enabled=0
```

This row fixes the route names and source shapes only. It does not infer
`Channel<T>` receiver types and does not lower channel waits through MIR.

### CONC-CONTEXT-002: explicit-scope context snapshot

Status: landed as reference runtime only.

Scope:

```text
src/runtime/context_snapshot.rs
src/runtime/global_hooks.rs
push_context_binding(name, value)
pop_context_binding(name)
current_context_snapshot()
context_snapshot_for_future(future)
```

Report fields:

```text
context_snapshot_runtime_enabled=1
context_snapshot_explicit_scope_only=1
context_snapshot_implicit_root_propagation=0
context_snapshot_program_json_enabled=0
context_snapshot_mir_lowering_enabled=0
context_snapshot_llvm_enabled=0
```

Reading:

```text
explicit_co_task_scope_child_context_snapshot=1
implicit_root_context_propagation=0
program_json_context_scope_support=0
mir_context_scope_lowering=0
```

This row proves child creation snapshots at runtime registration time. It does
not open Program JSON / MIR lowering for `ContextScope`.

### CONC-SOURCE-PARALLEL-001: structured parallel source reservation

Status: landed-docs.

`co { nowait ... await ... }` remains the current canonical user-facing
concurrency source surface:

```text
co_nowait_await_canonical_source_surface=1
nowait_os_thread_spawn=0
```

The future structured parallel source surface is reserved as design-only:

```hako
worker_scope workers = N {
    parallel i in range {
        work(i)
    }
}
```

Current status:

```text
worker_scope_design_reserved=1
worker_scope_parser_enabled=0
worker_scope_ast_json_enabled=0
worker_scope_program_json_enabled=0
worker_scope_mir_lowering_enabled=0
worker_scope_llvm_lowering_enabled=0
worker_scope_runtime_route_enabled=0
raw_thread_parser_enabled=0
source_level_thread_syntax=0
```

`workers = N` is a scheduler budget hint and upper bound. It must not become an
exact OS-thread-count promise:

```text
worker_scope_workers_is_upper_bound=1
worker_scope_exact_thread_count_promise=0
worker_scope_os_thread_spawn_direct=0
```

Opening source-visible `worker_scope` requires safety enforcement first:

```text
thread_safety_gate_required=1
hako_send_share_enforced=1
thread_registry_gc_roots_enabled=1
worker_scope_capture_check_enabled=1
worker_scope_value_movement_enabled=1
```

Until those fields are true, do not add parser support, AST JSON, Program JSON,
MIR lowering, LLVM lowering, or runtime route activation. If a later row makes
`worker_scope` source-visible, any route that uses fewer/equivalent workers or
falls back to cooperative/inline execution must report that route explicitly:

```text
worker_scope_silent_fallback_count=0
worker_scope_effective_route=inline_resolved_future|cooperative_task|worker_pool_task
worker_scope_effective_workers=<n>
```

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
