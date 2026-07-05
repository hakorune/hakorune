# 3077 - MIRBUILDER-PROGRAMJSON-ASSIGNMENT-VALUE-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonAssignmentValueShapeScanV1` as the next ProgramJSON
traversal capability.

The owner consumes current ProgramJSON v0 structure for `Local.name` and
`Local.expr`, then emits an `AssignmentValueShapeSnapshotV1` token snapshot for
covered assignment-value shapes. This is not a legacy `Assign`/`Assignment`
statement proof.

## Source Authority

```text
src/mir/builder/control_flow/plan/generic_loop/facts/extract/collection.rs::assignment_value_increments_var
src/mir/builder/control_flow/plan/normalizer/loop_body_lowering.rs::lower_assignment_value
```

## Minimum Rows

```text
local_i_expr_i_add_1
local_i_expr_1_add_i
local_i_expr_i_sub_1
local_i_expr_j_add_1_not_self
local_i_expr_i_mul_1_unsupported
local_i_expr_call_unsupported
if_then_local_i_expr_i_add_1
if_else_local_i_expr_i_sub_1
first_stmt_not_local_unsupported
```

## Required Output

```text
snapshot_kind=AssignmentValueShapeSnapshotV1
top_assignment_value_shape_kind=...
if_then_assignment_value_shape_kind=...
if_else_assignment_value_shape_kind=...
self_increment_candidate_count=...
self_decrement_candidate_count=...
unsupported_value_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Local.name` and `Local.expr` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported value expressions produce stable unsupported tokens;
- card lands implementation, fixture, and AOT parity gate together;
- next card may mark only covered `AssignmentValueShapeSnapshotV1` rows as a
  Rust ASTNode projector retire-candidate.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_assignment_value_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits
`AssignmentValueShapeSnapshotV1` for covered top-level `Local`, `If.then[0]`
`Local`, and `If.else[0]` `Local` assignment-value shapes. It observes
`Local.name`, `Local.expr`, `Binary.op`, `Binary.lhs`, and `Binary.rhs` only.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_assignment_value_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonAssignmentValueShapeScanV1
parity_rows=9
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
legacy_assign_proof=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- legacy-only `Assign` / `Assignment` rows as the proof of this capability;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.

## Next

```text
MIRBUILDER-PROGRAMJSON-ASSIGNMENT-VALUE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
