# 3004 - MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-SCAN-CAPABILITY-001

Status: landed

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

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_condition_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits
`ConditionShapeSnapshotV1` for covered Loop/If condition shapes. It reads
`Loop.cond`, optional first/second `If.cond`, `Compare.lhs`, `Compare.rhs`,
`Compare.op`, and `Bool.value` through ProgramJSON field helpers.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_condition_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonConditionShapeScanV1
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
MIRBUILDER-PROGRAMJSON-CONDITION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
