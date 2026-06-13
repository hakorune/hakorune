# CONC-SOURCE-PARALLEL-001: Source Parallel Surface Freeze

Status: Landed docs
Date: 2026-06-13
Scope: concurrency source-surface design reservation only.

## Decision

Hakorune keeps the current user-facing concurrency source surface centered on
`co { nowait ... await ... }`.

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

This row is documentation and report vocabulary only. It does not add parser,
AST JSON, Program JSON, MIR, LLVM, or runtime route support for `worker_scope`,
`parallel`, or raw `thread`.

## Invariants

```text
co_nowait_await_canonical_source_surface=1
nowait_os_thread_spawn=0
hako_source_owns_raw_os_thread=0
source_level_thread_syntax=0
worker_scope_design_reserved=1
worker_scope_parser_enabled=0
worker_scope_ast_json_enabled=0
worker_scope_program_json_enabled=0
worker_scope_mir_lowering_enabled=0
worker_scope_llvm_lowering_enabled=0
worker_scope_runtime_route_enabled=0
raw_thread_parser_enabled=0
```

`workers = N` is not an exact OS-thread-count promise:

```text
worker_scope_workers_is_upper_bound=1
worker_scope_exact_thread_count_promise=0
worker_scope_os_thread_spawn_direct=0
```

## Gate

Opening parser/lowering for `worker_scope` requires the safety gate first:

```text
THREAD-SAFETY-001 required
hako_send_share_enforced=1
thread_registry_gc_roots_enabled=1
worker_scope_capture_check_enabled=1
worker_scope_value_movement_enabled=1
```

Until that gate is green, `worker_scope` remains reserved design only.

## No Silent Fallback

Once `worker_scope` becomes source-visible, route fallback must be explicit:

```text
worker_scope_requested_workers=<n>
worker_scope_effective_workers=<m>
worker_scope_workers_is_upper_bound=1
worker_scope_exact_thread_count_promise=0
worker_scope_route=inline_resolved_future|cooperative_task|worker_pool_task
worker_scope_silent_fallback_count=0
```

The runtime may use fewer/equivalent workers or an inline/cooperative route only
with report evidence. It must not silently claim `worker_pool_task`.

## Reserved Diagnostics

```text
[concurrency/worker-scope-disabled]
[concurrency/parallel-outside-worker-scope]
[concurrency/raw-thread-disabled]
[concurrency/send-not-proven]
[concurrency/share-not-proven]
[concurrency/thread-root-missing]
```

## Non-Goals

- Do not implement parser support.
- Do not add AST JSON or Program JSON shapes.
- Do not add MIR metadata or lowering.
- Do not add LLVM lowering.
- Do not activate `WorkerPoolScheduler` from source syntax.
- Do not reinterpret `nowait` as OS thread spawn.
- Do not add raw `thread { ... }`.

## Evidence

Updated docs:

```text
docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md
docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md
docs/reference/concurrency/boundary-model.md
docs/reference/concurrency/semantics.md
CURRENT_TASK.md
```
