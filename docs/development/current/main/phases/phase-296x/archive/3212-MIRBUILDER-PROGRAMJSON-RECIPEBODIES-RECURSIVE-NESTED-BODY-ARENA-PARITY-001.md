# 3212 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RECURSIVE-NESTED-BODY-ARENA-PARITY-001

Status: landed

## Scope

Implement the selected recursive nested RecipeBodies arena DTO owner.

Covered row:

```text
local_loop_body_if_branch_return
```

The ProgramJSON route builds and summarizes:

```text
body0 = [Stmt:Local, LoopRef(body=1), Stmt:Return]
body1 = [IfRef(then=2, else=3), Stmt:Assignment]
body2 = [Exit:Return]
body3 = []
```

This composes the branch and loop body-reference surfaces already proved in
3203/3204 and 3206/3207. It remains a map-backed DTO only.

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_recursive_nested_body_arena_parity_gate.sh
```

Expected result:

```text
recursive_nested_body_arena_implemented=1
body_count=4
runtime_recipe_bodies_arena=0
recipe_bodies_materialization=0
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
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RECURSIVE-NESTED-BODY-ARENA-RUST-ASTNODE-PROJECTOR-RETIRE-CANDIDATE-001
```
