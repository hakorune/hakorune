# 3193 - MIRBUILDER-PROGRAMJSON-BLOCK-RECIPE-POST-EXIT-ALL-RECIPEBODIES-DESIGN-STOP-001

Status: landed

## Scope

Stop at the next Layer4 boundary after the covered flat block recipe if-mode
rows.

Covered flat rows:

```text
IfThenLocalNoElse
IfThenLocalElsePrint
IfThenReturnNoElse
IfThenLocalElseReturn
IfThenReturnElseLocal
IfThenReturnElseBreak
LoopExitAllowedBody
```

The next boundary is no longer another flat token projection. It is
`RecipeBodies` materialization: preserving `RecipeBlock` structure while
introducing `BodyId` / `StmtRef` references to an arena-like body store.

## Boundary

Source authority:

```text
docs/development/current/main/design/recipe-tree-and-parts-ssot.md
```

Fixed contract:

```text
RecipeBlock + RecipeItem = public structural truth
RecipeBodies / RecipeBody = internal arena storage
BodyId / StmtRef = reference boundary
```

Do not implement code in this card.

## Consultation

Recommended next consultation card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001
```

Question:

```text
Which minimal RecipeBodies surface should be exposed to .hako first:
DTO-only BodyId/StmtRef snapshot, one-shape arena builder parity, or a
different verifier-first boundary?
```

Recommended first slice if approved:

```text
ProgramJsonRecipeBodiesMinimalDtoV1
  one parseable ProgramJSON row
  emits RecipeBlock items plus BodyId/StmtRef references
  no lowering
  no full RecipeMatcher
  no runtime route switch
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_block_recipe_post_exit_all_recipebodies_design_stop_guard.sh
```

Expected guard result:

```text
boundary=RecipeBodiesDesignStop
implementation_allowed_now=0
recipe_bodies_materialization=0
full_recipe_matcher_execution=0
route_selection=0
source_selfhost_claim=0
```

## Non-Claims

```text
RecipeBodies materialization
full RecipeMatcher execution
runtime route switch
route selection
MIR lowering
MIR mutation
ID allocation
ProgramJSON full parser
Source Selfhost
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-MINIMAL-BASIS-CONSULTATION-001
```
