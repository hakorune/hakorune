# 3006 - MIRBUILDER-PROGRAMJSON-RETURN-EXPR-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonReturnExprShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit a
`ReturnExprShapeSnapshotV1` token snapshot for covered `Return.expr` shapes.

## Minimum Rows

```text
top_return_int
top_return_var
top_return_bool_true
top_return_compare_var_lt_int
top_return_compare_var_eq_int
top_return_call_unsupported
if_then_return_int
if_else_return_var
```

## Required Output

```text
snapshot_kind=ReturnExprShapeSnapshotV1
top_return_expr_kind=...
if_then_return_expr_kind=...
if_else_return_expr_kind=...
supported_return_expr_count=...
unsupported_return_expr_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Return.expr` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported return expressions are reported with a stable token;
- the card can name a concrete return-expr Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_return_expr_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits `ReturnExprShapeSnapshotV1`
for covered top-level `Return.expr`, `If.then[0].Return.expr`, and
`If.else[0].Return.expr` shapes.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_return_expr_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonReturnExprShapeScanV1
parity_rows=8
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
MIRBUILDER-PROGRAMJSON-RETURN-EXPR-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
