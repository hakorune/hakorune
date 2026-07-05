# 3055 - MIRBUILDER-PROGRAMJSON-FASTMEM-REGION-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a narrow `.hako` ProgramJSON traversal owner for covered `FastMemRegion`
statement shapes.

This card moves one more FastMemRegion token snapshot away from Rust ASTNode
projection. It reads the `contract` field and the first statement in `body`
only. It does not claim fastmem lowering, contract execution, route selection,
MIR mutation, backend lowering, ID allocation, ProgramJSON full parser support,
HakoAdoption, or Source Selfhost.

## Owner

```text
ProgramJsonFastMemRegionShapeScanV1
```

Implementation:

```text
lang/src/compiler/mirbuilder/program_json_fastmem_region_shape_scan.hako
```

Input:

```text
ProgramJSON v0
```

Output:

```text
FastMemRegionShapeSnapshotV1
```

## Covered Rows

```text
top_fastmem_empty
top_fastmem_local
top_fastmem_return
top_fastmem_break
top_fastmem_continue
top_fastmem_loop
top_fastmem_expr
if_then_fastmem_local
if_else_fastmem_empty
first_stmt_local_unsupported
```

## Guard

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_fastmem_region_shape_scan_parity_gate.sh
```

Expected contract:

```text
owner=ProgramJsonFastMemRegionShapeScanV1
output_contract=FastMemRegionShapeSnapshotV1
parity_rows=10
aot_execution_status=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
fastmem_lowering=0
contract_execution=0
source_selfhost_claim=0
```

## Non-Claims

```text
fastmem_lowering=0
contract_execution=0
source_selfhost_claim=0
mir_mutation=0
id_allocation=0
backend_lowering=0
full_recipe_matcher_execution=0
route_selection=0
runtime_fallback=0
new_backend_route=0
new_abi=0
programjson_full_parser_claim=0
hako_adopted_for_full_owner=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-FASTMEM-REGION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
