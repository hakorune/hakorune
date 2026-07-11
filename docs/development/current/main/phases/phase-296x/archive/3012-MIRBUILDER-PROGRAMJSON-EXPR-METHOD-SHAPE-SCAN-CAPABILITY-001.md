# 3012 - MIRBUILDER-PROGRAMJSON-EXPR-METHOD-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprMethodShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit an
`ExprMethodShapeSnapshotV1` token snapshot for covered expression-statement
method-call shapes.

## Minimum Rows

```text
expr_method_no_args
expr_method_int_arg
expr_method_var_arg
expr_method_bool_arg
expr_method_compare_var_lt_int_arg
expr_method_compare_var_eq_int_arg
expr_method_call_arg_unsupported
if_then_expr_method_int_arg
if_else_expr_method_var_arg
```

## Required Output

```text
snapshot_kind=ExprMethodShapeSnapshotV1
top_expr_method_arg_kind=...
if_then_expr_method_arg_kind=...
if_else_expr_method_arg_kind=...
supported_expr_method_count=...
unsupported_expr_method_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` method positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported method argument shapes are reported with a stable token;
- the card can name a concrete expr-method Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_method_shape_scan_parity_gate.sh
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_expr_method_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-expr-method-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-METHOD-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
