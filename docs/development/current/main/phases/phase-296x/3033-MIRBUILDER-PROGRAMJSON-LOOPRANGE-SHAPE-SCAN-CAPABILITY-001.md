# 3033 - MIRBUILDER-PROGRAMJSON-LOOPRANGE-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonLoopRangeShapeScanV1` as the next ProgramJSON traversal
capability.

The owner consumes ProgramJSON structure and emits a
`LoopRangeShapeSnapshotV1` token snapshot for covered `LoopRange` statement
shapes.

## Minimum Rows

```text
top_looprange_int_to_int_empty_body
top_looprange_var_to_int_continue_body
top_looprange_int_to_var_return_body
top_looprange_var_to_var_break_body
if_then_looprange_int_to_int_empty_body
if_else_looprange_var_to_int_continue_body
top_looprange_float_bound_unsupported
top_looprange_nested_loop_unsupported
first_stmt_not_looprange_unsupported
```

## Required Output

```text
snapshot_kind=LoopRangeShapeSnapshotV1
top_looprange_shape_kind=...
if_then_looprange_shape_kind=...
if_else_looprange_shape_kind=...
supported_looprange_count=...
unsupported_looprange_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `LoopRange.start`, `LoopRange.end`, and `LoopRange.body` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported bound/body shapes are reported with a stable token;
- the card can name a concrete LoopRange Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- loop lowering or iterator/range runtime semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_looprange_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonLoopRangeShapeScanV1
output_contract=LoopRangeShapeSnapshotV1
parity_rows=9
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
looprange_lowering=0
iterator_runtime_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_looprange_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-looprange-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOOPRANGE-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
