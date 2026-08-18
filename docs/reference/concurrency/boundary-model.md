# Concurrency Boundary Model

Status: SSOT
Decision: accepted direction; implementation is phased
Scope: user-facing concurrency surface design and verifier/CorePlan ownership model.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md`
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

## Public Roles and Surfaces

The final public model has four roles and seven visible surfaces. The four
task-ownership surfaces are intentionally counted separately because a result
type, task start, result observation, and lexical owner are different jobs.

| Role | Canonical surface | Meaning |
| --- | --- | --- |
| task ownership | `Future<T>`, `nowait`, `await`, `co` | Start, own, and observe one-shot task results. |
| ownership transfer | `Channel<T>` | Values cross task boundaries through an await-visible queue API. |
| serialized shared state | `sync box` | Shared mutable state is accessed only through serialized method boundaries. |
| structured ambient context | `context` | Request/trace/read-only config context is inherited by structured child tasks. |

Do not collapse these names into a single source-level `Boundary<T>` type.
The compiler/verifier/CorePlan may use a shared boundary model internally, but
source code should keep the role visible.

Worker-local/TLS storage is runtime substrate, not a fifth public role.

## Source Surface Direction

Canonical surface direction:

```text
Future<T>
co { ... }
nowait expr
await expr
Channel<T>
sync box
context
```

Retired/rejected source spellings and implementation-only terms:

```text
lock<T>
scoped
task_scope
worker_local
Atomic<T>
Mutex
Thread
Worker
true_parallel
```

Structured parallel iteration remains a future language problem, but
`worker_scope` and `parallel` are not reserved as its final spellings. A later
Decision must choose the source shape only after Send / Share / ThreadRoot and
capture safety are enforced. Raw `thread { ... }` remains rejected source.

`lock<T>` remains a useful implementation concept, but it should not become the
canonical user-facing shared-mutable surface. The preferred surface is
`sync box`, because it exposes a serialized object boundary rather than a raw
guard.

`TaskScope`, `TaskGroupBox`, `push_task_scope`, and `pop_task_scope` remain
valid compiler/runtime names. The source spelling `task_scope { ... }` is a
temporary compatibility input on the way to `co`-only Canonical source.

`scoped` is the historical/provisional name for context and is a temporary
compatibility input only. The canonical surface name is `context`, because the
feature is about structured ambient context, not task spawning or detached
execution.

`worker_local` remains runtime/internal unless a later explicit language row
opens a pinned worker-local surface. Mimalloc work must continue to use the
allocator substrate split, not source-level worker-local syntax.

## Co Scope Boundary

`co { ... }` is the canonical source spelling for a structured concurrency
scope.

Example:

```hako
co {
    local a = nowait workA()
    local b = nowait workB()

    local x = await a
    local y = await b

    return x + y
}
```

This is the accepted final surface, not a claim that every row is executable
on the current Phase-0 parser/lowerer. Current `nowait` binding syntax and the
normal-completion-only `co` exit restriction are recorded in `semantics.md`.

Meaning:

- `co` is a child-`Future` ownership boundary.
- `nowait` children created inside the block belong to that `co` scope.
- On scope exit, pending children are cancelled and joined according to the
  structured-concurrency runtime contract.
- The first child failure is surfaced by the scope.
- `co` does not guarantee true parallel execution.

Negative definitions:

- `co` is not detached work.
- `co` is not an OS thread guarantee.
- `co` is not channel `select`.
- `co` is not a scheduler/fairness guarantee.
- `co` is not a replacement for `nowait`; it owns child futures created inside.

Final and transitional spelling split:

```text
source canonical: co
temporary Compat2025 input: task_scope
semantic wording: structured concurrency scope / co scope
runtime owner: TaskGroupBox
runtime hooks: push_task_scope / pop_task_scope
```

Canonical diagnostics must reject the old spelling and guide it toward `co`.
Compat2025 may normalize it only until the compatibility sunset row closes:

```text
[concurrency/scope-compat]
`task_scope` is a compatibility spelling.
Use `co { ... }` for the canonical structured concurrency scope.
```

## Future Structured Parallel Boundary

Structured parallel iteration is a future design area, not an active or
reserved source grammar. The historical `worker_scope` / `parallel` sketch is
design vocabulary only; its exact spelling is undecided.

Required meaning before any later spelling is selected:

- One explicit scope owns the worker/parallel lifecycle.
- Captures must pass Send / Share / ThreadRoot safety checks.
- Any worker budget is an upper bound, not an exact OS-thread-count promise.
- The runtime may choose fewer/equivalent workers only with explicit report
  evidence; silent fallback is forbidden once this surface is source-visible.

Current status:

```text
structured_parallel_design_area=1
structured_parallel_exact_spelling_decided=0
worker_scope_keyword_reserved=0
parallel_keyword_reserved=0
worker_scope_parser_enabled=0
worker_scope_mir_lowering_enabled=0
worker_scope_runtime_route_enabled=0
```

Opening parser or lowering support requires the thread-safety gate:

```text
hako_send_share_enforced=1
thread_registry_gc_roots_enabled=1
worker_scope_capture_check_enabled=1
```

Raw `thread { ... }` remains permanently rejected as ordinary source. A future
expert substrate, if needed, must be a separately authorized capability API;
it does not reserve a `thread` block grammar today.

## Future Boundary

The accepted final surface is expression-shaped: `nowait expr` creates a
`Future<T>`. The current parser still accepts the historical binding statement
`nowait name = expr`; `CONC-NOWAIT-EXPR-D0/I0` must migrate that shape without
moving ordinary local-binding ownership into the async lowerer. In either
shape, Phase-0 may use sequential evaluation wrapped in a resolved future; it
is not a thread creation promise.

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

> Reference-only example: parser/AST transport and verifier checks are active,
> but Program JSON, MIR, Rust VM execution, and LLVM lowering remain fail-fast.

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
- `Decision: provisional` — the first Home concurrency profile rejects a
  receiver/field-anchored result handle from a `sync box` method and any
  escaping handle into a `sync box` field. A method guard ends at return, so it
  cannot silently keep that handle synchronized. APIs must return a snapshot
  or independently owned result until a separate synchronized-handle contract
  is designed.

This is stricter than exposing `lock<T>` directly, but it gives the compiler a
clear boundary for verifier facts, effect checks, and backend lowering.

## Context Boundary

`context` is the preferred source name for structured ambient context.

Example:

```hako
context request_id: RequestId = rid {
    co {
        local fut = nowait handle(request_id)
        await fut
    }
}
```

Decision:

- Structured child tasks created by `nowait` inside an explicit `co` scope
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
co / task_scope ownership and cancellation
co / task_scope compatibility facts
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

The implementation taskboard SSOT is:

```text
docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md
```

Summary rows:

| Row | Purpose |
| --- | --- |
| `CONC-BOUNDARY-001` | Adopt this Boundary model as the concurrency design SSOT. |
| `CONC-COMPAT-001` | Audit legacy spellings and archive smoke-only compatibility users. |
| `CONC-CO-001` | Add `co` as the canonical source spelling for structured concurrency scope. |
| `CONC-CHANNEL-001` | Update channel API docs so `send` / `recv` / `close` are await-visible and `try_*` APIs are non-blocking. |
| `CONC-SYNCBOX-001` | Keep raw `lock<T>` non-canonical; add `sync box` as the shared-mutable surface. |
| `CONC-SYNCBOX-002` | Reject `await` / `nowait` / channel waits inside serialized `sync box` methods. |
| `CONC-GUARD-AST-CRATE0` | Repair stale guard paths without changing language behavior. |
| `CONC-GRAM-SYNC0/CO0/CONTEXT0` | Add the already-live concurrency capsules to grammar registry and EBNF one row at a time. |
| `CONC-NOWAIT-EXPR-D0/I0` | Move the historical dedicated binding statement to the canonical `nowait expr -> Future<T>` expression without changing scheduler meaning. |
| `CONC-SCOPED-COMPAT-R0` / `CONC-TASK-SCOPE-COMPAT-R0` | Reject legacy spellings in Canonical, quarantine them to Compat2025, and delete their spelling carriers only at the compatibility sunset. |
| `CONC-RUNTIME-DOCS-OWNER-R0` | Keep public semantics, current availability, and runtime threading substrate in three separate owner docs. |
| `CONC-CHANNELBOX-DISPOSITION-D0/R0` | Retire the caller-zero legacy P2P `ChannelBox` only after its public Rust/type-name compatibility decision. |
| `CONC-SYNCBOX-VIEW-D0` | Seal the initial no-escaping-View boundary for serialized methods. |
| `CONC-CONTEXT-001` | Rename/design `scoped` as `context` and pin structured child inheritance. |

Implementation must remain separate from mimalloc substrate rows. Mimalloc may
continue using runtime/internal atomics, TLS slots, worker identity, and
thread-safe ABI routes without opening the user-facing concurrency surface.
