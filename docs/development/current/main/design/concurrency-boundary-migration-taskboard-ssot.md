# Concurrency Boundary Migration Taskboard

Status: SSOT
Scope: implementation rows for the concurrency Boundary model.

Related:
- `docs/reference/concurrency/boundary-model.md`
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`
- `docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md`
- `docs/reference/runtime/threading.md`

## Decision

The concurrency migration is implementation-first only after the Boundary model
is documented. The source surface should move toward:

```text
Future<T>
co
Channel<T>
sync box
context
```

Historical/provisional surfaces such as raw `lock<T>` and `scoped` may remain as
compatibility input only while active users are audited.
`task_scope` remains a compatibility spelling and runtime/semantic term; new
source examples should use `co`.

`lock<T>` must not be promoted to the canonical source surface. The canonical
shared-mutable surface is `sync box`; raw locks remain implementation concepts,
historical/provisional compatibility, or runtime/internal primitives.

## Runtime Substrate Side Lane

The runtime thread substrate already exists below the source surface:

```text
ThreadApi:
  sleep / yield_now / current_thread_id / spawn / join / detach

Scheduler:
  SingleThreadScheduler
  WorkerPoolScheduler

Future ownership:
  FutureBox
  TaskGroupBox
```

This does not open source-level true parallel semantics.

```text
nowait_os_thread_spawn=0
source_level_thread_syntax=0
worker_pool_source_route_enabled=0
lock_t_canonical_surface=0
sync_box_canonical_surface=1
```

When concurrency is prioritized before selfhost, first close the substrate
inventory and report/check vocabulary. Do not start with new `.hako` syntax.

Recommended substrate rows:

| Order | Row | Why now | Stop line |
| --- | --- | --- | --- |
| 1 | `CONC-RUNTIME-INVENTORY-001` | Sync docs with the implemented ThreadApi / WorkerPool / Future / TaskGroup substrate. | no behavior change |
| 2 | `CONC-SCHED-ROUTE-001` | Expose scheduler route vocabulary for `inline_resolved_future` / `cooperative_task` / `worker_pool_task`. | no default worker-pool activation |
| 3 | `CONC-CAP-INVENTORY-001` | Inventory send/share/thread-root safety before moving `.hako` values across workers. | report-only; no enforcement yet |
| 4 | `CONC-SYNCBOX-003` | Add serialized method-entry reference behavior for the canonical shared-mutable surface. | no fairness/reentrancy guarantee |
| 5 | `CONC-CHANNEL-002` / `003` | Implement the future `Channel<T>` queue runtime separately from legacy P2P `ChannelBox`. | no hidden blocking ordinary calls |

Only after these rows should a source-level `worker_scope` / `parallel` /
explicit worker surface be considered. `CONC-SOURCE-PARALLEL-001` reserves that
surface as design-only; parser, AST JSON, Program JSON, MIR, LLVM, and runtime
route activation stay closed until `THREAD-SAFETY-001` enforces send/share/root
safety.

## Recommended Task Order

Use this order when the language-surface cleanup is prioritized before
returning to the mimalloc lane:

| Order | Row | Why now | Stop line |
| --- | --- | --- | --- |
| 1 | `CONC-COMPAT-001` | Know which legacy spellings are active source vs smoke-only compatibility. | no parser/runtime deletion |
| 2 | `CONC-CO-001` | Make `co { ... }` the canonical structured-concurrency source spelling. | no `TaskGroupBox` / hook rename |
| 3 | `CONC-CHANNEL-001` | Pin the visible API shape before runtime wait work: `await send` / `await recv` / `await close`, `try_*` non-blocking. | no scheduler or wait runtime rewrite |
| 4 | `CONC-SYNCBOX-001` | Add the canonical shared-mutable syntax capsule. | no serialized runtime behavior |
| 5 | `CONC-SYNCBOX-002` | Add the important safety verifier: no `await` / `nowait` / channel wait inside serialized methods. | no lock-order inference |
| 6 | `CONC-CONTEXT-001` | Move the surface name from `scoped` toward `context`. | no propagation runtime |

After this cut, the user-facing concurrency surface is clean enough to return
to `MIMAP-022C`. Do not wait for full Channel runtime, `sync box` fairness,
context propagation, source-level `worker_local`, or true parallel language
semantics before resuming mimalloc work.

Allocator substrate rows are already tracked in
`docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`.
Those rows are separate from this taskboard and must not reopen user-facing
concurrency syntax.

## Compatibility Archive Rule

Yes: if an old compatibility surface is used only by smoke tests, legacy
fixture packs, or archived probes, it should be moved out of the active
language path while preserving traceability.

Archive is allowed only when all checks are true:

```text
1. no production/runtime/parser/backend owner requires the legacy spelling
2. no reference spec treats the legacy spelling as canonical
3. active examples can be rewritten to the canonical Boundary surface
4. remaining uses are smoke-only, legacy fixture-only, or archived probe-only
5. a guard or audit command records that no active non-smoke use remains
```

Archive is not allowed when:

```text
the spelling is still accepted as documented source syntax
the compatibility route is needed by Stage0/Stage1 bootstrap
the legacy path is the only coverage for a live runtime/backend behavior
the archive move would hide a fail-fast diagnostic regression
```

Archive protocol:

```text
1. add an audit row that lists active uses vs smoke/archive-only uses
2. rewrite active tests/examples to canonical syntax first
3. move smoke-only legacy fixtures under an archive/compat bucket, or rename
   them as legacy-compat fixtures
4. keep a short README/stub explaining the canonical replacement
5. add a no-active-use guard before deleting or quarantining parser/runtime code
```

Do not keep compatibility code just because a smoke still exercises it. If the
smoke is only proving historical input compatibility, move the smoke to the
compat/archive lane and let canonical smokes cover the live behavior.

## Implementation Rows

| Row | Status | Purpose | Output | Stop line |
| --- | --- | --- | --- | --- |
| `CONC-BOUNDARY-001` | landed-docs | Adopt Boundary model as design SSOT. | `docs/reference/concurrency/boundary-model.md` | no runtime change |
| `CONC-RUNTIME-INVENTORY-001` | landed-docs | Sync current implementation inventory for ThreadApi, WorkerPoolScheduler, FutureBox, and TaskGroupBox before source-level thread design. | `293x-1000-CONC-RUNTIME-INVENTORY-001-THREAD-SUBSTRATE-REALITY.md` | no behavior change |
| `CONC-SCHED-ROUTE-001` | landed-code | Pin scheduler route vocabulary and report/check fields for future worker-pool execution routes. | `src/runtime/scheduler_route.rs` + `293x-1001-CONC-SCHED-ROUTE-001-SCHEDULER-ROUTE-VOCABULARY.md` | no default worker-pool activation |
| `CONC-CAP-INVENTORY-001` | landed-code | Inventory HakoSend/HakoShare/ThreadRoot gaps before cross-worker value movement. | `thread_capability_inventory_report_fields()` + `293x-1002-CONC-CAP-INVENTORY-001-SEND-SHARE-THREAD-ROOT-GAPS.md` | no source semantics change |
| `CONC-FUTURE-SEM-001` | landed-code | Pin existing MIRBuilder `nowait` / `await` / `Future<T>` boundary and align MIR JSON producer with the already-supported reader opcodes before opening structured ownership lowering. | `293x-1035-CONC-FUTURE-SEM-001-MIRBUILDER-FUTURE-BOUNDARY.md` + `src/mir/builder/stmts/async_stmt.rs` + `src/runner/mir_json_emit/emitters/basic.rs` | no OS thread spawn semantics; no `co` ownership lowering |
| `CONC-CO-MIR-001` | implemented | Lower `co` / compatibility `task_scope` as explicit TaskGroup ownership events after the Future boundary is pinned. | `293x-CONC-CO-MIR-001-TASKGROUP-OWNERSHIP-LOWERING.md` + `src/mir/builder/stmts/task_scope_stmt.rs` + `co_task_scope_vm.sh` | v0 uses runtime hook calls and normal-completion-only lowering |
| `CONC-COMPAT-001` | landed-audit | Audit legacy concurrency spellings and smoke-only compatibility users. | `tools/checks/concurrency_boundary_surface_guard.sh` | no parser/runtime deletion |
| `CONC-CO-001` | landed-parser-json | Add `co` as canonical structured concurrency source spelling while keeping `task_scope` as compat/internal wording. | parser + AST JSON + Program JSON row | runtime hook lowering remains fail-fast |
| `CONC-CHANNEL-001` | landed-api-docs | Pin Channel API shapes around await-visible `send` / `recv` / `close`. | docs/reference + guard | no wait runtime rewrite |
| `CONC-CHANNEL-002` | landed-code | Implement `await ch.close()` semantics in the future `Channel<T>` queue runtime scaffold. | `src/runtime/channel_queue.rs` + `293x-1004-CONC-CHANNEL-002-REFERENCE-CLOSE-SEMANTICS.md` | no true parallel scheduler |
| `CONC-CHANNEL-003` | landed-code | Implement await-visible `send` / `recv` route shape as a fail-fast bridge. | `src/runtime/channel_route.rs` + `293x-1005-CONC-CHANNEL-003-AWAIT-VISIBLE-ROUTE-BRIDGE.md` | no hidden blocking ordinary call |
| `CONC-SYNCBOX-001` | landed-parser-json | Add `sync box` parser/AST capsule and canonical docs. | parse/AST JSON roundtrip guard + lowering fail-fast | no serialized runtime yet |
| `CONC-SYNCBOX-002` | landed-verifier | Add verifier rule: no `await` / `nowait` / channel wait inside `sync box` method. | parser-side fail-fast diagnostics guard | no lock-order inference |
| `CONC-SYNCBOX-003` | landed-code | Add reference-only serialized method-entry behavior. | `src/runtime/sync_box.rs` + `293x-1003-CONC-SYNCBOX-003-REFERENCE-SERIALIZED-ENTRY.md` | Program JSON / MIR / LLVM fail-fast continue |
| `CONC-CONTEXT-001` | landed-parser-json | Add `context` surface as canonical name and quarantine `scoped` as compat. | parser/AST JSON guard + scoped compat audit | no propagation runtime yet |
| `CONC-CONTEXT-002` | landed-code | Implement context snapshot on `nowait` child creation inside explicit `co` / compatibility `task_scope`. | `src/runtime/context_snapshot.rs` + `293x-1006-CONC-CONTEXT-002-CONTEXT-SNAPSHOT-REFERENCE.md` | implicit root is not detached propagation |
| `CONC-WORKERLOCAL-001` | pending | Keep `worker_local` source syntax closed while allocator substrate remains internal. | no-source-worker-local guard | no mimalloc behavior change |
| `CONC-SOURCE-PARALLEL-001` | landed-docs | Reserve the source-level worker/parallel surface while keeping parser/lowering closed. | `293x-CONC-SOURCE-PARALLEL-001-SOURCE-PARALLEL-SURFACE-FREEZE.md` + report vocabulary | `THREAD-SAFETY-001` required before parser/lowering; raw thread syntax closed |

## Row Details

### CONC-COMPAT-001

Audit targets:

```text
lock<T>
lock { ... }
scoped
with scoped
task_scope
Channel<T> blocking send/recv without await
Channel close() without await
worker_local source syntax
```

Classification:

| Class | Meaning | Action |
| --- | --- | --- |
| active source | used by docs/reference, lang examples, compiler fixtures, or non-archive apps | migrate to Boundary surface before quarantine |
| active guard | used by a current guard to prove live behavior | update guard to canonical surface first |
| smoke-only compat | used only by smokes/profiles/archive probes | move to compat/archive bucket |
| historical docs | landed card or archived note only | leave as historical, no live action |

Acceptance:

```text
rg-based audit command is checked in:
  tools/checks/concurrency_boundary_surface_guard.sh
no active non-smoke use remains before parser/runtime quarantine
canonical parser/Program JSON tests cover the live behavior
compat smoke is either archived or explicitly named legacy-compat
```

### CONC-CO-001

Canonical source spelling:

```hako
co {
    local fut = nowait { work() }
    return await fut
}
```

Compatibility/internal spelling:

```text
task_scope
TaskGroupBox
push_task_scope / pop_task_scope
```

Rules:

```text
co is not detach
co is not thread
co is not select
co is not a true-parallel guarantee
co owns child Futures created inside the block
```

Acceptance:

```text
parser accepts co block
AST/Program JSON carries the same structured-scope meaning as task_scope
task_scope remains accepted as compatibility spelling
diagnostics prefer co for new source
no runtime owner rename in this row
runtime/MIR hook lowering stays fail-fast until CONC-CONTEXT-002 or a dedicated co-runtime row
```

### CONC-FUTURE-SEM-001

This row pins the existing MIRBuilder Future boundary before `co` lowering is
opened.

Current owner:

```text
src/mir/builder/stmts/async_stmt.rs
```

Contract:

```text
nowait -> evaluate expression -> FutureNew -> Future<T> value type
await -> evaluate future expression -> Safepoint -> Await -> Safepoint
nowait_os_thread_spawn=0
```

MIR JSON bridge:

```text
producer emits: future_new / future_set / await / safepoint
reader already accepts: future_new / future_set / await / safepoint
```

Stop line:

```text
no source-level thread syntax
no worker-pool activation
no reinterpretation of nowait as OS thread spawn
no co/task_scope ownership lowering in this row
```

LLVM harness parity is not owned by this row. Current
`async_min_harness.sh` reaches ny-llvmc and fails with
`unsupported pure shape for current backend recipe`; track that under a
dedicated LLVM/backend recipe row if needed.

Next code-opening row:

```text
CONC-CO-MIR-001
  lower co / compat task_scope as explicit TaskGroup ownership events
  design choice required before code:
    runtime hook calls vs metadata-only events vs dedicated MIR instructions
```

### CONC-CO-MIR-001

This row opens structured `co` / compatibility `task_scope` ownership lowering
after the Future boundary is pinned.

Existing runtime truth:

```text
src/runtime/global_hooks.rs
  push_task_scope()
  pop_task_scope()
  register_future_to_current_group(future)

src/runtime/context_snapshot.rs
  context snapshot value shape
```

The first executable slice should not create a second TaskGroup truth owner.
The default recommendation is runtime hook calls:

```text
co/task_scope enter -> push_task_scope()
body lowering
co/task_scope exit -> pop_task_scope()
```

Decision:

```text
co_taskgroup_lowering_shape=runtime_hook_calls
co_taskgroup_pop_error_policy=fail_fast
co_taskgroup_future_registration_owner=runtime_global_hooks
co_taskgroup_new_mir_opcode_count=0
co_early_exit_policy=normal_completion_only
program_json_co_lowering_enabled=0
llvm_co_lowering_enabled=0
```

Stop line:

```text
no OS thread spawn semantics
no worker-pool activation
no worker_scope / parallel parser or MIR lowering
no Channel / sync-box / context widening
no silent ignore of pop_task_scope() errors
no return/throw/break/continue escaping co/task_scope in v0
```

Implementation split:

```text
CONC-CO-MIR-001A:
  decision pin, docs only

CONC-CO-MIR-001B:
  VM extern hooks:
    env.task_scope.push
    env.task_scope.pop
  status=implemented

CONC-CO-MIR-001C:
  MIRBuilder lexical lowering:
    push -> body -> pop
  early exits fail-fast
  status=implemented

CONC-CO-MIR-001D:
  fixtures / guards:
    co + nowait + await positive
    co { return ... } negative
  status=implemented
```

### CONC-CHANNEL-001

### CONC-CHANNEL-002

Reference runtime scaffold:

```text
source surface:
  await ch.close()

runtime owner:
  src/runtime/channel_queue.rs

legacy separation:
  src/core/channel_box.rs ChannelBox is not reused
```

Contract pinned in this row:

```text
close marks the queue closed
close wakes blocking reference receivers
send after close is rejected and returns the value to the caller
recv drains buffered values after close
recv returns closed only after the buffer is empty
double close is rejected
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

Stop line:

```text
no source-level blocking call
no hidden ordinary send/recv wait
no worker-pool activation
no Program JSON / MIR / LLVM route widening
```

### CONC-CHANNEL-003

Await-visible route descriptors:

```text
await ch.send(value)
await ch.recv()
await ch.close()
ch.try_send(value)
ch.try_recv()
```

Runtime owner:

```text
src/runtime/channel_route.rs
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

Stop line:

```text
route descriptors only
no Program JSON / MIR / LLVM lowering
no ordinary blocking send / recv / close call
no legacy P2P ChannelBox reuse
```

Docs/API decision already fixed by `boundary-model.md`:

```text
await ch.send(v)
await ch.recv()
await ch.close()
ch.try_send(v)
ch.try_recv()
```

This row should make current channel docs and examples consistent. It may add
fixtures that are expected to fail-fast until runtime rows are implemented, but
must not silently accept hidden blocking calls.

Acceptance:

```text
reference docs use recv, not receive, for the canonical API
close is written as await ch.close()
try_send / try_recv are explicitly non-blocking
the existing Rust P2P ChannelBox is not treated as the new Channel<T> queue
```

### CONC-SYNCBOX-001 / 002 / 003

Split parser, verifier, and runtime behavior:

```text
001: parse and carry sync box metadata
002: reject await/nowait/blocking waits inside sync methods
003: reference serialized method-entry behavior
```

`lock<T>` is not a competing canonical surface. If raw lock syntax exists in
legacy tests or design notes, `CONC-COMPAT-001` decides whether it is active
compatibility or archive-only coverage. The canonical direction stays
`sync box`.

This keeps syntax acceptance separate from semantic enforcement and backend
lowering.

### CONC-CONTEXT-001 / 002

Split naming from propagation:

```text
001: context syntax / docs / scoped compat quarantine
002: creation-time snapshot inheritance for explicit co/task_scope children
```

The implicit root scope must not become detached context propagation.

CONC-CONTEXT-002 reference owner:

```text
src/runtime/context_snapshot.rs
src/runtime/global_hooks.rs
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

Runtime contract:

```text
push_context_binding(name, value)
register_future_to_current_group(future) inside explicit co/task_scope
  -> captures current context stack snapshot
register_future_to_current_group(future) outside explicit scope
  -> no context propagation
```

### CONC-SOURCE-PARALLEL-001

This row reserves the future structured parallel source surface without opening
parser or lowering support.

Current canonical source surface:

```hako
co {
    local a = nowait { workA() }
    local b = nowait { workB() }

    local x = await a
    local y = await b
    return x + y
}
```

Reserved future structured parallel surface:

```hako
worker_scope workers = N {
    parallel i in range {
        work(i)
    }
}
```

Closed surface:

```hako
thread {
    work()
}
```

Decisions:

```text
co_nowait_await_canonical_source_surface=1
worker_scope_design_reserved=1
worker_scope_parser_enabled=0
worker_scope_ast_json_enabled=0
worker_scope_program_json_enabled=0
worker_scope_mir_lowering_enabled=0
worker_scope_llvm_lowering_enabled=0
worker_scope_runtime_route_enabled=0
raw_thread_parser_enabled=0
```

`workers = N` is a scheduler budget hint and upper bound, not an exact OS
thread-count promise:

```text
worker_scope_workers_is_upper_bound=1
worker_scope_exact_thread_count_promise=0
worker_scope_os_thread_spawn_direct=0
```

Opening parser/lowering for `worker_scope` requires `THREAD-SAFETY-001` to
enforce the safety boundary:

```text
thread_safety_gate_required=1
hako_send_share_enforced=1
thread_registry_gc_roots_enabled=1
worker_scope_capture_check_enabled=1
worker_scope_value_movement_enabled=1
```

Until those fields are true, `worker_scope` is documentation-only. Do not add a
parser capsule, AST JSON shape, MIR metadata carry, LLVM lowering, or runtime
worker-pool route for it.

No silent fallback rule:

```text
worker_scope_silent_fallback_count=0
```

Once `worker_scope` becomes source-visible, any route that executes fewer
workers than requested, or uses a cooperative/inline route instead of a worker
pool, must report the effective route and reason. It must not silently claim
`worker_pool_task`.

Fail-fast tags reserved for later implementation:

```text
[concurrency/worker-scope-disabled]
[concurrency/parallel-outside-worker-scope]
[concurrency/raw-thread-disabled]
[concurrency/send-not-proven]
[concurrency/share-not-proven]
[concurrency/thread-root-missing]
```

## Mimalloc Stop Line

None of these rows is a prerequisite for the current mimalloc allocator
substrate lane. Mimalloc may continue using runtime/internal:

```text
hako_worker_current_id_i64
hako_tls_cache_slot_get_i64 / hako_tls_cache_slot_set_i64
hako_atomic_*
thread-safe hako_mem ABI
```

Those substrate rows do not open source-level `Channel`, `sync box`,
`context`, `worker_local`, `lock<T>`, or true-parallel semantics.
