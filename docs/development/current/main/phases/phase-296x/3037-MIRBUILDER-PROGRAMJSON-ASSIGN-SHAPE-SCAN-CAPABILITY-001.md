# 3037 - MIRBUILDER-PROGRAMJSON-ASSIGN-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonAssignShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits an `AssignShapeSnapshotV1`
token snapshot for covered `Assign` statement shapes.

## Minimum Rows

```text
top_assign_var_int
top_assign_var_var
top_assign_var_bool_true
top_assign_var_compare_lt
top_assign_var_binary_add
top_assign_target_field_unsupported
top_assign_call_unsupported
if_then_assign_var_int
if_else_assign_var_var
first_stmt_not_assign_unsupported
```

## Required Output

```text
snapshot_kind=AssignShapeSnapshotV1
top_assign_shape_kind=...
if_then_assign_shape_kind=...
if_else_assign_shape_kind=...
supported_assign_count=...
unsupported_assign_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  assignment positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported assignment targets or values are reported with stable tokens;
- the card can name a concrete assignment Rust ASTNode projector slice as a
  retire-candidate after parity is green.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_assign_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits `AssignShapeSnapshotV1` for
covered top-level `Assign`, `If.then[0].Assign`, and `If.else[0].Assign`
shapes. It observes `target` and `value` fields only; it does not mutate MIR.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_assign_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonAssignShapeScanV1
parity_rows=10
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
MIRBUILDER-PROGRAMJSON-ASSIGN-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
