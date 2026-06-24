---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Taskize the read_next_number_literal staged loop/break acceptance backlog and keep it separate from rust-subset app-front shape work.
Related:
  - apps/rust-subset-to-hako/STATUS.md
  - docs/development/current/main/design/recipe-tree-and-parts-ssot.md
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/phases/phase-296x/296x-1237-RUST-SUBSET-NEXT-TASKS-STRUCTURE-AND-LOOP-BACKLOG-001.md
  - docs/development/current/main/phases/phase-296x/296x-1239-RUST-SUBSET-SYN-ADAPTER-WHILE-STATEMENT-001.md
---

# COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001

## Decision

Keep `read_next_number_literal()` loop/break acceptance as a compiler backlog,
not as a rust-subset converter feature.

The request is valid:

```text
read_next_number_literal-style staged scanner loop with conditional break
```

should eventually be accepted by Recipe/CorePlan. The current token-payload
route remains the stability route until compiler acceptance is green.

## Why This Is Not App-Front Work

The rust-subset app-front now accepts Rust `while` as a transport shape:

```text
Rust while -> RustSubset While -> .hako loop(cond)
```

That row explicitly does not accept:

```text
break
continue
multi-stage scanner loop exits
```

Those are compiler acceptance questions. Mixing them into the converter lane
would hide the real owner and risk reintroducing parser WIP before the backend
can lower it.

## Recipe Direction

The expected direction is recursive Recipe/CorePlan acceptance:

```text
Facts:
  observe staged loop structure and conditional exit tree

Recipe:
  carry the loop body, conditional break leaf, and continuation block as data

Lower:
  lower only from Recipe/CorePlan parts
  do not re-scan AST shape or special-case read_next_number_literal
```

This matches the current policy: Facts produce a recipe, Lower consumes the
recipe, and unsupported shapes fail fast.

## Task Ladder

### 1. COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001

Purpose:

```text
Capture a minimal .hako fixture that has the same staged loop/break shape as
read_next_number_literal(), without restoring json_native parser WIP.
```

Acceptance:

```text
fixture_exists=1
fixture_is_minimal=1
json_native_route_changed=0
default_exe_aot_result_may_be_green=1
planner_required_result_claim=0
```

### 2. COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001

Purpose:

```text
Run the minimal fixture through the current planner_required path and record
the exact reject/freezer token.
```

Output:

```text
first_reject_owner=<owner>
missing_recipe_part=<part>
lowering_owner=<parts|feature_pipeline|unknown>
implementation_allowed=0
```

### 3. COREPLAN-LOOP-BREAK-RECURSIVE-RECIPE-ACCEPTANCE-001

Purpose:

```text
Add the smallest recursive Recipe/CorePlan acceptance needed for the captured
staged loop with conditional break.
```

Non-goals:

```text
do not add a read_next_number_literal name branch
do not add an AST rewrite
do not add a converter workaround
do not generalize to every loop/break shape in the first row
```

### 4. COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001

Purpose:

```text
Only after the compiler fixture is green, restore/probe the json_native
read_next_number_literal route that was previously backed out.
```

Acceptance:

```text
compiler_fixture_green=1
json_native_parser_route_green=1
token_payload_stability_route_can_remain_or_retire_by_followup=1
```

## Other Known Not-Yet-Accepted Shapes

Keep these visible but do not open them in the loop/break row:

```text
compiler_acceptance:
  continue inside staged loop
  nested loop break/continue interactions not covered by the captured fixture
  loop-carried PHI shapes exposed by staged scanner bodies

rust_subset_app_front:
  else-if source spelling
  returnless void function body hardening
  Vec method calls such as push/len/get
```

## Stop Lines

```text
do not reintroduce the read_next_number_literal parser loop before compiler acceptance is green
do not treat token payload route as the final parser number design
do not implement break/continue in rust-subset converter as a workaround
do not add by-name branches for read_next_number_literal
do not mix this backlog with RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005
```

## Contract

```text
output_contract=coreplan-loop-break-recipe-backlog-taskization-v0

read_next_number_literal_backlog_taskized=1
compiler_acceptance_owner=Recipe/CorePlan
rust_subset_app_front_owner=0
recursive_recipe_direction_recorded=1
token_payload_stability_route_preserved=1
next_implementation_task=COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001
current_app_front_blocker_unchanged=RUST-SUBSET-SYN-ADAPTER-NEXT-SHAPE-SELECTION-005

summary=ok
```
