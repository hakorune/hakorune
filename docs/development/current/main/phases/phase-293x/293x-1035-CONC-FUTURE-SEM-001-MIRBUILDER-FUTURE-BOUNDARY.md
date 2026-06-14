# CONC-FUTURE-SEM-001 MIRBuilder Future Boundary

Status: Landed-code
Date: 2026-06-15
Scope: `nowait` / `await` / `Future<T>` MIRBuilder boundary before opening
structured `co` ownership lowering.

Related:
- `docs/reference/concurrency/semantics.md`
- `docs/reference/concurrency/boundary-model.md`
- `docs/development/current/main/design/concurrency-boundary-migration-taskboard-ssot.md`
- `docs/development/current/main/design/concurrency-async-pre-selfhost-ssot.md`
- `docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md`
- `src/mir/builder/stmts/async_stmt.rs`

## Decision

`nowait` / `await` are owned by MIRBuilder and are the current canonical
Future boundary:

```text
nowait:
  evaluate expression in the current route
  wrap the produced value with MIR FutureNew
  register Future<T> in value_types

await:
  evaluate the future expression
  emit Safepoint
  emit MIR Await
  register the awaited result type from Future<T> when known
  emit Safepoint
```

This row does not add source syntax, scheduler behavior, or worker-pool
execution. It names the existing MIRBuilder contract so the next rows can add
structured ownership events without reinterpreting `nowait`.

## Non-Negotiable Boundaries

```text
nowait_os_thread_spawn=0
source_level_thread_syntax=0
worker_pool_source_route_enabled=0
worker_scope_parser_enabled=0
worker_scope_mir_lowering_enabled=0
raw_thread_parser_enabled=0
```

`nowait` means Future/task ownership. It is not an OS thread spawn promise.
Runtime routes may later choose inline, cooperative, or worker-pool execution
for eligible tasks, but source meaning remains Future ownership.

## Current Code Evidence

```text
src/mir/builder/stmts/async_stmt.rs
  build_nowait_statement:
    expression -> FutureNew
    Future<T> type registration
    variable_map binding

  build_await_expression:
    expression -> Safepoint -> Await -> Safepoint
    Future<T> result type propagation
```

The MIR JSON v0 reader already accepts `future_new`, `future_set`, `await`, and
`safepoint`. This row also aligns the MIR JSON producer with that reader so the
VM route can round-trip the existing Future boundary.

The next row must not duplicate this contract in `co` lowering. `co` owns child
Future scope; it does not redefine Future construction or await semantics.

## Acceptance

Required gates:

```text
cargo test -q --lib backend::mir_interpreter::handlers::async_contract_tests
tools/smokes/v2/profiles/integration/async/async_min_vm.sh
tools/smokes/v2/profiles/integration/apps/phase29z_vm_hako_s5_await_non_future_reject_vm.sh
```

LLVM harness status:

```text
tools/smokes/v2/profiles/integration/llvm/async_min_harness.sh
  invocation fixed to use bash for tools/run_llvm_harness.sh
  current result: fails in ny-llvmc with "unsupported pure shape for current backend recipe"
  owner: separate LLVM/backend recipe row, not CONC-FUTURE-SEM-001
```

Documentation checks:

```text
concurrency semantics lists CONC-FUTURE-SEM-001 as landed-docs
taskboard places CONC-FUTURE-SEM-001 before CONC-CO-MIR-001
CONC-CO-MIR-001 remains the first code-opening row for structured ownership
```

## Next Row

```text
CONC-CO-MIR-001:
  lower co / compat task_scope as explicit TaskGroup ownership events
  no thread spawn
  no worker-pool activation
  no Channel/sync-box/context widening
```

Stop before implementation if the row needs to decide the exact MIR shape for
task-scope enter/exit instructions or whether to express the boundary as
runtime hook calls, metadata-only events, or dedicated MIR instructions. That
is the next design consultation point.

Optional later row:

```text
CONC-FUTURE-LLVM-001:
  make the LLVM harness accept the current Future boundary or document the
  required rewrite route as an executable backend contract
```
