# MIMAP-011 allocator facade lifecycle route SSOT

Decision: accepted.

`MIMAP-011` routes allocator facade calls to the lifecycle-aware page selection
policy introduced by `MIMAP-010`.

## Backend acceptance

Acceptance backend: LLVM/EXE primary.

VM remains a small semantic-reference smoke backend, but VM object-heavy page queue
or facade retention is not required for this row.

## Owner boundary

Facade owner:

```text
lang/src/hako_alloc/memory/allocator_facade_box.hako
```

Selection owner:

```text
lang/src/hako_alloc/memory/page_queue_lifecycle_box.hako
```

The facade exposes scalar lifecycle selection methods:

```text
lifecycleSelectionBegin()
lifecycleSelectionConsider(page_id, decommitted, retired, reusable, available)
lifecycleSelectionFinish()
lifecycleSelectedKind()
```

This row intentionally does not pass page objects through the facade/queue route.
Object-heavy page retention remains covered by `VM-LIM-001` and must be proven by
LLVM/EXE before it becomes an allocator route requirement.

## Proof and guard

```text
apps/mimalloc-facade-lifecycle-route-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_lifecycle_route_exe_guard.sh
```

## Non-goals

- no provider activation
- no host allocator replacement
- no hooks
- no OSVM execution
- no page object retention inside queue-like boxes

Next row: `MIMAP-012 object-backed lifecycle queue LLVM route pilot`.
