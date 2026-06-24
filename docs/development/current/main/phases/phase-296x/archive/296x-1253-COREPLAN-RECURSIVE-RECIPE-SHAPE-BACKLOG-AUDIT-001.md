---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Keep recursive Recipe/CorePlan shape acceptance visible without mixing it into rust-subset app-front or json_native hardening rows.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1241-COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1242-COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001.md
  - docs/development/current/main/phases/phase-296x/296x-1243-COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001.md
  - apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
---

# COREPLAN-RECURSIVE-RECIPE-SHAPE-BACKLOG-AUDIT-001

## Decision

Keep recursive Recipe/CorePlan acceptance as the compiler-side direction for
future structured control-flow shapes, but do not implement a new rule until a
minimal failing fixture proves the owner.

The user request is valid:

```text
RECIPE should recursively analyze nested structured shapes when the shape is
semantic and local enough to describe as parts.
```

However, the captured `read_next_number_literal`-style staged loop/break canary
is already accepted:

```text
planner_required_green=1
selected_rule=LoopSimpleWhile
flowbox_adopt_features=break
```

Therefore:

```text
new_recipe_acceptance_required_now=0
implementation_allowed=0
next_compiler_acceptance_work_requires_new_minimal_failing_fixture=1
```

## What Happened With read_next_number_literal

The real json_native route was temporarily moved back to the token payload
stability path to keep the converter and bool-return validation green.

That does not mean the final compiler design should avoid the shape. It means:

```text
1. capture the exact failing compiler shape
2. prove it fails under planner_required
3. add the smallest recursive Recipe/CorePlan part
4. then restore the real json_native route
```

The currently captured canary stopped at step 2 because it is already green.

## Known Shape Backlog

### Compiler Acceptance

Open only with a minimal failing fixture:

```text
continue inside staged loop
nested loop break/continue interactions
multi-exit scanner loop if it differs from the captured canary
loop-carried PHI shape exposed by real scanner/parser bodies
return/break/continue interaction across nested blocks
```

### RustSubset App-Front Transport

These are source-shape tasks, not Recipe/CorePlan acceptance tasks:

```text
else-if source spelling
returnless void function body hardening
Vec method calls such as push/len/get
Rust match / trait / generic items remain unsupported handoff until selected
```

### json_native Hardening

These are app/library stability tasks, not generic compiler acceptance rows:

```text
returned token-array element route recovery
critical key materializer retire
small number materializer retire
FileBox/smoke serialization guard
```

## Recursive Recipe Rule

Use this rule when a real compiler gap appears:

```text
Facts:
  observe only, no AST rewrite

Recipe:
  encode nested parts recursively
  keep exit/break/continue edges explicit
  carry PHI/loop-carried needs as data

Lower:
  consume Recipe only
  do not re-scan source shape
```

Unknown or unsupported sub-shapes must fail fast:

```text
unknown_subshape -> freeze/reject
silent_fallback=0
```

## Next Compiler Task When Reopened

```text
COREPLAN-RECURSIVE-RECIPE-MINIMAL-FAILING-FIXTURE-SELECTION-001
```

Purpose:

```text
Find the next real compiler acceptance gap by selecting one minimal failing
fixture from the known shape backlog.
```

Do not open it while the current json_native route recovery and rust-subset
source-shape selection rows are active unless the user explicitly switches lane.

## Stop Lines

```text
do not add recursive Recipe/CorePlan code for a green fixture
do not treat app-front source transport as compiler acceptance
do not restore read_next_number_literal WIP without a dedicated failing fixture
do not add by-name branches for scanner/parser method names
do not mix this with JSON-NATIVE-TOKEN-ARRAY-ELEMENT-ORIGIN-SHADOW-001
```

## Contract

```text
output_contract=coreplan-recursive-recipe-shape-backlog-audit-v0

recursive_recipe_direction_preserved=1
read_next_number_literal_canary_green=1
new_recipe_acceptance_required_now=0
implementation_allowed=0
known_compiler_shape_backlog_recorded=1
next_compiler_task_when_reopened=COREPLAN-RECURSIVE-RECIPE-MINIMAL-FAILING-FIXTURE-SELECTION-001

summary=ok
```
