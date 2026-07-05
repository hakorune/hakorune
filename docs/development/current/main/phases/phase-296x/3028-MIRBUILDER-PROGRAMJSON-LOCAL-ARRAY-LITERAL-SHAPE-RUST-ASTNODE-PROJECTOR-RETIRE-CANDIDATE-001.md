# 3028 - MIRBUILDER-PROGRAMJSON-LOCAL-ARRAY-LITERAL-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001

Status: active

## Scope

After 3027 is parity-green, mark only the covered
`LocalArrayLiteralShapeSnapshotV1` ProgramJSON rows as a Rust ASTNode projector
retire-candidate.

This card does not retire the full Rust ASTNode projector. It only records that
the covered `Local.expr ArrayLiteral` snapshot rows can be produced by the
`.hako` ProgramJSON traversal path and kept out of the runtime migration path
once the caller is wired to the new snapshot owner.

## Requires

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_array_literal_shape_scan_parity_gate.sh
```

The 3027 gate must prove:

```text
capability=ProgramJsonLocalArrayLiteralShapeScanV1
output=LocalArrayLiteralShapeSnapshotV1
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
array_lowering_semantics=0
runtime_array_allocation_semantics=0
```

## Covered Rows

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

## Retire Candidate

```text
LocalArrayLiteralShapeSnapshotV1
for covered Local.expr ArrayLiteral ProgramJSON rows
```

## Not Retired

```text
full Rust ASTNode projector
full local array literal extractor
Array<T> lowering semantics
runtime array allocation semantics
RecipeMatcher
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
HakoAdoption
Source Selfhost
new ABI
```

## Acceptance

- the 3027 parity gate is green;
- retire-candidate scope names only the covered rows above;
- Rust projector runtime dependency removal remains `0`;
- Rust projector oracle-only state remains documented;
- Array lowering and runtime allocation semantics remain explicitly unclaimed.
