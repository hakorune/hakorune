## Concurrency Semantics (docs-only; to guide MVP)

Status: reference for userland Phase‑0. No runtime changes during the feature‑pause.

See also:
- `docs/reference/concurrency/lock_scoped_worker_local.md` (Lock/Scoped/WorkerLocal SSOT)

Terminology note:
- This doc uses “spawn” as a generic term for “starting a concurrent task”. The current Nyash surface syntax for async start is `nowait`.
- The structured-concurrency surface should be read as `task_scope`.
- The current runtime scaffold behind that scope is `TaskGroupBox` plus `push_task_scope()` / `pop_task_scope()`.
- `RoutineScopeBox` is historical wording only; do not use it as the current code name.

### Blocking & Non-Blocking
- `send(v)` blocks when the buffer is full (capacity reached). Unblocks when a receiver takes an item.
- `receive()` blocks when buffer is empty. Unblocks when a sender provides an item.
- `try_send(v) -> Bool`: returns immediately; false if full or closed.
- `try_receive() -> (Bool, Any?)`: returns immediately; false if empty and not closed.
- `receive_timeout(ms) -> (Bool, Any?)`: returns false on timeout.

Implementation note (Phase‑0): no busy loop. Use cooperative queues; later replaced by OS-efficient wait.

### Close Semantics
- `close()` marks the channel closed. Further `send` is an error.
- After close, `receive` continues to drain buffered items; once empty, returns a closed indicator (shape per API, e.g., `(false, End)`).
- Double close is an error.

### Select Semantics
- `when(channel, handler)` registers selectable cases.
- `await()` examines cases:
  - If one or more ready: choose one (random/round‑robin) and execute its handler atomically.
  - If none ready: Phase‑0 may block via a multi-wait helper or return false if non-blocking policy requested.
- Fairness: no starvation requirement; Phase‑0 uses simple fairness; Phase‑2 integrates runtime help.

### Structured Concurrency (`task_scope`, Phase-0 scaffold)
- `task_scope` is the user-facing structured-concurrency boundary.
- `TaskGroupBox` is the current runtime-side owner for child futures registered under that scope.
- Current lifecycle vocabulary is `cancelAll()` and `joinAll(timeout_ms)`, plus best-effort bounded join on scope exit.
- `fini()`, detached tasks, sibling-failure aggregation, and root-scope policy remain later-phase work.
- Cancellation should eventually unblock channel waits promptly; Phase-0 only guarantees best-effort scope-owned future cleanup.

### Future `await` (current VM contract)
- This document is not the canonical owner for `await fut` failure/cancel semantics.
- The current `await fut` contract is pinned by `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`.
- Current VM `await` is narrow:
  - `await` requires a `Future` operand
  - a non-`Future` operand is a fail-fast type error
  - there is no timeout or cancellation result shape yet
- `task_scope.cancelAll()` does not yet define a user-visible `await` interruption contract.

### Types & Safety
- Phase‑0: runtime tag checks on `ChannelBox` send/receive are optional; document expected element type.
- Future: `TypedChannelBox<T>` with static verification; falls back to runtime tags when needed.

### Observability
- Enable `NYASH_CONC_TRACE=1` to emit JSONL events.
- Event schema (recommendation):
  - Common: `ts` (ms), `op` (string), `rid` (routine id), `cid` (channel id, optional), `ok` (bool), `scope` (scope id, optional)
  - spawn/start/join/cancel: `{op:"spawn", rid, ts}`, `{op:"join", rid, ok, ts}`
  - send/recv: `{op:"send", cid, size, cap, ts}`, `{op:"recv", cid, size, cap, ts}`
  - park/unpark: `{op:"park", rid, reason, ts}`, `{op:"unpark", rid, reason, ts}`
  - select: `{op:"select", cases:n, chosen:k, ts}`
  - close: `{op:"close", cid, ts}`
- Causality: producers must emit before consumers observe; timestamps monotonic per process.

### Test Plan (smokes)
- ping_pong.hako: two routines exchange N messages; assert order and count.
- bounded_pc.hako: producer/consumer with capacity=1..N; ensure no busy-wait and correct totals.
- select_two.hako: two channels; verify first-ready choice and distribution.
- close_semantics.hako: send after close -> error; drain -> End; double close -> error.
- scope_cancel.hako: `task_scope` / `TaskGroupBox` cancels children; parked receivers unblocked.

### Migration Path
- Phase‑0 userland boxes are kept while Phase‑2 runtime grows; API stable.
- Replace cooperative wait with WaiterBox (Phase‑1) and runtime park/unpark later.
