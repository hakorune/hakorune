# 3210 - HAKO-PROGRAMJSON-RECIPEBODIES-ARRAY-HELPER-TOTAL-MAP-CONTRACT-001

Status: landed

## Scope

Close the nearest remaining RecipeBodies array-helper risk found after 3208.

`ProgramJsonRecipeBodiesOneShapeArenaBuilderBox._build_body_items` returned a
fresh array from an unannotated helper on an AOT/MIR JSON path. The cleanup is
to keep the array local to the `MapBox` body builder and avoid publishing a raw
ArrayBox helper return.

## Implementation

- Remove `_build_body_items`.
- Build `StmtRef` arrays inside `_body_map(...): MapBox`.
- Keep the one-shape arena DTO contract unchanged.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_array_helper_total_map_contract_guard.sh
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_one_shape_arena_builder_parity_gate.sh
```

## Non-Claims

```text
aot_array_return_widening=0
by_name_aot_exception=0
programjson_new_shape=0
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
runtime_route_switch=0
source_selfhost_claim=0
new_backend_route=0
new_abi=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-LOOP-BODY-ARENA-NEXT-CONTRACT-SELECTION-001
```
