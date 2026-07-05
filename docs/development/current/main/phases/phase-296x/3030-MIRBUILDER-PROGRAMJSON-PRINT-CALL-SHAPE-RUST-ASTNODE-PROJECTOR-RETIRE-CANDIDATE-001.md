# 3030 - MIRBUILDER-PROGRAMJSON-PRINT-CALL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

After 3029 is parity-green, mark only the covered
`PrintCallShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector and does not claim
general `Expr.expr Call` ownership. It only records that covered
print-lowered calls can be produced by the `.hako` ProgramJSON traversal path.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_print_call_shape_scan_parity_gate.sh
```

The 3029 gate must prove:

```text
capability=ProgramJsonPrintCallShapeScanV1
output=PrintCallShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
general_expr_call_ownership=0
console_log_route_execution=0
backend_print_lowering=0
```

## Covered Rows

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

## Retire Candidate

```text
PrintCallShapeSnapshotV1
for covered ProgramJSON Expr.expr Call(name="env.console.log") rows
```

## Not Retired

```text
full Rust ASTNode projector
full print statement extractor
general Expr.expr Call ownership
console/log route execution
backend print lowering
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption
Source Selfhost
new ABI
```

## Acceptance

- the 3029 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- general call ownership and backend print lowering remain explicitly
  unclaimed.
