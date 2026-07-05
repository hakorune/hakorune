# 3031 - MIRBUILDER-PROGRAMJSON-THROW-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonThrowExprShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit a
`ThrowExprShapeSnapshotV1` token snapshot for covered `Throw.expr` shapes.

## Minimum Rows

```text
top_throw_int
top_throw_var
top_throw_str
top_throw_bool_true
top_throw_compare_var_lt_int
top_throw_call_unsupported
if_then_throw_int
if_else_throw_var
first_stmt_not_throw_unsupported
```

## Required Output

```text
snapshot_kind=ThrowExprShapeSnapshotV1
top_throw_expr_kind=...
if_then_throw_expr_kind=...
if_else_throw_expr_kind=...
supported_throw_expr_count=...
unsupported_throw_expr_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Throw.expr` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported throw expressions are reported with a stable token;
- the card can name a concrete throw-expr Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- exception runtime semantics or catch/finally matching;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_throw_expr_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonThrowExprShapeScanV1
output_contract=ThrowExprShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
exception_runtime_semantics=0
catch_finally_matching=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_throw_expr_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-throw-expr-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-THROW-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
