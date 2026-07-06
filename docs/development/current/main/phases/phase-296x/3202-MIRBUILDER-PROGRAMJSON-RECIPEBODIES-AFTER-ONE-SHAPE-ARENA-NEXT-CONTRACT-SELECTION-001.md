# 3202 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-AFTER-ONE-SHAPE-ARENA-NEXT-CONTRACT-SELECTION-001

Status: landed

## Decision

After the one-shape arena-builder retire-candidate, select:

```text
A_IF_BRANCH_MULTI_BODY_ARENA_PARITY
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-IF-BRANCH-MULTI-BODY-ARENA-PARITY-001
```

## Why

3200 proved a map-backed arena DTO with one root body. The next RecipeBodies
property to prove is not full runtime publication or verifier policy. It is
multiple body references:

```text
root body 0
  IfRef -> then body 1
          else body 2
```

This moves the `.hako` path toward the real `RecipeBlock { body_id, items }`
shape while staying below full RecipeMatcher, route selection, lowering,
mutation, ID allocation, and runtime route switching.

## Acceptance For Next Card

```text
must consume ProgramJSON
must build structured result map
must expose root_body_id
must expose if then/else body ids
must emit three bodies
must keep runtime_route_switch=0
```

Rows:

```text
local_if_then_local_else_print_return
```

## Forbidden In Next Card

```text
full RecipeBodies runtime publication
RecipeBodies::bodies direct access
full RecipeMatcher execution
verifier policy reimplementation
loop body arena claim
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_after_one_shape_arena_next_contract_selection_guard.sh
```

Expected result:

```text
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEBODIES-IF-BRANCH-MULTI-BODY-ARENA-PARITY-001
if_branch_multi_body_arena_implemented=0
recipe_bodies_materialization=0
runtime_route_switch=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-IF-BRANCH-MULTI-BODY-ARENA-PARITY-001
```
