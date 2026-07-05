# 3010 - MIRBUILDER-PROGRAMJSON-ASSIGNMENT-VALUE-SHAPE-SCAN-CAPABILITY-001

Status: active

## Scope

Implement `ProgramJsonAssignmentValueShapeScanV1` as the next ProgramJSON
traversal capability.

The owner must consume ProgramJSON structure and emit an
`AssignmentValueShapeSnapshotV1` token snapshot for covered assignment value
shapes.

## Minimum Rows

```text
assign_int_then_return_var
assign_bool_then_return_var
assign_var_then_return_var
assign_compare_var_lt_int_then_return_var
assign_compare_var_eq_int_then_return_var
assign_call_unsupported_then_return_var
if_then_assign_int_then_return_var
if_else_assign_var_then_return_var
```

## Required Output

```text
snapshot_kind=AssignmentValueShapeSnapshotV1
top_assignment_value_kind=...
if_then_assignment_value_kind=...
if_else_assignment_value_kind=...
supported_assignment_value_count=...
unsupported_assignment_value_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  assignment value positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported assignment values are reported with a stable token;
- the card can name a concrete assignment-value Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
