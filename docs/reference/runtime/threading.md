# Runtime Threading Substrate Reference

Status: provisional reference

This document is the reference entry for the current runtime threading
substrate. It describes code-side execution support only. It does not widen the
`.hako` source-level concurrency surface.

Design SSOT:

- `docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md`
- `docs/reference/concurrency/semantics.md`
- `docs/reference/runtime/substrate-capabilities.md`

Historical Ring0 notes may exist under old phase archives. Treat those as
provenance only; this file is the current `docs/reference/` entry for the
runtime thread substrate.

Code owners:

- `src/runtime/scheduler.rs`
- `src/runtime/thread_registry.rs`
- `src/runtime/ring0/traits.rs`
- `src/runtime/ring0/std_impls.rs`
- `src/runtime/global_hooks.rs`
- `src/boxes/http_server_box.rs`

## Stop Line

The current runtime thread substrate does not mean:

```text
nowait_os_thread_spawn=0
hako_source_owns_raw_os_thread=0
source_level_thread_syntax=0
detached_task_source_contract=0
hako_source_thread_support_claim_from_c_pthread_bench=0
provider_activation=0
global_allocator_replacement=0
```

`nowait` remains a Future/task semantic boundary. A scheduler may choose an
execution route, but `.hako` source does not directly own OS thread creation.

## Layer Map

```text
.hako source
  nowait / await / co / task_scope / sync box / context
        |
        v
Future / Task / TaskGroup ownership semantics
        |
        v
Scheduler
  SingleThreadScheduler
  WorkerPoolScheduler
        |
        v
ThreadApi
  sleep / yield_now / current_thread_id / spawn / join / detach
        |
        v
platform threads
```

Runtime boxes may use `ThreadApi` as substrate, but that is not language-level
thread syntax.

## Current Implementation Inventory

| Surface | Code owner | Current reference reading |
| --- | --- | --- |
| `Scheduler` trait | `src/runtime/scheduler.rs` | Runtime execution policy. It can enqueue, delay, poll, and yield tasks. It is not `.hako` syntax. |
| `SingleThreadScheduler` | `src/runtime/scheduler.rs` | Cooperative queue plus delayed-task polling. This is the default runtime scheduler path. |
| `WorkerPoolScheduler` | `src/runtime/scheduler.rs` | Runtime worker-pool substrate using `ThreadApi::spawn`; not wired as a source-level `nowait` promise. |
| `ThreadRegistry` | `src/runtime/thread_registry.rs` | Diagnostic/future-root registry for runtime worker threads. It tracks `WorkerId`, host thread id, role, and name. |
| `ThreadApi` | `src/runtime/ring0/traits.rs` | Ring0 OS-thread substrate abstraction. It does not know `.hako` source semantics. |
| `StdThread` | `src/runtime/ring0/std_impls.rs` | Standard host implementation of `ThreadApi` over `std::thread`. |
| global hooks | `src/runtime/global_hooks.rs` | Runtime helper surface. `spawn_task_after` falls back through `ThreadApi::spawn` and `detach`. |
| `HTTPServerBox` client handling | `src/boxes/http_server_box.rs` | Server box routes per-client handler work through `ThreadApi::spawn` and `detach`; active connection ids are removed when handlers finish. |

## ThreadApi Contract

Current shape:

```text
sleep(duration)
yield_now()
current_thread_id() -> HostThreadId
spawn(ThreadSpawnSpec, closure) -> Result<ThreadHandle, ThreadSpawnError>
join(ThreadHandle) -> Result<ThreadExit, ThreadJoinError>
detach(ThreadHandle) -> Result<(), ThreadJoinError>
```

Rules:

- `ThreadHandle` is opaque. Callers must not depend on host thread ids or
  `std::thread::JoinHandle` representation.
- `HostThreadId` is diagnostic/registry identity, not a stable language value.
- `ThreadExit` is intentionally narrow: `Ok` or `Panic(String)`.
- `spawn`/`join`/`detach` are runtime substrate operations. They are not
  exposed as `.hako` source-level `thread {}` or detached-task syntax.
- `detach` removes the handle from the ThreadApi registry without joining.

## Scheduler Contract

`Scheduler` owns execution policy:

```text
spawn(name, task)
spawn_after(delay_ms, name, task)
poll()
yield_now()
spawn_with_token(name, token, task)
```

Current routes:

- `SingleThreadScheduler` queues work and moves due delayed tasks during
  `poll()`.
- `WorkerPoolScheduler` starts runtime worker threads with `ThreadApi::spawn`.
  Delayed tasks are managed by a timer thread and sent to the worker queue when
  due.
- `CancellationToken` exists as substrate vocabulary, but the default
  `spawn_with_token` path still delegates to `spawn` unless an implementation
  overrides it.

Source meaning remains separate:

```text
nowait != ThreadApi::spawn
co/task_scope != raw OS thread ownership
```

## ThreadRegistry Contract

`ThreadRegistry` currently tracks runtime thread registrations:

```text
WorkerId
ThreadRegistryRole = RuntimeWorker | HostThread | Test
ThreadRegistration {
  worker_id
  host_thread_id
  role
  name
}
```

Rules:

- Registering the same host thread id returns the existing `WorkerId`.
- Runtime worker pool threads unregister on thread exit through RAII cleanup.
- The registry is diagnostic/future-root substrate. It does not authorize
  moving or sharing arbitrary `Box` values across threads.

## Runtime Hook Routing

`spawn_task_after(delay_ms, name, task)` first tries the configured scheduler.
If no scheduler is available, it uses:

```text
ThreadApi::spawn
ThreadApi::sleep
ThreadApi::detach
```

Fallback spawn failure is fail-fast. Successful fallback scheduling returns
`true`.

## Box-Level Routing

`HTTPServerBox` currently uses `ThreadApi` for per-client handler work:

```text
HTTPServerBox accept loop
  -> ThreadApi::spawn("HTTPServerBox.client", handler)
  -> ThreadApi::detach(handle)
  -> unregister active connection id when handler finishes
```

This is a box/runtime implementation route only. It does not imply a
source-level thread surface.

`TimeBox`, `SoundBox`, and `SocketBox` route user-visible sleep/polling waits
through Ring0 `ThreadApi::sleep`:

```text
TimeBox.sleep
SoundBox playback/timing waits
SocketBox accept-loop polling wait
```

These routes preserve the old blocking behavior while keeping OS sleep
ownership under the runtime thread substrate.

## Benchmark Claim Boundary

C pthread allocator benchmarks and replacement-front LD_PRELOAD tests are
allocator execution evidence only.

Use these readings in reports:

```text
benchmark_thread_origin=c_pthread
concurrency_surface_claim=not_applicable
hako_source_thread_support_claim=0
allocator_threading_evidence=c_side
nowait_os_thread_spawn=0
c_pthread_benchmark_hako_thread_support_claim=0
```

Runtime worker-pool or `ThreadApi` smokes may prove runtime substrate behavior.
They do not prove `.hako` source-level true parallel semantics unless a later
source-surface decision explicitly opens that route.

## Future Work

Future rows may add:

- capture safety / send-share capability;
- thread roots and GC/handle cleanup;
- context snapshot propagation into scheduler-owned structured child tasks;
- worker-pool execution route selection for eligible `nowait` tasks;
- explicit source-level worker syntax, if accepted later.

Until those rows land, the runtime thread substrate remains implementation
support, not a user-facing raw-thread API.
