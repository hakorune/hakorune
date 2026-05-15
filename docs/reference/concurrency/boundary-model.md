# Concurrency Boundary Model

Status: SSOT
Decision: accepted direction; implementation is phased
Scope: user-facing concurrency surface design and verifier/CorePlan ownership model.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`

## Decision

Hakorune concurrency is organized around explicit task boundaries.

The guiding rule is:

```text
Do not cross task boundaries without an explicit boundary.
```

This keeps the Go-style preference for ownership transfer, but does not force
all shared state through channels. Low-level allocator and runtime substrate may
use atomics, TLS/worker-local slots, and internal mutexes without opening those
as user-facing language semantics.

## Boundary Kinds

| Boundary | Source role | Meaning |
| --- | --- | --- |
| `Future<T>` | one-shot result | A task result is observed exactly through `await`. |
| `Channel<T>` | ownership transfer | Values cross task boundaries through an await-visible queue API. |
| `sync box` | serialized shared state | Shared mutable state is accessed only through serialized method boundaries. |
| `context` | structured ambient context | Request/trace/read-only config context is inherited by structured child tasks. |
| worker-local substrate | runtime/internal cache only | Allocator caches, scratch state, and per-worker counters; not user semantics. |

Do not collapse these names into a single source-level `Boundary<T>` type.
The compiler/verifier/CorePlan may use a shared boundary model internally, but
source code should keep the role visible.

## Source Surface Direction

Canonical surface direction:

```text
task_scope
nowait { ... }
await expr
Channel<T>
sync box
context
```

Non-canonical or deferred public surface:

```text
lock<T>
scoped
worker_local
Atomic<T>
Mutex
Thread
Worker
true_parallel
```

`lock<T>` remains a useful implementation concept, but it should not become the
canonical user-facing shared-mutable surface. The preferred surface is
`sync box`, because it exposes a serialized object boundary rather than a raw
guard.

`scoped` is the historical/provisional name for context. The preferred surface
name is `context`, because the feature is about structured ambient context, not
task spawning or detached execution.

`worker_local` remains runtime/internal unless a later explicit language row
opens a pinned worker-local surface. Mimalloc work must continue to use the
allocator substrate split, not source-level worker-local syntax.

## Future Boundary

`nowait { ... }` creates a `Future<T>`. In the current Phase-0 line it may be
implemented as sequential evaluation wrapped in a resolved future; it is not a
thread creation promise.

`await fut` is the only way to observe the future result.

Blocking or potentially blocking APIs should be await-visible. A hidden wait in
ordinary-looking code is not the preferred Hakorune surface.

## Channel Boundary

`Channel<T>` is a type/API surface, not a keyword. It exists for ownership
transfer between tasks.

Preferred API shape:

```hako
await jobs.send(job)
local next = await jobs.recv()
await jobs.close()

local maybe = jobs.try_recv()
local ok = jobs.try_send(job)
```

Decision:

- `send` is awaitable because it may wait for capacity.
- `recv` is awaitable because it may wait for an item or close event.
- `close` is also awaitable for consistency, even if an implementation can
  complete it immediately.
- `try_send` / `try_recv` are non-await APIs and must not block.

Close contract:

- `await ch.close()` marks the channel closed and wakes all current waiters.
- After close, new `send` attempts are fail-fast errors or return an explicit
  closed result shape for fallible APIs. Silent drop is forbidden.
- After close, `recv` drains already-buffered items first.
- Once the buffer is empty, `recv` returns the channel-closed result shape.
- Double close is a fail-fast error unless a later API explicitly introduces an
  idempotent `try_close`.
- Cancellation of a task waiting in `send` / `recv` / `close` is owned by the
  structured runtime wait contract; Phase-0 may keep this as a future runtime
  row, but docs must not pretend the wait is detached.

`Channel<T>` is not the allocator remote-free queue model. Allocator remote-free
queues are allocator-owned structures over atomic/TLS substrate.

## Sync Box Boundary

`sync box` is the preferred source surface for shared mutable state.

Example:

```hako
sync box Counter {
    value: i64 = 0

    inc(delta: i64): void {
        me.value += delta
    }

    get(): i64 {
        return me.value
    }
}
```

Meaning:

- `sync box` is an identity object with serialized public method entry.
- Its stored fields are shared state owned by the object.
- Public method bodies run under the sync boundary.
- Guards are not first-class values and cannot escape.
- Re-entrancy and fairness are not promised unless a later row explicitly adds
  them.

Forbidden inside a `sync box` serialized method:

- `await`
- `nowait`
- channel `send` / `recv` / `close` waits
- blocking calls
- lock/guard acquisition that can create lock-order cycles

Initial rule:

- Calling another `sync box` method while inside a serialized `sync box` method
  should be rejected unless a later verifier row introduces an explicit,
  acyclic lock-order contract.

This is stricter than exposing `lock<T>` directly, but it gives the compiler a
clear boundary for verifier facts, effect checks, and backend lowering.

## Context Boundary

`context` is the preferred source name for structured ambient context.

Example:

```hako
context request_id: RequestId = rid {
    task_scope {
        local fut = nowait {
            handle(request_id)
        }
        await fut
    }
}
```

Decision:

- Structured child tasks created by `nowait` inside an explicit `task_scope`
  inherit the parent active `context` bindings.
- The inheritance snapshot is taken when the child task is created.
- Context values are restored at block exit.
- Detached work is not part of the current surface; the implicit root scope is
  not a context propagation promise.

Allowed context payloads:

- request id
- trace/span/correlation id
- tenant id
- read-only config snapshots

Forbidden context payloads:

- shared mutable state
- lock guards or sync-box guards
- file/socket/resource handles
- allocator-owned buffers
- large payloads that make implicit data flow hard to audit

If accidental inheritance would change program correctness, the value does not
belong in `context`.

## Stage Ownership

Stage0 / runtime kernel owns substrate:

```text
Future runtime primitives
atomic operations
TLS / worker-local slots
internal mutex/wait primitives
OSVM
thread-safe ABI
```

Stage1 owns language semantics:

```text
nowait / await
task_scope ownership and cancellation
Channel<T> API meaning
sync box no-await/no-block verifier rules
context inheritance
```

MIR metadata / CorePlan owns route contracts:

```text
boundary facts
await safepoints
sync method serialization plans
channel wait routes
context snapshot/restore facts
allocator substrate routes
```

Backend owns lowering:

```text
future lowering
channel wait/notify lowering
sync method entry lowering
atomic/TLS/OSVM lowering
native wait/mutex calls
```

VM remains a reference/proof lane. True parallel scheduling is a later surface
and must not be inferred from Phase-0 `nowait` or allocator substrate stress.

## Migration Tasks

The migration should be cut into docs-first rows:

| Row | Purpose |
| --- | --- |
| `CONC-BOUNDARY-001` | Adopt this Boundary model as the concurrency design SSOT. |
| `CONC-SYNCBOX-001` | Move `lock<T>` from canonical surface to implementation concept; define `sync box` verifier rules. |
| `CONC-CHANNEL-001` | Update channel API docs so `send` / `recv` / `close` are await-visible and `try_*` APIs are non-blocking. |
| `CONC-CONTEXT-001` | Rename/design `scoped` as `context` and pin structured child inheritance. |

Implementation must remain separate from mimalloc substrate rows. Mimalloc may
continue using runtime/internal atomics, TLS slots, worker identity, and
thread-safe ABI routes without opening the user-facing concurrency surface.
