# Lock / Scoped / WorkerLocal (Concurrency State-Model SSOT)

Status: SSOT for state model; runtime surface is phased
Decision: provisional

This document defines the **minimum** concurrency-related state model for Nyash/Hakorune.
It is designed to be:
- explicit (no hidden thread semantics),
- safe by default (no silent fallback),
- fast (C++ backends can map 1:1 to primitives),
- consistent with the “Box boundary + SSOT” philosophy.

Related SSOT:
- `docs/reference/concurrency/boundary-model.md` (new canonical concurrency Boundary model)
- `docs/reference/language/variables-and-scope.md` (locals / lexical scope)
- `docs/reference/concurrency/semantics.md` (current implementation status, `nowait` / `await`, `task_scope`, channels)
- Pre-selfhost execution plan (VM+LLVM): `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- Mimalloc allocator substrate cut:
  `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`

---

## 1) The 4 kinds of “local”

### 1. `local` (lexical locals)
- Purpose: normal variables / temporaries.
- Lifetime: lexical block (`{ ... }`) and call activation.
- Concurrency: **thread-irrelevant**. Each routine/task has its own activation/locals.

### 2. `sync box` / `lock<T>` (shared mutable state)
- Purpose: shared mutable state must be accessed only through an explicit
  serialized boundary.
- Preferred source surface: `sync box`.
- `lock<T>` reading: historical/provisional concept and possible implementation
  substrate, not the preferred canonical user-facing surface.
- Performance: intended to lower to a single mutex/serialized method-entry
  primitive in native backends.

### 3. `context` / `scoped` (context values; meaning/diagnostics)
- Purpose: request-id / trace / config / “meaningful context” that must not leak.
- Lifetime: dynamic scope; **always** restored when leaving the block.
- Concurrency: inherited by child routines under structured concurrency.
- Preferred source name: `context`. `scoped` is the historical/provisional name.

### 4. `worker_local` (cache; performance-only)
- Purpose: allocator caches / scratch buffers / per-worker statistics.
- Lifetime: worker lifetime (often OS thread).
- Concurrency: **not** a semantic mechanism; values may survive across unrelated tasks.

---

## 2) `sync box`: the mutable-sharing contract

### 2.1 Surface model
The new canonical direction is `sync box`, owned by
`docs/reference/concurrency/boundary-model.md`.

The core rule:
- **All shared mutation happens inside a serialized boundary.**
- User-facing shared mutable objects should be modeled as `sync box` methods.
- `lock<T>` remains an implementation concept or compatibility design spelling,
  not the preferred canonical source surface.

### 2.2 `sync box` method boundary

```
sync box Counter {
  value: i64 = 0

  inc(delta: i64): void {
    me.value += delta
  }
}
```

Rules:
1. Public `sync box` methods enter a serialized boundary.
2. The boundary is released at method exit, including failure paths.
3. The guard does not exist as a value and cannot escape.
4. Re-entrancy is not guaranteed (assume non‑reentrant).
5. No fairness guarantee.

### 2.3 Fail-fast safety rules (strict/dev)
Inside a serialized `sync box` method:
- `await` / `nowait` / `yield` / channel waits / blocking calls are forbidden (deadlock + scheduling hazards).
- Calling another `sync box` method is forbidden until a later verifier row defines an explicit acyclic order contract.
- Cleanup handlers and object finalizers (`fini()`) must not acquire locks (to avoid lock-order traps).

Violations are **errors**, not silent fallback.

---

## 3) `context`: task-local context (ThreadLocal alternative)

### 3.1 Surface model (syntax is provisional)

```
context request_id: RequestId = rid {
  handle()
}
```

### 3.2 What belongs in `context` (SSOT boundary)
`context` is for *ambient, cross-cutting context* that is useful across deep call chains
without turning everything into “pass req_id everywhere” plumbing.

`scoped` remains historical/provisional wording for the same design area.

Allowed (recommended):
- trace/span/correlation id
- request id / tenant id (identifiers / read-only context)
- read-only config snapshots (timeouts, flags, mode)

Forbidden (do not use `context` / historical `scoped` for these):
- shared mutable state (Maps/Vectors/etc. that you mutate across calls)
- locks / mutex guards
- resources (file handles, sockets, allocator-owned buffers)
- large payloads that would encourage implicit data flow

Rule of thumb:
- If it can affect *program correctness* when accidentally inherited, it does not belong in `context`.
- If it is purely *observability / context*, it belongs in `context`.

### 3.3 Inheritance to child tasks
`context` values are inherited by **structured** child tasks.

Boundary rule:
- A child task started with `nowait` inside explicit `task_scope` inherits a
  snapshot of the parent’s active `context` bindings at child creation time.
- The implicit root-scope fallback is not a detached/task-local propagation contract.
- Concrete propagation wiring is still phased, but the semantic direction is pinned.
- Task ownership, `nowait` wording, `TaskGroupBox` observation APIs, and
  sibling-failure details are owned by `docs/reference/concurrency/semantics.md`.

Rules:
1. Scope entry binds a key/value; scope exit restores the previous binding.
2. A `context` value must not be persisted as “state”; it is context only.
3. Structured child tasks inherit active bindings under `task_scope`.
4. Current `task_scope.cancelAll()` is narrow: it marks owned pending futures as cancelled, but it does not define general blocking-call interruption yet.

Notes:
- `context` is intended to replace ThreadLocal-style request context without leakage.

---

## 4) `worker_local`: TLS-like cache isolation

`worker_local` is intentionally separated from language meaning:
- It may be implemented with TLS (`__thread`, `thread_local`) or per-worker arrays.
- It is **not** inherited by tasks/routines (tasks can move between workers).
- Allocator internals may use runtime/internal worker-local or TLS slots before
  source-level `worker_local` is exposed.

Allowed use:
- allocator thread cache
- scratch buffers
- per-worker counters

Forbidden use:
- request-id / trace / user-visible semantics
- anything that must not leak across tasks

If/when exposed to userland, access must be explicitly “pinned” to a worker (`pin { ... }`-style),
otherwise it remains runtime/internal only. Mimalloc migration uses the
runtime/internal substrate side of this split; it does not open userland
`worker_local` syntax by itself.
