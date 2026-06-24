---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Taskize the next recursive Recipe/CorePlan intake after the green staged loop/break canary.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1241-COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1243-COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-1253-COREPLAN-RECURSIVE-RECIPE-SHAPE-BACKLOG-AUDIT-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKIZATION-001

## Decision

Keep the reported `read_next_number_literal()` multi-stage loop with `break`
as a valid compiler acceptance target, but do not implement a new Recipe rule
from the already-green minimal canary.

The next compiler-side row is an intake row:

```text
COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
```

It must capture the next real shape that is not accepted today, reduce it to a
minimal fixture, and only then decide whether a recursive Recipe/CorePlan rule
is needed.

## Why This Is Not Immediate Implementation

The current staged loop/break canary is already green:

```text
canary=apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
planner_required_green=1
selected_rule=LoopSimpleWhile
flowbox_adopt_features=break
```

Therefore the current facts do not justify new compiler code.

The reported problem can still be real if the full `read_next_number_literal()`
shape contains a stronger structure than the captured canary:

```text
break inside deeper nested block
continue in the same staged loop family
loop-carried PHI exposed by scanner/parser state
multi-exit EOF/error/value scanner loop
return/break/continue interaction across nested blocks
```

That stronger shape must be captured as its own failing fixture before opening
Recipe/CorePlan implementation.

## Task Ladder

### 1. COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001

Purpose:

```text
Collect the next real compiler acceptance candidate from json_native or
selfhost code, then reduce it to a minimal .hako fixture.
```

Acceptance:

```text
source_shape_recorded=1
fixture_path=<path>
fixture_is_minimal=1
by_name_branch_count=0
json_native_route_changed=0
implementation_allowed=0
```

### 2. COREPLAN-RECURSIVE-RECIPE-FAILING-GAP-INVENTORY-001

Purpose:

```text
Run the reduced fixture under planner_required and record the first real
reject/freeze owner.
```

Acceptance:

```text
planner_required_fails=1
first_reject_owner=<owner>
missing_recipe_part=<part|none>
new_recipe_acceptance_required=<0|1>
implementation_allowed=0
```

If the fixture is green, close the row without implementation and pick another
candidate.

### 3. COREPLAN-RECURSIVE-RECIPE-MINIMAL-ACCEPTANCE-001

Open only if the failing-gap inventory proves a Recipe/CorePlan owner.

Purpose:

```text
Add the smallest recursive Recipe part needed for the failing fixture.
```

Non-goals:

```text
do not add a read_next_number_literal name branch
do not add an AST rewrite
do not implement every break/continue shape
do not change rust-subset source transport
```

## Other Currently Visible Unsupported Shapes

Keep these separated by owner:

```text
compiler_recipe_backlog:
  continue inside staged loop
  nested break/continue interactions
  loop-carried PHI scanner shapes
  return/break/continue interaction across nested blocks

rust_subset_app_front_backlog:
  else-if source spelling
  returnless void function body hardening
  Vec method calls such as push/len/get
  match / trait / generic Rust items as unsupported handoff until selected

json_native_hardening_backlog:
  tokenizer NUMBER payload regression promotion
  small number materializer retire
  critical JSON key bridge retire
```

## Recursive Recipe Rule

Use Recipe recursively when the source shape is semantic and local enough to
describe as parts:

```text
Facts:
  observe only
  no AST rewrite

Recipe:
  carry nested control-flow parts as data
  keep break/continue/return exits explicit
  carry PHI/loop-carried requirements as data

Lower:
  consume Recipe only
  do not re-scan source shape
```

Unknown sub-shapes fail fast:

```text
unknown_subshape -> freeze/reject
silent_fallback=0
```

## Stop Lines

```text
do not implement new Recipe/CorePlan code for a green fixture
do not restore read_next_number_literal WIP before a failing fixture exists
do not add scanner/parser method-name special cases
do not mix compiler acceptance with rust-subset adapter shape support
do not use token payload materializer stability as proof of compiler acceptance
```

## Contract

```text
output_contract=coreplan-recursive-recipe-real-shape-intake-taskization-v0

recursive_recipe_real_shape_intake_taskized=1
current_staged_loop_break_canary_green=1
implementation_allowed=0
next_compiler_task=COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
requires_minimal_failing_fixture=1
other_unsupported_shapes_bucketed=1
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005

summary=ok
```
