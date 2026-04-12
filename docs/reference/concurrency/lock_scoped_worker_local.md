# Lock / Scoped / WorkerLocal (Concurrency SSOT)

Status: Draft (docs-only; no runtime changes during the feature‑pause)
Decision: provisional

This document defines the **minimum** concurrency-related state model for Nyash/Hakorune.
It is designed to be:
- explicit (no hidden thread semantics),
- safe by default (no silent fallback),
- fast (C++ backends can map 1:1 to primitives),
- consistent with the “Box boundary + SSOT” philosophy.

Related SSOT:
- `docs/reference/language/variables-and-scope.md` (locals / lexical scope)
- `docs/reference/concurrency/semantics.md` (channels / `task_scope` / structured concurrency)
- Pre-selfhost execution plan (VM+LLVM): `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`

---

## 1) The 4 kinds of “local”

### 1. `local` (lexical locals)
- Purpose: normal variables / temporaries.
- Lifetime: lexical block (`{ ... }`) and call activation.
- Concurrency: **thread-irrelevant**. Each routine/task has its own activation/locals.

### 2. `lock<T>` (shared mutable state; 99% solution)
- Purpose: the **only** endorsed way to share **mutable** state across routines.
- Ownership: the wrapped value is conceptually shared; mutation requires a lock scope.
- Performance: intended to lower to a single mutex + RAII guard in native backends.

### 3. `scoped` (context values; meaning/diagnostics)
- Purpose: request-id / trace / config / “meaningful context” that must not leak.
- Lifetime: dynamic scope; **always** restored when leaving the block.
- Concurrency: inherited by child routines under structured concurrency.

### 4. `worker_local` (cache; performance-only)
- Purpose: allocator caches / scratch buffers / per-worker statistics.
- Lifetime: worker lifetime (often OS thread).
- Concurrency: **not** a semantic mechanism; values may survive across unrelated tasks.

---

## 2) `lock<T>`: the only mutable-sharing contract

### 2.1 Surface model
The spec uses the generic spelling `lock<T>` as a concept. The concrete box/type name is an implementation detail.

The core rule:
- **All shared mutation happens inside a `lock` scope.**

### 2.2 `lock` scope (syntax is provisional)

```
lock m {
  // mutate m's payload
}
```

Rules:
1. `m` must be a `lock<T>` instance, otherwise it is a compile error (or fail-fast in strict/dev).
2. Lock is acquired at block entry and released at block exit (including exceptions).
3. The “guard” does not exist as a value (cannot escape).
4. Re-entrancy is not guaranteed (assume non‑reentrant).
5. No fairness guarantee.

### 2.3 Fail-fast safety rules (strict/dev)
Inside a `lock` scope:
- `await` / `nowait` / `yield` / blocking calls are forbidden (deadlock + scheduling hazards).
- Finalizers (`fini` / cleanup handlers) must not acquire locks (to avoid lock-order traps).

Violations are **errors**, not silent fallback.

Note:
- This doc uses “spawn” as a generic term for “starting a concurrent task”, but the current Nyash surface syntax is `nowait`.

---

## 3) `scoped`: task-local context (ThreadLocal alternative)

### 3.1 Surface model (syntax is provisional)

```
with scoped RequestId = rid {
  handle()
}
```

### 3.2 What belongs in `scoped` (SSOT boundary)
`scoped` is for *ambient, cross-cutting context* that is useful across deep call chains
without turning everything into “pass req_id everywhere” plumbing.

Allowed (recommended):
- trace/span/correlation id
- request id / tenant id (identifiers / read-only context)
- read-only config snapshots (timeouts, flags, mode)

Forbidden (do not use `scoped` for these):
- shared mutable state (Maps/Vectors/etc. that you mutate across calls)
- locks / mutex guards
- resources (file handles, sockets, allocator-owned buffers)
- large payloads that would encourage implicit data flow

Rule of thumb:
- If it can affect *program correctness* when accidentally inherited, it does not belong in `scoped`.
- If it is purely *observability / context*, it belongs in `scoped`.

### 3.3 Inheritance to child tasks
`scoped` values are intended to be inherited by **structured** child tasks.

SSOT rule (provisional):
- A child task started inside `task_scope` inherits the parent’s active `scoped` bindings.
- Current runtime scaffolding names that boundary with `TaskGroupBox` plus task-scope hooks; full child-scheduling wiring is a later phase.

Note:
- This doc uses “spawn” as a generic term for “starting a concurrent task”, but the current Nyash surface syntax is `nowait`.

Rules:
1. Scope entry binds a key/value; scope exit restores the previous binding.
2. A `scoped` value must not be persisted as “state”; it is context only.
3. Structured child tasks inherit the active bindings under `task_scope` (current runtime scaffold: `TaskGroupBox`).
4. Current `task_scope.cancelAll()` is narrow: it marks owned pending futures as cancelled, but it does not define general blocking-call interruption yet.

Notes:
- `scoped` is intended to replace ThreadLocal-style “request context” without leakage.

---

## 4) `worker_local`: TLS-like cache isolation

`worker_local` is intentionally separated from language meaning:
- It may be implemented with TLS (`__thread`, `thread_local`) or per-worker arrays.
- It is **not** inherited by tasks/routines (tasks can move between workers).

Allowed use:
- allocator thread cache
- scratch buffers
- per-worker counters

Forbidden use:
- request-id / trace / user-visible semantics
- anything that must not leak across tasks

If/when exposed to userland, access must be explicitly “pinned” to a worker (`pin { ... }`-style),
otherwise it remains runtime/internal only.
