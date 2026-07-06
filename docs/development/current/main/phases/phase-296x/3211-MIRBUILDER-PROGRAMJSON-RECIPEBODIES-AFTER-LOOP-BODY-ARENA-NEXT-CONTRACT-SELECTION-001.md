# 3211 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-LOOP-BODY-ARENA-NEXT-CONTRACT-SELECTION-001

Status: landed

## Decision

After the Loop-body multi-body arena retire-candidate and the 3208-3210 AOT
contract cleanup, select:

```text
A_RECURSIVE_NESTED_BODY_ARENA_PARITY
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RECURSIVE-NESTED-BODY-ARENA-PARITY-001
```

## Why

3203/3204 proved branch body references:

```text
IfRef -> then body 1
         else body 2
```

3206/3207 proved loop body references:

```text
LoopRef -> body 1
```

The next RecipeBodies property is composing these reference forms in one nested
arena DTO without claiming runtime `RecipeBodies`, full RecipeMatcher
execution, route selection, lowering, mutation, ID allocation, or runtime route
switching.

## Acceptance For Next Card

```text
must consume ProgramJSON
must build structured result map
must expose root_body_id
must expose nested body references
must emit at least four bodies
must keep runtime_route_switch=0
```

Rows:

```text
local_loop_body_if_branch_return
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_loop_body_arena_next_contract_selection_guard.sh
```

Expected result:

```text
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RECURSIVE-NESTED-BODY-ARENA-PARITY-001
recursive_nested_body_arena_implemented=0
recipe_bodies_materialization=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RECURSIVE-NESTED-BODY-ARENA-PARITY-001
```
