# 3039 - MIRBUILDER-PROGRAMJSON-ENUM-MATCH-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonEnumMatchShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits an `EnumMatchShapeSnapshotV1`
token snapshot for covered `EnumMatch` expression shapes.

## Minimum Rows

```text
top_option_some_bool_true_none_bool_false
top_option_some_bind_var_none_null
top_result_enum_unsupported
top_option_scrutinee_call_unsupported
top_option_arm_order_unsupported
top_option_three_arms_unsupported
if_then_option_some_bool_true_none_bool_false
if_else_option_some_bind_var_none_null
first_stmt_not_enum_match_unsupported
```

## Required Output

```text
snapshot_kind=EnumMatchShapeSnapshotV1
top_enum_match_shape_kind=...
if_then_enum_match_shape_kind=...
if_else_enum_match_shape_kind=...
supported_enum_match_count=...
unsupported_enum_match_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  EnumMatch positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported enum names, scrutinee kinds, arm order, or arm count are reported
  with stable tokens;
- the card can name a concrete EnumMatch Rust ASTNode projector slice as a
  retire-candidate after parity is green.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_enum_match_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits `EnumMatchShapeSnapshotV1`
for covered top-level `EnumMatch`, `If.then[0].EnumMatch`, and
`If.else[0].EnumMatch` shapes. It observes `enum`, `scrutinee`, `arms`,
`variant`, `bind`, and `expr` fields only.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_enum_match_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonEnumMatchShapeScanV1
parity_rows=9
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
match_lowering_semantics=0
enum_runtime_semantics=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-ENUM-MATCH-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- match lowering or enum runtime semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
