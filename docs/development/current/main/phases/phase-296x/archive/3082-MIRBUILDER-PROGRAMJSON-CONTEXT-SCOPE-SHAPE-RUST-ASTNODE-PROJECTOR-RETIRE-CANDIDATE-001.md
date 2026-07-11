# 3082 - MIRBUILDER-PROGRAMJSON-CONTEXT-SCOPE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3081 is green, mark only the covered `ContextScopeShapeSnapshotV1`
ProgramJSON traversal rows as a Rust ASTNode projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_context_scope_shape_scan_parity_gate.sh
```

## Covered Rows

```text
top_context_var_empty
top_context_int_local
top_context_str_return
top_context_bool_expr
top_context_var_loop
top_context_call_value_unsupported
top_context_body_break_unsupported
if_then_context_int_local
if_else_context_var_empty
first_stmt_local_unsupported
```

## Retire Candidate

```text
ContextScopeShapeSnapshotV1 for covered ProgramJSON ContextScope statement rows
```

## Not Retired

- full Rust ASTNode projector;
- full ContextScope extractor or lowerer;
- context lowering or runtime context semantics;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_context_scope_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-context-scope-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
