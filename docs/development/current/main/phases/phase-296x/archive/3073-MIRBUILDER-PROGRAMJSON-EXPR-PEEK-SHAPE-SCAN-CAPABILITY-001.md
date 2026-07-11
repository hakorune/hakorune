# 3073 - MIRBUILDER-PROGRAMJSON-EXPR-PEEK-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprPeekShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits an `ExprPeekShapeSnapshotV1`
token snapshot for covered expression-statement `Peek` expression shapes.

## Minimum Rows

```text
expr_peek_var_one_arm_null_else
expr_peek_var_two_arm_int_else
expr_peek_call_one_arm_str_else
expr_peek_var_no_arm_null_else
expr_peek_var_three_arm_unsupported
if_then_expr_peek_var_one_arm_null_else
if_else_expr_peek_var_two_arm_int_else
expr_ternary_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=ExprPeekShapeSnapshotV1
top_expr_peek_shape_kind=...
if_then_expr_peek_shape_kind=...
if_else_expr_peek_shape_kind=...
supported_expr_peek_count=...
unsupported_expr_peek_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` Peek positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported Peek shapes are reported with a stable token;
- the card can name a concrete expr-peek Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_peek_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprPeekShapeScanV1
output_contract=ExprPeekShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
peek_parse_lowering_semantics=0
pattern_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_peek_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/mirbuilder-programjson-expr-peek-shape-scan-parity-v0.json
```

Next:

```text
MIRBUILDER-PROGRAMJSON-EXPR-PEEK-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- PeekParse lowering semantics or pattern semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
