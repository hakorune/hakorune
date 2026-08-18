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
nowait expr
await expr
Channel<T>
sync box
context
```

`scoped`, source `task_scope`, and the historical `nowait name = expr` binding
statement are compatibility inputs with explicit sunset rows. Internal
`TaskScope` / `TaskGroupBox` / runtime hook names are not source aliases and
remain. `lock<T>`, source `worker_local`, and raw `thread {}` are rejected.
Structured parallelism remains a future concept, but `worker_scope` and
`parallel` are no longer reserved as its exact source spellings.

`lock<T>` must not be promoted to the canonical source surface. The canonical
shared-mutable surface is `sync box`; raw locks remain implementation concepts,
historical/provisional compatibility, or runtime/internal primitives.

## 2026-08-19 Surface Census Receipt

The current repository census keeps the public surface decision small and
separates it from the runtime thread substrate:

```text
canonical meaning surfaces:
  Future<T> / nowait / await / co
  Channel<T>
  sync box
  context

future design only:
  structured parallelism (exact spelling undecided)

runtime-only:
  Scheduler / ThreadApi / ThreadRegistry / worker-local-TLS / mutexes
```

The active `.hako` corpus has no `task_scope`, `scoped`, `sync box`,
`worker_scope`, `parallel`, raw `thread {}`, `lock<T>`, or `worker_local`
source use. It still contains seven historical `nowait name = expr` lines and
no `local name = nowait expr` lines, so `CONC-NOWAIT-EXPR-D0` remains a
design decision rather than an already-landed syntax claim. `ChannelBox` has
no in-repo construction caller, but its crate-root export and builtin type
name are externally observable; its disposition therefore needs a public-API
decision before deletion.

The first cleanup row is `CONC-GUARD-AST-CRATE0`: refresh the three stale
guards to the current frontend-AST, split decoder, Program JSON, and MIR owner
paths with behavior/grammar delta zero. Only after that row should the
grammar/compatibility sequence proceed. `worker_scope` is not an alternate
spelling of `co`: it would describe a future worker-budget/parallel boundary,
whereas `co` owns structured child Futures and cancellation. Its exact source
spelling is intentionally unreserved until Send/Share/ThreadRoot capture
safety is closed.

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

Only after these rows may a later Decision choose a structured-parallel source
surface. Parser, AST JSON, Program JSON, MIR, LLVM, and runtime route activation
stay closed until `THREAD-SAFETY-001` enforces send/share/root safety; the old
`worker_scope` / `parallel` sketch does not reserve those spellings.

## Recommended Task Order

### 2026-08-19 public-surface reduction order

This is parked work and never changes `CURRENT_STATE.toml` by itself. Each row
is one BoxShape or BoxCount; grammar acceptance, runtime activation, and
compatibility retirement must not share a commit.

| Order | Row | Purpose | Stop line |
| --- | --- | --- | --- |
| 1 | `CONC-GUARD-AST-CRATE0` | Refresh the three red guards to the current frontend-AST, split decoder, Program JSON, and MIR owner paths. | behavior/grammar delta 0 |
| 2 | `CONC-NOWAIT-EXPR-D0` | Seal precedence, AST owner, Future result type, and compatibility normalization for `nowait expr`. | design only; current statement stays live |
| 3 | `CONC-GRAM-SYNC0` | Register the already-live `sync box` capsule in registry, EBNF, corpus, and both parser witnesses. | no runtime widening |
| 4 | `CONC-GRAM-CO0` | Register `co` canonical and source `task_scope` Canonical-rejected/Compat2025-only. | internal TaskScope names unchanged |
| 5 | `CONC-GRAM-CONTEXT0` | Register `context` canonical and `scoped` Canonical-rejected/Compat2025-only. | no context propagation widening |
| 6 | `CONC-NOWAIT-EXPR-I0` | Add expression-shaped Future creation while retaining the old statement only as compatibility input. | no scheduler/OS-thread change |
| 7 | `CONC-NOWAIT-EXPR-MIGRATE-R0` | Rewrite the eight active old-statement occurrences and move binding ownership back to ordinary `local`. | no alias parser deletion yet |
| 8 | `CONC-SCOPED-COMPAT-R0` | Quarantine `scoped` before `task_scope`; keep spelling fields until the whole Compat2025 row retires. | no source_keyword deletion |
| 9 | `CONC-TASK-SCOPE-COMPAT-R0` | Quarantine source `task_scope`; retain TaskScope/TaskGroup/runtime vocabulary. | no runtime owner rename |
| 10 | `CONC-RUNTIME-DOCS-OWNER-R0` | Stub the old lock/scoped/worker-local page and route public meaning, current status, and threading substrate to their three existing owners. | docs only |
| 11 | `CONC-CHANNELBOX-DISPOSITION-D0` | Decide the public Rust/type-name/builtin compatibility of repo-caller-zero legacy P2P ChannelBox/MessageBox. | census only; no rename/delete |
| 12 | `CONC-CHANNELBOX-R0` | Prefer complete retirement; delete owner/export/builtin/UI wording atomically only after D0 permits it. | canonical Channel remains caller-zero |
| 13 | `CONC-NONCANONICAL-VOCAB-R0` | Record lock/worker-local/thread as rejected/runtime-only and structured-parallel spelling as undecided. | no parser or scheduler route |
| 14 | `CONC-CO-EXIT-TRANSACTION-D0/I0` | Replace the `co`-specific early-exit AST scan with the common scope-exit transaction. | no second cleanup/exit ledger |
| 15 | `CONC-SYNCBOX-EFFECT0` | Replace the wait-like AST scan with verified callable effects. | no lock-order or runtime widening |

`source_keyword` and old `ASTNode::Nowait { variable, ... }` deletion are
conditional final rows: they open only after their Compat2025 inputs have no
remaining contract. Home ownership grammar remains owned by the Home taskboard.

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
| `CONC-GUARD-AST-CRATE0` | pending-clean-first | Refresh three stale concurrency guards after AST, decoder, Program JSON, and MIR owner splits. | current-owner paths + five green concurrency guards | behavior/grammar delta 0; wait guard remains unchanged |
| `CONC-GRAM-SYNC0` | pending | Register the already parser-live `sync box` capsule in Language-v1 grammar SSOT and EBNF. | registry + EBNF + Rust/Hako witness | no Program JSON/MIR/runtime activation |
| `CONC-GRAM-CO0` | pending | Register canonical `co` and source `task_scope` as separate Canonical-rejected/Compat2025-only spelling evidence. | registry + EBNF + Rust/Hako witness | no scheduler/runtime widening |
| `CONC-GRAM-CONTEXT0` | pending | Register canonical `context` and `scoped` as separate Canonical-rejected/Compat2025-only spelling evidence. | registry + EBNF + Rust/Hako witness | no propagation widening |
| `CONC-NOWAIT-EXPR-D0/I0/MIGRATE-R0` | pending-design | Replace the dedicated binding statement with `nowait expr -> Future<T>` and ordinary local binding through a compatibility migration. | Decision + AST/parser/lowering + eight active-source rewrites | no scheduler/OS-thread change |
| `CONC-SCOPED-COMPAT-R0` / `CONC-TASK-SCOPE-COMPAT-R0` | pending | Quarantine aliases by grammar profile; delete spelling carriers only after Compat2025 sunset. | stable rejects + compat fixtures + no-active-use guard | no internal TaskScope rename |
| `CONC-RUNTIME-DOCS-OWNER-R0` | pending-docs | Retire the duplicate lock/scoped/worker-local page to a short historical stub and correct ThreadRegistry scope. | owner links + stub + runtime wording | behavior/grammar delta 0 |
| `CONC-CHANNELBOX-DISPOSITION-D0/R0` | pending-design | Decide and then retire the repo-caller-zero legacy P2P ChannelBox/MessageBox public identity. | public-compat Decision + atomic delete/absence guard | no canonical Channel activation |
| `CONC-NONCANONICAL-VOCAB-R0` | pending-docs | Keep lock/worker-local/thread out of source and leave structured-parallel spelling undecided. | reference/status/guard wording | no parser/runtime route |
| `CONC-SYNCBOX-VIEW-D0` | pending-design / historical row name | Forbid receiver/field-anchored Home result handles from escaping synchronized entry in the first profile. | Home result-relation Decision + fail-fast fixtures + reference-only example label | no synchronized handle token or hidden snapshot |
| `CONC-SYNCBOX-EFFECT0` | pending-design | Seal callable effects for blocking calls, nested sync calls, and lock-order-sensitive operations. | verified effect ABI + conservative rejection fixtures | parser node scan is not the final authority |
| `CONC-CO-EXIT-TRANSACTION-D0/I0` | pending-design | Project normal/return/throw/break/continue cleanup through the common scope-exit transaction. | exact exit census + TaskGroup cleanup projection | no co-specific AST scan or second exit ledger |
| `CONC-SYNCBOX-TRANSFER-D0` | pending-design | Decide Home transfer/Share/Send/Sync capability for `co`, `nowait`, and Channel transfer. | explicit transfer contract | no runtime-behavior inference |
| `CONC-CONTEXT-001` | landed-parser-json | Add `context` surface as canonical name and quarantine `scoped` as compat. | parser/AST JSON guard + scoped compat audit | no propagation runtime yet |
| `CONC-CONTEXT-002` | landed-code | Implement context snapshot on `nowait` child creation inside explicit `co` / compatibility `task_scope`. | `src/runtime/context_snapshot.rs` + `293x-1006-CONC-CONTEXT-002-CONTEXT-SNAPSHOT-REFERENCE.md` | implicit root is not detached propagation |
| `CONC-WORKERLOCAL-001` | pending | Keep `worker_local` source syntax closed while allocator substrate remains internal. | no-source-worker-local guard | no mimalloc behavior change |
| `CONC-SOURCE-PARALLEL-001` | superseded-spelling / landed-safety | Preserve the Send/Share/ThreadRoot stop line while releasing the old `worker_scope` / `parallel` spelling reservation. | historical card + current Boundary model | exact future spelling undecided; raw thread syntax closed |

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
task_scope remains current compatibility input until CONC-GRAM-CO0, then is Compat2025-only
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

### CONC-GUARD-AST-CRATE0 / CONC-GRAM-*

`CONC-GUARD-AST-CRATE0` is a behavior-neutral prerequisite. Its historical
name is narrower than the audited drift: update only these stale guards to
read the current frontend AST, split decoder, Program JSON, and MIR owners:

```text
concurrency_sync_box_surface_guard.sh
concurrency_boundary_surface_guard.sh
concurrency_context_surface_guard.sh
```

Required owner refresh includes
`crates/hakorune_frontend_ast/src/{ast_node.rs,utils/node_type.rs}`,
`roundtrip_decoder{,/declarations}.rs`, `lowering/statements.rs`, and the
current raw-expression dispatch modules. Do not recreate deleted facade paths.
Baseline audit (2026-08-19): channel API and sync-wait guards are green;
boundary, context, and sync-surface guards are red only on these stale owner
paths/expectations and are classified as known baseline debt for this row.
`concurrency_sync_box_wait_guard.sh` is already green and must not be rewritten
as part of this row. After the guard seam is current, add grammar SSOT one
durable task at a time: `sync box`, then the separate `co` and `task_scope`
spelling rows, then the separate `context` and `scoped` spelling rows. Every
grammar task closes registry, EBNF, corpus, Rust parser witness, Hako parser
witness, and negative fixtures together without opening Program JSON, MIR, or
runtime execution.

### CONC-SYNCBOX-VIEW-D0 / EFFECT0 / TRANSFER-D0

Initial Home result-handle safety boundary (`Decision: provisional` until D0
closes; row name is historical):

```text
sync box method -> receiver/field-anchored result handle: reject
sync box field -> escaping handle: reject
snapshot/independently-owned result: requires an exact declared Home ABI
synchronized-handle token: later independent Decision
```

A method-entry lock cannot silently anchor a result handle after the guard is released.
`EFFECT0` separately replaces the current explicit `await`/`nowait` node scan
with an exact callable-effect contract for blocking and nested sync operations.
`TRANSFER-D0` separately decides Home transfer/share/Send/Sync across
task/channel edges; current runtime behavior is never permission authority.
Fairness is not an open question in these rows:
the Phase-0 contract deliberately provides no fairness or starvation guarantee.

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

This historical row landed the safety stop line without opening parser or
lowering support. Its concrete `worker_scope` / `parallel` spelling reservation
is superseded: structured parallelism remains future work, exact spelling is
undecided, and neither word is reserved by Language v1.

Accepted target source surface (not the current parser syntax):

```hako
co {
    local a = nowait { workA() }
    local b = nowait { workB() }

    local x = await a
    local y = await b
    return x + y
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
structured_parallel_design_area=1
structured_parallel_exact_spelling_decided=0
worker_scope_keyword_reserved=0
parallel_keyword_reserved=0
worker_scope_parser_enabled=0
worker_scope_ast_json_enabled=0
worker_scope_program_json_enabled=0
worker_scope_mir_lowering_enabled=0
worker_scope_llvm_lowering_enabled=0
worker_scope_runtime_route_enabled=0
raw_thread_parser_enabled=0
```

Any future worker budget is an upper bound, not an exact OS-thread-count
promise. Existing historical report-field names remain runtime vocabulary:

```text
worker_scope_workers_is_upper_bound=1
worker_scope_exact_thread_count_promise=0
worker_scope_os_thread_spawn_direct=0
```

Opening parser/lowering for any structured-parallel spelling requires
`THREAD-SAFETY-001` to enforce the safety boundary:

```text
thread_safety_gate_required=1
hako_send_share_enforced=1
thread_registry_gc_roots_enabled=1
worker_scope_capture_check_enabled=1
worker_scope_value_movement_enabled=1
```

Until those fields are true, do not add a parser capsule, AST JSON shape, MIR
metadata carry, LLVM lowering, or runtime worker-pool route for any spelling.

No silent fallback rule:

```text
structured_parallel_silent_fallback_count=0
```

Once a structured-parallel surface becomes source-visible, any route that
executes fewer workers than requested, or uses a cooperative/inline route
instead of a worker pool, must report the effective route and reason. It must
not silently claim `worker_pool_task`. Old worker-scope-specific diagnostic
tags are historical evidence, not reserved future API.

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
