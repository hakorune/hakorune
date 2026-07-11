# 3200 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-PARITY-001

Status: landed

## Scope

Implement `ProgramJsonRecipeBodiesOneShapeArenaBuilderBox`.

This owner consumes Program(JSON v0), uses the existing `recipe_root`, builds a
map-backed one-shape RecipeBodies-like arena DTO, and summarizes it through a
canonical token string for parity.

Covered rows:

```text
empty_stmt_only_body
single_local_stmt_body
local_then_print_stmt_body
```

## Contract

The structured result map contains:

```text
root_body_id = 0
body_count = 1
bodies[0].body_id = 0
bodies[0].item_count = N
bodies[0].items[i] = { kind: StmtRef, body_id: 0, stmt_idx: i, item_code: ... }
```

This is a RecipeBodies-like arena DTO only. It is not runtime `RecipeBodies`
publication, not `RecipeBodies::bodies` access, and not a route switch.
`item_code` is intentionally numeric inside the arena DTO; the canonical
summary converts it back to `Stmt:Local` / `Stmt:Print` tokens.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_one_shape_arena_builder_parity_gate.sh
```

Expected result:

```text
one_shape_arena_builder_implemented=1
structured_result_map_built=1
runtime_recipe_bodies_arena=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Non-Claims

```text
runtime RecipeBodies arena
RecipeBodies::bodies access
full RecipeMatcher execution
verifier policy reimplementation
route selection
MIR lowering
MIR mutation
ID allocation
DirectAbi route publication
runtime route switch
ProgramJSON full parser
new backend route
new ABI
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-ONE-SHAPE-ARENA-BUILDER-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
