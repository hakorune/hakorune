# 3027 - MIRBUILDER-PROGRAMJSON-LOCAL-ARRAY-LITERAL-SHAPE-SCAN-CAPABILITY-001

Status: active

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

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- Array<T> lowering or runtime array allocation semantics;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
