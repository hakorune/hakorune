# 3025 - MIRBUILDER-PROGRAMJSON-EXPR-LOGICAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprLogicalShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit an
`ExprLogicalShapeSnapshotV1` token snapshot for covered expression-statement
logical expression shapes.

## Minimum Rows

```text
expr_logical_var_and_var
expr_logical_var_or_var
expr_logical_bool_and_var
expr_logical_var_or_bool
expr_logical_nested_unsupported
if_then_expr_logical_var_and_var
if_else_expr_logical_var_or_var
compare_expr_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprLogicalShapeSnapshotV1
top_expr_logical_shape_kind=...
if_then_expr_logical_shape_kind=...
if_else_expr_logical_shape_kind=...
supported_expr_logical_count=...
unsupported_expr_logical_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` logical positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported logical operand shapes are reported with a stable token;
- the card can name a concrete expr-logical Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_logical_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprLogicalShapeScanV1
output_contract=ExprLogicalShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
boolean_short_circuit_lowering_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_logical_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-expr-logical-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXPR-LOGICAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- boolean short-circuit lowering semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
