# 3206 - ProgramJSON RecipeBodies Loop Body Multi-Body Arena Parity

Status: Landed
Date: 2026-07-07
Token: `MIRBUILDER-PROGRAMJSON-RECIPEBODIES-LOOP-BODY-MULTI-BODY-ARENA-PARITY-001`

## Scope

Implement the first loop-body RecipeBodies-like arena DTO in `.hako`:

```text
Program.body = [
  Local,
  Loop(body=[Assignment]),
  Return
]
```

The owner consumes Program(JSON v0) through the existing phase-state parser,
uses the existing RecipeItem tree, and emits a map-backed arena summary:

```text
body0 = [Stmt:Local, LoopRef(body=1), Stmt:Return]
body1 = [Stmt:Assignment]
```

## Contract

- The implementation owner is
  `ProgramJsonRecipeBodiesLoopBodyArenaBuilderBox`.
- The arena is a DTO proof only. It is not runtime `RecipeBodies`.
- Item kinds are kept as numeric codes inside maps and only rendered into
  stable tokens at the summary boundary.
- The gate proves AOT runtime parity for the covered row and verifies that MIR
  JSON observes the `ProgramJsonV0PhaseStateBox.parse/2` route.

## Evidence

```text
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_loop_body_multi_body_arena_parity_gate.sh
```

Expected summary:

```text
programjson_traversal_used=1
recipe_root_used=1
structured_result_map_built=1
bodyid_stmtref_tokens_emitted=1
loop_body_id_emitted=1
body_count=2
runtime_parity_green=1
```

## Non-Claims

```text
recipe_bodies_materialization=0
runtime_recipe_bodies_arena=0
full_recipe_matcher_execution=0
verifier_policy_reimplementation=0
route_selection=0
mir_mutation=0
id_allocation=0
backend_lowering_claim=0
directabi_route_publication_claim=0
runtime_route_switch=0
source_selfhost_claim=0
programjson_full_parser_claim=0
new_backend_route=0
new_abi=0
```

## Next

`MIRBUILDER-PROGRAMJSON-RECIPEBODIES-LOOP-BODY-MULTI-BODY-ARENA-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001`
