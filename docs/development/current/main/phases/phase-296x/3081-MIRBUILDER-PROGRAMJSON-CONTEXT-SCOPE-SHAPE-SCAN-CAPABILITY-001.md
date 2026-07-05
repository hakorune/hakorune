# 3081 - MIRBUILDER-PROGRAMJSON-CONTEXT-SCOPE-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonContextScopeShapeScanV1` as the next ProgramJSON
traversal capability.

The owner consumes ProgramJSON structure and emits a
`ContextScopeShapeSnapshotV1` token snapshot for covered `ContextScope`
statement shapes. It observes the context `value` kind and the first body
statement kind.

ProgramJSON v0 evidence is `src/macro/ast_json/joinir_compat.rs`, where
`ASTNode::ContextScope` emits `type: "ContextScope"` with `spelling`, `name`,
`declared_type`, `value`, and `body` fields.

## Minimum Rows

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

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_context_scope_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonContextScopeShapeScanV1
output_contract=ContextScopeShapeSnapshotV1
parity_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
context_lowering=0
context_runtime_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_context_scope_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-context-scope-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-CONTEXT-SCOPE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- context lowering or runtime context semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
