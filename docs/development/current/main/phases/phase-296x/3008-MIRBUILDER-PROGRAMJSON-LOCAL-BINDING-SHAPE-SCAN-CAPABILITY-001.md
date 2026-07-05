# 3008 - MIRBUILDER-PROGRAMJSON-LOCAL-BINDING-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Implement `ProgramJsonLocalBindingShapeScanV1` as the next ProgramJSON
traversal capability.

The owner must consume ProgramJSON structure and emit a
`LocalBindingShapeSnapshotV1` token snapshot for covered local-binding shapes.

## Minimum Rows

```text
local_int_then_return_var
local_bool_then_return_var
local_var_alias_then_return_var
local_compare_var_lt_int_then_return_var
local_compare_var_eq_int_then_return_var
local_call_unsupported_then_return_var
if_then_local_int_then_return_var
if_else_local_var_then_return_var
```

## Required Output

```text
snapshot_kind=LocalBindingShapeSnapshotV1
top_local_init_kind=...
if_then_local_init_kind=...
if_else_local_init_kind=...
supported_local_init_count=...
unsupported_local_init_count=...
```

## Acceptance

- `.hako` implementation traverses ProgramJSON object fields for covered local
  binding positions;
- parity gate compares canonical fields against Rust ASTNode-token oracle rows;
- unsupported local initializers are reported with a stable token;
- the card can name a concrete local-binding Rust ASTNode projector slice as
  retire-candidate after parity is green.

## Implementation

```text
lang/src/compiler/mirbuilder/program_json_local_binding_shape_scan.hako
```

The owner consumes ProgramJSON structure and emits
`LocalBindingShapeSnapshotV1` for covered top-level `Local.expr`,
`If.then[0].Local.expr`, and `If.else[0].Local.expr` shapes.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_local_binding_shape_scan_parity_gate.sh
```

Result:

```text
owner=ProgramJsonLocalBindingShapeScanV1
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
MIRBUILDER-PROGRAMJSON-LOCAL-BINDING-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```

## Forbidden

- prebuilt token snapshot input;
- source string / regex / raw contains proof;
- RecipeMatcher execution;
- MIR mutation, backend lowering, route selection, ID allocation, or new ABI;
- full Rust ASTNode projector retirement, ProgramJSON full parser claim,
  HakoAdoption, or Source Selfhost claim.
