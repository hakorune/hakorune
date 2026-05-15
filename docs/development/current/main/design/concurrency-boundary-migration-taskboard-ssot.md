# Concurrency Boundary Migration Taskboard

Status: SSOT
Scope: implementation rows for the concurrency Boundary model.

Related:
- `docs/reference/concurrency/boundary-model.md`
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md`

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
| `CONC-COMPAT-001` | ready | Audit legacy concurrency spellings and smoke-only compatibility users. | inventory + no-active-use guard plan | no parser/runtime deletion |
| `CONC-CO-001` | ready | Add `co` as canonical structured concurrency source spelling while keeping `task_scope` as compat/internal wording. | parser/docs guard + diagnostic plan | no runtime owner rename |
| `CONC-CHANNEL-001` | ready | Pin Channel API shapes around await-visible `send` / `recv` / `close`. | reference/API docs + fixture plan | no wait runtime rewrite |
| `CONC-CHANNEL-002` | pending | Implement `await ch.close()` semantics in the current ChannelBox/runtime scaffold. | VM/reference guard for close wake/drain/send-after-close | no true parallel scheduler |
| `CONC-CHANNEL-003` | pending | Implement await-visible `send` / `recv` route shape or fail-fast bridge. | parser/MIR/runtime route guard | no hidden blocking ordinary call |
| `CONC-SYNCBOX-001` | ready | Add `sync box` parser/AST capsule and canonical docs. | parse/AST JSON roundtrip guard | no serialized runtime yet |
| `CONC-SYNCBOX-002` | pending | Add verifier rule: no `await` / `nowait` / channel wait inside `sync box` method. | fail-fast diagnostics guard | no lock-order inference |
| `CONC-SYNCBOX-003` | pending | Add VM/reference serialized method-entry behavior. | no-contention reference guard | no fairness/reentrancy guarantee |
| `CONC-CONTEXT-001` | ready | Add `context` surface as canonical name and quarantine `scoped` as compat. | parser/docs guard + scoped compat audit | no propagation runtime yet |
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
ChannelBox blocking send/receive without await
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
rg-based audit command is checked in
no active non-smoke use remains before parser/runtime quarantine
canonical smoke covers the live behavior
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

### CONC-SYNCBOX-001 / 002 / 003

Split parser, verifier, and runtime behavior:

```text
001: parse and carry sync box metadata
002: reject await/nowait/blocking waits inside sync methods
003: reference serialized method-entry behavior
```

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
