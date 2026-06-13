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
explicit worker surface be considered.

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
| `CONC-COMPAT-001` | landed-audit | Audit legacy concurrency spellings and smoke-only compatibility users. | `tools/checks/concurrency_boundary_surface_guard.sh` | no parser/runtime deletion |
| `CONC-CO-001` | landed-parser-json | Add `co` as canonical structured concurrency source spelling while keeping `task_scope` as compat/internal wording. | parser + AST JSON + Program JSON row | runtime hook lowering remains fail-fast |
| `CONC-CHANNEL-001` | landed-api-docs | Pin Channel API shapes around await-visible `send` / `recv` / `close`. | docs/reference + guard | no wait runtime rewrite |
| `CONC-CHANNEL-002` | pending | Implement `await ch.close()` semantics in the future `Channel<T>` queue runtime scaffold. | VM/reference guard for close wake/drain/send-after-close | no true parallel scheduler |
| `CONC-CHANNEL-003` | pending | Implement await-visible `send` / `recv` route shape or fail-fast bridge. | parser/MIR/runtime route guard | no hidden blocking ordinary call |
| `CONC-SYNCBOX-001` | landed-parser-json | Add `sync box` parser/AST capsule and canonical docs. | parse/AST JSON roundtrip guard + lowering fail-fast | no serialized runtime yet |
| `CONC-SYNCBOX-002` | landed-verifier | Add verifier rule: no `await` / `nowait` / channel wait inside `sync box` method. | parser-side fail-fast diagnostics guard | no lock-order inference |
| `CONC-SYNCBOX-003` | landed-code | Add reference-only serialized method-entry behavior. | `src/runtime/sync_box.rs` + `293x-1003-CONC-SYNCBOX-003-REFERENCE-SERIALIZED-ENTRY.md` | Program JSON / MIR / LLVM fail-fast continue |
| `CONC-CONTEXT-001` | landed-parser-json | Add `context` surface as canonical name and quarantine `scoped` as compat. | parser/AST JSON guard + scoped compat audit | no propagation runtime yet |
| `CONC-CONTEXT-002` | pending | Implement context snapshot on `nowait` child creation inside explicit `co` / compatibility `task_scope`. | VM/reference guard | implicit root is not detached propagation |
| `CONC-WORKERLOCAL-001` | pending | Keep `worker_local` source syntax closed while allocator substrate remains internal. | no-source-worker-local guard | no mimalloc behavior change |

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

### CONC-CHANNEL-001

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
