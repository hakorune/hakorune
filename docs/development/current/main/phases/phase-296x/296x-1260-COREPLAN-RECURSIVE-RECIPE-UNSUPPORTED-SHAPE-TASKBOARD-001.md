---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Refresh the recursive Recipe/CorePlan unsupported-shape taskboard without opening implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1241-COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001.md
  - docs/development/current/main/phases/phase-296x/296x-1243-COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-1253-COREPLAN-RECURSIVE-RECIPE-SHAPE-BACKLOG-AUDIT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1256-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKIZATION-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# COREPLAN-RECURSIVE-RECIPE-UNSUPPORTED-SHAPE-TASKBOARD-001

## Decision

Keep recursive Recipe/CorePlan as the compiler-side direction for real control
flow acceptance, but do not implement new Recipe code from the currently
available evidence.

The reported `read_next_number_literal()` multi-stage loop with `break` remains
the leading real-shape candidate. It is not reopened as WIP until a minimal
failing fixture proves a compiler owner.

## Current Facts

The existing minimal canary is accepted:

```text
fixture=apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
default_exe_aot_green=1
planner_required_green=1
selected_route=LoopSimpleWhile + flowbox/adopt break
```

Therefore:

```text
new_recipe_implementation_allowed=0
read_next_number_literal_by_name_branch_allowed=0
json_native_route_restore_from_this_card=0
```

The previous `read_next_number_literal()` WIP was correctly backed out to keep
JSON number payload validation stable. That rollback does not reject recursive
Recipe; it only says the current evidence is not yet a compiler failing shape.

## Why Recursive Recipe Is Still The Right Shape

When the compiler sees nested control flow, staged local values, and explicit
exits, the clean representation is still data-first:

```text
Facts:
  observe nested parts only
  no AST rewrite

Recipe:
  carry nested loop/if parts recursively
  keep break/continue/return exits explicit
  carry loop-carried/PHI requirements as data

Lower:
  consume Recipe only
  do not re-scan source shape
```

This is the natural fix if a new failing fixture proves that the current
LoopSimpleWhile/flowbox route is not enough.

## Unsupported Shape Queue

### Compiler Recipe/CorePlan Queue

Open these only after a minimal failing fixture exists:

```text
1. read_next_number_literal full multi-stage scanner loop
2. continue inside staged loop
3. nested break/continue interactions
4. loop-carried PHI scanner shape
5. return/break/continue interaction across nested blocks
6. multi-exit EOF/error/value scanner loop
```

### RustSubset App-Front Queue

These are source transport/converter features, not compiler Recipe work:

```text
1. else-if source spelling
2. returnless void function body hardening
3. Vec method calls such as push/len/get
4. match handoff as Unsupported until selected
5. trait/generic Rust items as Unsupported until selected
```

### json_native Hardening Queue

These are app/runtime library stability tasks, not new compiler acceptance:

```text
1. JsonNode numeric value conversion owner probe
2. number materializer retire after numeric value conversion is green
3. critical JSON key bridge retire after scanner-derived critical keys are stable
```

## Task Order

### 1. COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001

Purpose:

```text
Capture the next real compiler acceptance candidate from json_native or
selfhost code and reduce it to a minimal .hako fixture.
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
planner_required_fails=<0|1>
first_reject_owner=<owner|none>
missing_recipe_part=<part|none>
new_recipe_acceptance_required=<0|1>
implementation_allowed=0
```

If the fixture is green, close the row and pick another candidate.

### 3. COREPLAN-RECURSIVE-RECIPE-MINIMAL-ACCEPTANCE-001

Open only if the inventory proves a compiler owner.

Purpose:

```text
Add the smallest recursive Recipe part needed for the failing fixture.
```

Acceptance:

```text
single_acceptance_shape_added=1
fixture_green=1
planner_required_green=1
observability_updated=1
```

## Stop Lines

```text
do not implement new Recipe/CorePlan code for a green fixture
do not reintroduce read_next_number_literal WIP before a failing fixture exists
do not special-case read_next_number_literal by method name
do not mix RustSubset source-shape support with compiler Recipe acceptance
do not use JSON token payload stability as proof of compiler acceptance
do not use .hako app workaround to avoid compiler acceptance work
```

## Contract

```text
output_contract=coreplan-recursive-recipe-unsupported-shape-taskboard-v0

recursive_recipe_direction_kept=1
current_staged_loop_break_canary_green=1
implementation_allowed=0
requires_minimal_failing_fixture=1
compiler_recipe_queue_refreshed=1
rust_subset_app_front_queue_separated=1
json_native_hardening_queue_separated=1
next_compiler_task=COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-001
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005

summary=ok
```
