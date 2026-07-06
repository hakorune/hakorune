# 3199 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-MINIMAL-DTO-NEXT-CONTRACT-SELECTION-001

Status: landed

## Decision

After the minimal DTO retire-candidate, select:

```text
B_ONE_SHAPE_ARENA_BUILDER_PARITY
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-PARITY-001
```

## Why

3197/3198 proved the snapshot-local `BodyId`/`StmtRef` reference surface for
three StmtOnly rows. The next useful step is not another string-only DTO and not
full `RecipeBodies` runtime publication. It is one structured, map-backed
arena-shaped result for the same minimal StmtOnly rows.

This keeps the migration moving from DTO text toward the Layer4 RecipeBodies
shape while staying below verifier policy, route selection, lowering, mutation,
ID allocation, and runtime route switching.

## Acceptance For Next Card

```text
must consume ProgramJSON
must build structured result map
must expose root_body_id
must expose body item refs
must compare against the minimal DTO oracle
must keep runtime_route_switch=0
```

Rows:

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

## Forbidden In Next Card

```text
full RecipeBodies runtime publication
RecipeBodies::bodies direct access
full RecipeMatcher execution
verifier policy reimplementation
route selection
MIR lowering
MIR mutation
ID allocation
runtime route switch
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_minimal_dto_next_contract_selection_guard.sh
```

Expected result:

```text
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-PARITY-001
one_shape_arena_builder_implemented=0
recipe_bodies_materialization=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-PARITY-001
```
