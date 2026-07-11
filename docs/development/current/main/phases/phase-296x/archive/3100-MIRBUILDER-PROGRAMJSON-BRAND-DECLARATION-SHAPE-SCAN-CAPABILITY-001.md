# 3100 - MIRBUILDER-PROGRAMJSON-BRAND-DECLARATION-SHAPE-SCAN-CAPABILITY-001

Status: landed

## Scope

Add a real `.hako` ProgramJSON traversal capability for covered
`BrandDeclaration` shapes.

Owner:

```text
ProgramJsonBrandDeclarationShapeScanV1
```

Output:

```text
BrandDeclarationShapeSnapshotV1
```

The traversal reads actual Program(JSON v0) structure:

```text
Program.body[0] -> BrandDeclaration.name / BrandDeclaration.underlying_type
```

## Covered Rows

```text
brand_i64
brand_string
brand_bool
brand_arraybox
brand_mapbox
brand_custom_other
brand_missing_underlying_unsupported
first_stmt_local_unsupported
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_brand_declaration_shape_scan_parity_gate.sh
```

Green output:

```text
owner=ProgramJsonBrandDeclarationShapeScanV1
parity_rows=8
execution_backend=aot
token_snapshot_parity=green
programjson_traversal_used=1
string_only_facade=0
rust_astnode_projector_retire_candidate=1
```

## Non-Claims

```text
brand_type_resolution = 0
brand_lowering = 0
full_recipe_matcher_execution = 0
route_selection = 0
mir_mutation = 0
backend_lowering = 0
id_allocation = 0
programjson_full_parser_claim = 0
hako_adopted_for_full_owner = 0
source_selfhost_claim = 0
new_backend_route = 0
new_abi = 0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-BRAND-DECLARATION-SHAPE-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
