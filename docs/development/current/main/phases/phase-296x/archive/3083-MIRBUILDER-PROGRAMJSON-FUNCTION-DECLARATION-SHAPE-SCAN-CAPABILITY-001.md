# 3083 - MIRBUILDER-PROGRAMJSON-FUNCTION-DECLARATION-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonFunctionDeclarationShapeScanV1` as the next ProgramJSON
traversal capability.

The owner consumes ProgramJSON structure and emits a
`FunctionDeclarationShapeSnapshotV1` token snapshot for covered top-level
`FunctionDeclaration` shapes. It observes return-type presence and the first
body statement kind.

ProgramJSON v0 evidence is `src/macro/ast_json/joinir_compat.rs`, where
`ASTNode::FunctionDeclaration` emits `kind: "FunctionDeclaration"` with
`name`, `params`, `return_type`, and `body` fields.

## Minimum Rows

```text
fn_no_ret_empty
fn_ret_empty
fn_no_ret_local
fn_ret_return
fn_no_ret_expr
fn_no_ret_loop
fn_ret_if
fn_missing_name_unsupported
fn_body_break_unsupported
first_stmt_local_unsupported
```

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_function_declaration_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonFunctionDeclarationShapeScanV1
output_contract=FunctionDeclarationShapeSnapshotV1
parity_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
function_lowering=0
function_route_selection=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_function_declaration_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-function-declaration-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-FUNCTION-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- function lowering or route selection;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
