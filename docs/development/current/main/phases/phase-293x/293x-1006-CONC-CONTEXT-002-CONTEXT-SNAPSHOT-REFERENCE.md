# CONC-CONTEXT-002 Context Snapshot Reference

Status: landed-code
Scope: runtime reference snapshots for structured ambient `context`.

## Decision

`CONC-CONTEXT-002` implements creation-time context snapshots for futures
registered inside an explicit `co` / compatibility `task_scope`.

It does not open Program JSON, MIR, or LLVM lowering for `ContextScope`.

## Landed Code

```text
src/runtime/context_snapshot.rs
src/runtime/global_hooks.rs
```

Runtime API:

```text
push_context_binding(name, value)
pop_context_binding(name)
current_context_snapshot()
context_snapshot_for_future(future)
```

## Contract

```text
explicit co/task_scope child:
  register_future_to_current_group(fut)
  -> captures current context stack snapshot

implicit root child:
  register_future_to_current_group(fut)
  -> no context propagation
```

Snapshot semantics:

```text
snapshot is taken at child registration time
later parent context changes do not mutate the child snapshot
same-name nested context resolves to the latest binding in the snapshot
```

## Report Fields

```text
context_snapshot_runtime_enabled=1
context_snapshot_explicit_scope_only=1
context_snapshot_implicit_root_propagation=0
context_snapshot_program_json_enabled=0
context_snapshot_mir_lowering_enabled=0
context_snapshot_llvm_enabled=0
```

## Stop Line

```text
no Program JSON ContextScope support
no MIR ContextScope lowering
no LLVM lowering
implicit root is not detached context propagation
```

## Verification

```bash
cargo test -q --lib runtime::context_snapshot
cargo test -q --lib runtime::global_hooks
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

```text
CONC-SOURCE-PARALLEL-001:
  decide source-level worker/parallel surface only after substrate safety rows.
```
