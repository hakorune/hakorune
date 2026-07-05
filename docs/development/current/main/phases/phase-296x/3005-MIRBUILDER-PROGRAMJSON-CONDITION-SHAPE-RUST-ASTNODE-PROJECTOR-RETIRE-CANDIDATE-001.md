# 3005 - MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

Mark only the covered `ConditionShapeSnapshotV1` ProgramJSON traversal rows as
a Rust ASTNode projector retire-candidate.

Covered rows:

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

## Requires

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_condition_shape_scan_parity_gate.sh
```

## Retire Candidate

```text
Rust ASTNode projector slice:
  ConditionShapeSnapshotV1 for covered Loop.cond / If.cond ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full condition extractor
full loop_cond_continue_with_return facts extractor
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption for a full owner
Source Selfhost
```

## Acceptance

- retire-candidate fixture names only the covered condition-shape rows;
- guard requires the 3004 ProgramJSON condition-shape parity gate to be green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only for this covered slice.
