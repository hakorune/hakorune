# 3041 - MIRBUILDER-PROGRAMJSON-LOCAL-STRING-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonLocalStringLiteralShapeScanV1` as the next ProgramJSON
traversal capability.

The owner consumes ProgramJSON structure and emits a
`LocalStringLiteralShapeSnapshotV1` token snapshot for covered `Local.expr`
string literal shapes.

## Minimum Rows

```text
top_local_str_hello
top_local_str_empty
top_local_str_err
top_local_str_other
top_local_int_unsupported
top_local_var_unsupported
if_then_local_str_hello
if_else_local_str_err
first_stmt_not_local_unsupported
```

## Required Output

```text
snapshot_kind=LocalStringLiteralShapeSnapshotV1
top_local_string_literal_shape_kind=...
if_then_local_string_literal_shape_kind=...
if_else_local_string_literal_shape_kind=...
supported_local_string_literal_count=...
unsupported_local_string_literal_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Local.expr` string literal positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported initializer kinds are reported with stable tokens;
- the card can name a concrete local string literal Rust ASTNode projector slice
  as a retire-candidate after parity is green.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_local_string_literal_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits
`LocalStringLiteralShapeSnapshotV1` for covered top-level `Local.expr`,
`If.then[0].Local.expr`, and `If.else[0].Local.expr` shapes.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_string_literal_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonLocalStringLiteralShapeScanV1
parity_rows=9
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
stringbox_materialization=0
string_literal_lowering=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOCAL-STRING-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- StringBox materialization or string literal lowering;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
