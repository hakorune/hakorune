# 3027 - MIRBUILDER-PROGRAMJSON-LOCAL-ARRAY-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonLocalArrayLiteralShapeScanV1` as the next ProgramJSON
traversal capability.

The owner must consume ProgramJSON structure and emit a
`LocalArrayLiteralShapeSnapshotV1` token snapshot for covered `Local.expr`
array literal initializer shapes.

## Minimum Rows

```text
local_array_int_empty
local_array_int_one
local_array_int_two
local_array_bool_one
local_array_str_one
local_array_nested_unsupported
if_then_local_array_int_one
if_else_local_array_bool_one
expr_array_literal_unsupported
first_stmt_not_local_unsupported
```

## Required Output

```text
snapshot_kind=LocalArrayLiteralShapeSnapshotV1
top_local_array_literal_shape_kind=...
if_then_local_array_literal_shape_kind=...
if_else_local_array_literal_shape_kind=...
supported_local_array_literal_count=...
unsupported_local_array_literal_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered
  `Local.expr` `ArrayLiteral` positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported array element shapes are reported with a stable token;
- the card can name a concrete local-array-literal Rust ASTNode projector slice
  as retire-candidate after parity is green.

## Task Cut

```text
A. ProgramJSON scanner vocabulary
   Add only the stable field-key vocabulary needed by this traversal
   (`declared_type`, `element_type`, `elements`). This is scanner support, not
   Array<T> lowering or runtime allocation semantics.

B. `.hako` traversal capability
   Implement ProgramJsonLocalArrayLiteralShapeScanV1 so the output snapshot is
   produced by walking ProgramJSON fields, not by accepting prebuilt tokens.

C. Parity gate and handoff
   Prove the 10 covered rows with AOT parity, then hand off to 3028 for the
   scoped Rust ASTNode projector retire-candidate card.
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- Array<T> lowering or runtime array allocation semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_array_literal_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonLocalArrayLiteralShapeScanV1
output_contract=LocalArrayLiteralShapeSnapshotV1
parity_rows=10
execution_backend=aot
aot_execution_status=green
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
array_lowering_semantics=0
runtime_array_allocation_semantics=0
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_local_array_literal_shape_scan.hako
```

Fixture:

```text
docs/development/current/main/design/fixtures/rust-lifecycle/
  mirbuilder-programjson-local-array-literal-shape-scan-parity-v0.json
```

## Next

```text
MIRBUILDER-PROGRAMJSON-LOCAL-ARRAY-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
