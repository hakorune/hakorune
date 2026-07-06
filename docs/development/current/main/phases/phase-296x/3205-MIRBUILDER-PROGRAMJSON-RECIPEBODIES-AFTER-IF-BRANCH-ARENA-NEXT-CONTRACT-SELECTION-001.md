# 3205 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-IF-BRANCH-ARENA-NEXT-CONTRACT-SELECTION-001

Status: landed

## Decision

After the If-branch multi-body arena retire-candidate, select:

```text
A_LOOP_BODY_MULTI_BODY_ARENA_PARITY
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-LOOP-BODY-MULTI-BODY-ARENA-PARITY-001
```

## Why

3203/3204 proved root body to branch body references:

```text
IfRef -> then body 1
         else body 2
```

The next RecipeBodies property is the loop-side body reference:

```text
root body 0
  LoopRef -> body 1
```

This keeps the work in the map-backed arena DTO layer and still avoids full
RecipeMatcher execution, route selection, lowering, mutation, ID allocation,
and runtime route switching.

## Acceptance For Next Card

```text
must consume ProgramJSON
must build structured result map
must expose root_body_id
must expose loop_body_id
must emit two bodies
must keep runtime_route_switch=0
```

Rows:

```text
local_loop_body_assignment_return
```

## Forbidden In Next Card

```text
full RecipeBodies runtime publication
RecipeBodies::bodies direct access
full RecipeMatcher execution
verifier policy reimplementation
recursive nested body arena claim
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_if_branch_arena_next_contract_selection_guard.sh
```

Expected result:

```text
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-LOOP-BODY-MULTI-BODY-ARENA-PARITY-001
loop_body_multi_body_arena_implemented=0
recipe_bodies_materialization=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-LOOP-BODY-MULTI-BODY-ARENA-PARITY-001
```
