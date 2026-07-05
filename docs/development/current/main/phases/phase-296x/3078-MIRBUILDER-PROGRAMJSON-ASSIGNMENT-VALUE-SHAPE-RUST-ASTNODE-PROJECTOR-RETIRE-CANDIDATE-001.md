# 3078 - MIRBUILDER-PROGRAMJSON-ASSIGNMENT-VALUE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: landed

## Scope

After 3077 is green, mark only the covered
`AssignmentValueShapeSnapshotV1` ProgramJSON traversal rows as a Rust ASTNode
projector retire-candidate.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_assignment_value_shape_scan_parity_gate.sh
```

## Covered Rows

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

## Retire Candidate

```text
AssignmentValueShapeSnapshotV1 for covered ProgramJSON Local.name + Local.expr
assignment-value rows
```

## Not Retired

- full Rust ASTNode projector;
- full assignment-value extractor or lowerer;
- legacy `Assign` / `Assignment` compatibility proof;
- RecipeMatcher, route selection, MIR lowering, MIR mutation, or ID allocation;
- ProgramJSON full parser;
- HakoAdoption for a full owner;
- Source Selfhost.

## Acceptance

- retire fixture names only the covered assignment-value rows;
- guard requires 3077 parity green;
- runtime Rust projector dependency removal remains `0`;
- Rust projector remains oracle-only;
- legacy `Assign` proof remains unclaimed.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_assignment_value_shape_retire_rust_astnode_projector_candidate_guard.sh
```

Decision fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-assignment-value-shape-retire-rust-astnode-projector-candidate-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-CAPABILITY-BATCH-CONTINUATION
```
