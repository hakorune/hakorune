# 3004 - MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-SCAN-CAPABILITY-001

Status: active

## Scope

Implement `ProgramJsonConditionShapeScanV1` as the next ProgramJSON traversal
capability.

The owner must consume ProgramJSON structure and emit a
`ConditionShapeSnapshotV1` token snapshot for loop and if condition shapes.

## Minimum Rows

```text
loop_cond_compare_var_lt_int
loop_cond_compare_var_eq_int
loop_cond_bool_true
loop_cond_var_bool
if_cond_compare_var_eq_int
if_cond_compare_var_lt_int
if_cond_bool_true
unsupported_call_condition
```

## Required Output

```text
snapshot_kind=ConditionShapeSnapshotV1
loop_cond_kind=...
if_cond_kind=...
supported_condition_count=...
unsupported_condition_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for `Loop.cond`
  and `If.cond`;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported conditions fail fast with a stable token;
- the card can name a concrete condition-shape Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
