# 3010 - MIRBUILDER-PROGRAMJSON-EXPR-CALL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonExprCallShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit an
`ExprCallShapeSnapshotV1` token snapshot for covered expression-statement call
shapes.

## Why Not Assignment

Current ProgramJSON v0 lowering maps assignment statements into `Local` nodes:

```text
ProgramJSON v0 assignment lowering
  -> { "type": "Local", "name": ..., "expr": ... }
```

Therefore an `Assignment`/`Assign` capability would target legacy compatibility
fixtures rather than the current ProgramJSON route. Use `Expr` statement calls
as the next real ProgramJSON traversal slice.

## Minimum Rows

```text
expr_call_no_args
expr_call_int_arg
expr_call_var_arg
expr_call_bool_arg
expr_call_compare_var_lt_int_arg
expr_call_compare_var_eq_int_arg
expr_call_call_arg_unsupported
if_then_expr_call_int_arg
if_else_expr_call_var_arg
```

## Required Output

```text
snapshot_kind=ExprCallShapeSnapshotV1
top_expr_call_arg_kind=...
if_then_expr_call_arg_kind=...
if_else_expr_call_arg_kind=...
supported_expr_call_count=...
unsupported_expr_call_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr` call positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported call argument shapes are reported with a stable token;
- the card can name a concrete expr-call Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_expr_call_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits `ExprCallShapeSnapshotV1`
for covered top-level `Expr.expr Call`, `If.then[0].Expr.expr Call`, and
`If.else[0].Expr.expr Call` shapes.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_expr_call_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonExprCallShapeScanV1
parity_rows=9
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-EXPR-CALL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- legacy-only `Assign` / `Assignment` compatibility fixtures as proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
