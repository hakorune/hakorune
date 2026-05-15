## Concurrency Semantics

Status: concurrency semantics SSOT for the current Phase-0 / CONC line.
True parallel scheduling remains future work.

See also:
- `docs/reference/concurrency/boundary-model.md` (new Boundary-model design SSOT)
- `docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md` (implementation rows and compatibility archive rule)
- `docs/reference/concurrency/lock_scoped_worker_local.md` (`lock` / `scoped` / `worker_local` historical/provisional state-model notes)
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md` (VM+LLVM execution ledger and historical phase provenance)

## Implementation Status

Use this table as the quick read for "what works now". Use `CONC-*`
labels for current cross-doc status; historical `Phase 242x` style labels are
kept only as provenance in phase logs and execution ledgers.

| Feature | Design | VM | LLVM | Notes |
| --- | --- | --- | --- | --- |
| `nowait` / `await` | yes | yes | yes | CONC-1..4 done for Phase-0 future/await parity. |
| `Channel` | yes | scaffold | not active | Boundary model requires await-visible `send` / `recv` / `close`; runtime wait integration is later. |
| `co` / `task_scope` | yes | scaffold | scaffold | `co` is the preferred source spelling; `task_scope` remains compatibility/runtime wording. |
| `sync box` | yes | no | no | Parser/AST JSON capsule is active; verifier/runtime rows are later. |
| `lock<T>` | provisional | no | no | Implementation concept / historical design spelling; not the preferred canonical surface. |
| `context` / `scoped` | yes | no | no | `context` is the preferred surface name; `scoped` is historical/provisional wording. |
| `worker_local` | yes | no | no | Design-only/cache-only model; not a semantic mechanism. |
| true parallel scheduler | no | no | no | Phase-1+ future work; no detached-task contract yet. |

Mimalloc reading:
- This table is the user-facing concurrency surface status.
- Mimalloc migration reads allocator substrate requirements from
  `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`.
- Internal worker-local/TLS cache slots, atomics, OSVM, and thread-safe
  `hako_mem` ABI are runtime/backend substrate rows; they do not imply that
  source-level `worker_local`, `lock<T>`, channels, task scopes, or true
  parallel language semantics are active.

## CONC Vocabulary

| CONC id | Meaning | Current state |
| --- | --- | --- |
| CONC-0 | SSOT and drift inventory for async/concurrency docs. | done |
| CONC-1 | VM `FutureNew` / `Await` minimal support. | done |
| CONC-2 | Method-call `nowait` lowering unified to Phase-0 future wrapping. | done |
| CONC-3 | LLVM harness parity for Phase-0 futures. | done |
| CONC-4 | VM+LLVM smoke wiring. | done |

Terminology note:
- This doc uses “spawn” as a generic term for “starting a concurrent task”. The current Nyash surface syntax for async start is `nowait`.
- The structured-concurrency source surface should be read as `co`.
- `task_scope` is the compatibility spelling and semantic/runtime wording.
- The current runtime scaffold behind that scope is `TaskGroupBox` plus `push_task_scope()` / `pop_task_scope()`.
- `RoutineScopeBox` is historical wording only; do not use it as the current code name.

### Blocking & Non-Blocking
- `await ch.send(v)` waits when the buffer is full. It resumes when capacity is available or the channel is closed/cancelled.
- `await ch.recv()` waits when the buffer is empty. It resumes when a sender provides an item or close event.
- `ch.try_send(v) -> Bool`: returns immediately; false if full or closed.
- `ch.try_recv() -> Option<T>`: returns immediately; `None` when empty or closed-without-buffered-items.
- timeout-shaped receive is reserved for a later API row; do not introduce a hidden blocking ordinary call.

Implementation note (Phase-0): no busy loop. Use cooperative queues; later replaced by OS-efficient wait.

### Close Semantics
- `await ch.close()` marks the channel closed and wakes current waiters. It is await-visible for API consistency even if a runtime can complete it immediately.
- Further `send` is an error or an explicit closed result in fallible APIs. Silent drop is forbidden.
- After close, `recv` continues to drain buffered items; once empty, returns the closed result shape.
- Double close is an error unless a later API explicitly introduces `try_close`.

Implementation naming note:
- The existing Rust `src/core/channel_box.rs` `ChannelBox` is an older P2P/arrow communication box, not the canonical `Channel<T>` await-visible queue API described here.
- `src/lib.rs` currently exports that legacy `ChannelBox` / `MessageBox`, and `src/boxes/box_trait.rs` lists `ChannelBox` as a builtin type name; those facts do not make it the `Channel<T>` queue contract.
- `CONC-CHANNEL-*` rows must not reinterpret that P2P box as the new task boundary without an explicit migration row.

### Select Semantics
- `when(channel, handler)` registers selectable cases.
- `await()` examines cases:
  - If one or more ready: choose one (random/round‑robin) and execute its handler atomically.
  - If none ready: Phase-0 may block via a multi-wait helper or return false if non-blocking policy requested.
- Fairness: no starvation requirement; Phase-0 uses simple fairness; Phase-2 integrates runtime help.

### Structured Concurrency (`co`, Phase-0 scaffold)
- `co` is the preferred user-facing structured-concurrency boundary.
- `task_scope` is accepted as a compatibility spelling and remains the
  semantic/runtime wording in existing implementation notes.
- `TaskGroupBox` is the current runtime-side owner for child futures registered under that scope.
- Current lifecycle vocabulary is `cancelAll()` and `joinAll(timeout_ms)`, plus best-effort bounded join on scope exit.
- current scope exit is structured shutdown for the popped explicit scope:
  - cancel pending child futures as `scope-exit-cancelled`
  - then bounded-join that same scope
- after child cancellation/join, lexical cleanup handlers run, then local drops,
  then object `fini()` only if ownership actually ends
- current explicit scope-exit path now also surfaces that scope's latched `first_failure`
- current `joinAll(timeout_ms)` path now surfaces that same latched first failure as `ResultBox::Err(first_failure_payload)`
- object `fini()` aggregate surfacing remains later-phase work; cleanup
  ordering is defined by `docs/reference/language/scope-exit-semantics.md`.
- bare `nowait` is not detached.
- `nowait` inside explicit `co` / compatibility `task_scope` belongs to that scope.
- `nowait` outside explicit `co` / compatibility `task_scope` falls back to an implicit root scope owned by the runtime hooks registry.
- detached work is still reserved for a later explicit surface; do not treat the current implicit root scope as detached-task semantics.
- first failure in an explicit `co` / compatibility `task_scope` cancels pending siblings with reason `sibling-failed`.
- late child futures registered after that first failure are immediately cancelled with the same reason.
- implicit root scope does not participate in sibling-failure cancellation in the current cut.
- aggregate/multi-failure reporting is now a separate owner-side diagnostic surface:
  - `TaskGroupBox.failureReport()` returns `ArrayBox`
  - report order is `[first_failure, additional_failures...]`
  - `joinAll()` and scope exit still surface only the first failure
- Cancellation should eventually unblock channel waits promptly; Phase-0 only guarantees best-effort scope-owned future cleanup.
- Structured child tasks created by `nowait` inside an explicit `co` scope
  inherit the active `context`/historical `scoped` bindings by snapshot at child
  creation time. The implicit root scope remains best-effort ownership only and
  is not a detached context propagation contract.

### Serialized Shared State (`sync box`, parser/verifier capsule)
- `sync box` is the preferred shared-mutable source surface.
- Current implementation accepts and preserves the `sync box` AST/AST-JSON
  capsule, but serialized method entry is not runtime-active yet.
- Sync methods reject `await` and `nowait` during parser-side declaration
  validation. Because canonical Channel waits are await-visible
  (`await ch.send`, `await ch.recv`, `await ch.close`), this also keeps channel
  waits out of sync methods in the current surface.
- Program JSON and MIR lowering fail-fast rather than treating `sync box` as an
  ordinary `box`.

### Future `await` (current VM contract)
- This document is not the canonical owner for `await fut` failure/cancel semantics.
- The current `await fut` contract is pinned by `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`.
- Current VM `await` is narrow:
  - `await` requires a `Future` operand
  - a non-`Future` operand is a fail-fast type error
  - a failed future surfaces as `TaskFailed(error)`
  - a scope-cancelled future surfaces as `Cancelled(reason)`
  - there is no timeout result shape in the VM-side contract yet
- Current taxonomy is intentionally small:
  - `ContractError`
  - `TaskFailed(error)`
  - `Cancelled(reason)` for scope-owned pending futures only
- `task_scope.cancelAll()` is the current runtime API name for a narrow future-owner cut:
  - it cancels owned pending futures with reason `scope-cancelled`
  - late child futures registered after that cancellation are immediately cancelled with the same reason
  - it does not yet define interruption for arbitrary blocking APIs
- scope exit is a separate narrow future-owner cut:
  - explicit-scope exit cancels still-pending owned futures with reason `scope-exit-cancelled`
  - nested explicit scopes clean up when they exit; they are not deferred to the outermost scope
  - explicit scope exit now returns/rethrows the popped scope's latched `first_failure`
  - explicit scope-exit timeout still has no dedicated public payload in the current cut
- `joinAll()` is now a slightly wider owner-side surface:
  - `ResultBox::Ok(void)` when the bounded join finishes in time and no first failure is latched
  - `ResultBox::Err(first_failure_payload)` when a first failure is latched
  - `ResultBox::Err(TaskJoinTimeout: timed out after Nms)` when the bounded join deadline is hit without a latched first failure
  - first failure wins over timeout
- sibling-failure policy is owned by the structured-concurrency section above; `await` only observes the resulting `Cancelled(reason)` state.
- `FutureBox` success is single-assignment in the current contract:
  - once a future is `Ready`, later `set_result` / failed / cancelled writes are ignored
- plugin/runtime timeout is not part of the VM-side `await` contract:
  - `env.future.await` may still surface `ResultBox::Err("Timeout")`
  - MIR `Await` does not currently expose a timeout result shape

### Root-scope note
- The current implicit root scope is best-effort ownership only.
- It exists so top-level `nowait` / `FutureNew` paths have an owner even when
  no explicit `co` / compatibility `task_scope` is open.
- It does not currently promise lexical join, sibling-failure cancellation, or a detached-task shutdown contract.

### Types & Safety
- Phase-0 docs describe `Channel<T>` as a typed ownership-transfer boundary.
- Runtime tag checks for a future queue implementation are optional until a runtime row lands, but docs/examples must still use the `Channel<T>` API shape.
- Future: static verification; fall back to runtime tags only when the implementation row explicitly allows it.

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
- close_semantics.hako: send after close -> error; drain -> closed result shape; double close -> error.
- scope_cancel.hako: `co` / compatibility `task_scope` plus `TaskGroupBox` cancels owned child futures with stable reasons; blocking-API interruption remains a later runtime wait contract.

### Migration Path
- Phase-0 userland boxes are kept while Phase-2 runtime grows; API stable.
- Replace cooperative wait with WaiterBox (Phase-1) and runtime park/unpark later.
