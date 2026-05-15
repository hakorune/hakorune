# Mimalloc Concurrency Substrate Boundary SSOT

Status: SSOT
Date: 2026-05-15
Scope: mimalloc-facing concurrency substrate vs user-facing concurrency language surface.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/lock_scoped_worker_local.md`
- `docs/reference/runtime/substrate-capabilities.md`
- `docs/reference/mir/metadata-facts-ssot.md`
- `docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md`

## Decision

Mimalloc migration needs concurrency substrate, not the full concurrency
language surface.

Required for mimalloc:

```text
worker/thread identity
runtime-internal worker-local / TLS cache slots
atomic load/store/CAS/fetch_add routes
OS virtual memory reserve/commit/decommit
thread-safe hako_mem ABI
remote-free / abandoned-owner / page-ownership policy
```

Not required for mimalloc migration:

```text
nowait/await language expansion
Channel
task_scope
scoped request context propagation
source-level worker_local syntax
true parallel language semantics
```

`worker_local` is the important split: mimalloc needs an allocator-internal
worker-local/TLS substrate, not a source-level `worker_local` feature.

## Feature Reading

| Feature | Mimalloc requirement | Owner reading |
| --- | --- | --- |
| `nowait` / `await` | not a direct allocator prerequisite | keep existing Phase-0 Future parity; do not tie allocator thread cache to async semantics |
| `Channel` | not required | allocator remote-free queues are allocator-owned structures, not `ChannelBox` |
| `task_scope` | not required | Future ownership and allocator heap ownership stay separate |
| `lock<T>` | source surface not required; internal mutex may be required | runtime/internal primitive can exist without opening `lock<T>` syntax |
| `scoped` | not required | request/trace context must not become allocator correctness state |
| `worker_local` | internal substrate required; source surface deferred | use runtime/internal TLS or worker slots for heap/cache/stats; no user syntax yet |
| true parallel | language semantics deferred; native substrate smoke required later | VM remains proof/reference; LLVM/EXE owns real substrate behavior |

## Layer Ownership

Stage0 / runtime kernel owns substrate providers:

```text
worker/thread id
TLS / worker-local slots
atomic load/store/CAS/fetch_add
mutex / low-level lock primitive if needed internally
OSVM reserve/commit/decommit
hako_mem_alloc/free/realloc ABI
TLS diagnostics such as hako_last_error
```

Stage1 owns language semantics only when they are exposed:

```text
lock<T> semantics and no-await-in-lock verifier
source-level worker_local semantics if opened later
scoped inheritance rules
task_scope ownership / cancellation / failure contracts
capability checking and verifier facts
hako_alloc lifecycle invariants
```

MIR metadata / CorePlan owns compiler route contracts:

```text
extern_call_routes
lowering_plan
capability gates
atomic memory-order route rows
TLS worker-cache slot route rows
no-fallback / fail-fast decisions
```

Backend / LLVM owns native lowering:

```text
atomic intrinsic or runtime call emit
TLS access emit
mutex primitive call emit
OSVM call emit
thread-safe ABI call
```

VM owns proof/reference behavior only:

```text
current_worker_id = 0
TLS slot map for worker 0
atomic slots as single-thread deterministic cells
lock primitive as no-contention guard
no true thread pool
```

## Implementation Wave

| Row | Status | Purpose |
| --- | --- | --- |
| `MIMAP-SUBSTRATE-CONC-001` | landed | Pin this boundary and task order. |
| `MIMAP-SUBSTRATE-CONC-002` | ready | Inventory/guard existing route facts for `hako.atomic`, `hako.tls`, `hako.osvm`, and `hako.mem`; no behavior change. |
| `MIMAP-021C` | ready after 002 | Return to facade page-source allocation-miss fallback. |
| `MIMAP-WORKER-001` | planned after 021C | Internal worker identity substrate; VM returns 0. |
| `MIMAP-TLS-001` | planned | Allocator-internal worker-local cache slot usage; no source syntax. |
| `MIMAP-ATOMIC-001` | planned | Consolidate allocator ownership atomic route set. |
| `MIMAP-REMOTE-001` | planned | Production-facade remote-free policy integration over existing atomic/TLS proofs. |
| `MIMAP-THREADSAFE-ABI-001` | planned | Thread-safe `hako_mem` ABI contract. |
| `MIMAP-PAR-STRESS-001` | parked | Native multi-worker stress after substrate rows are live. |

## Stop Lines

- Do not open source-level `worker_local` for mimalloc.
- Do not implement `lock<T>` syntax for allocator internals.
- Do not use `ChannelBox` for allocator remote-free queues.
- Do not require true parallel VM behavior.
- Do not add raw helper-name backend classifiers; route through
  `extern_call_routes` / `lowering_plan`.
- Do not activate provider hooks, host allocator replacement, or
  `#[global_allocator]`.
