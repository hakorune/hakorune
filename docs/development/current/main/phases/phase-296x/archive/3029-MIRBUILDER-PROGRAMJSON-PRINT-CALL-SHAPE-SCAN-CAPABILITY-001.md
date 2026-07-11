# 3029 - MIRBUILDER-PROGRAMJSON-PRINT-CALL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonPrintCallShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit a
`PrintCallShapeSnapshotV1` token snapshot for covered print-lowered call
shapes:

```text
Print(...)
  -> Expr.expr Call(name="env.console.log", args=[...])
```

This is a statement-lowering shape check, not a general call-expression
claim. General `Expr.expr Call` rows remain owned by
`ProgramJsonExprCallShapeScanV1`.

## Minimum Rows

```text
print_call_int_arg
print_call_var_arg
print_call_str_arg
print_call_bool_arg
print_call_compare_var_lt_int_arg
if_then_print_call_int_arg
if_else_print_call_var_arg
non_print_call_unsupported
first_stmt_not_expr_unsupported
```

## Required Output

```text
snapshot_kind=PrintCallShapeSnapshotV1
top_print_call_arg_kind=...
if_then_print_call_arg_kind=...
if_else_print_call_arg_kind=...
supported_print_call_count=...
unsupported_print_call_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Expr.expr Call(name="env.console.log")` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- non-print calls and unsupported argument shapes are reported with stable
  tokens;
- the card can name a concrete print-call Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- claiming general `Expr.expr Call` ownership;
- console/log route execution or backend print lowering;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_print_call_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonPrintCallShapeScanV1
output_contract=PrintCallShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
general_expr_call_ownership=0
console_log_route_execution=0
backend_print_lowering=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_print_call_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-print-call-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-PRINT-CALL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
