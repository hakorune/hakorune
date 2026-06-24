---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Refresh the compiler Recipe/CorePlan real-shape taskboard after the continue fixture capture.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1276-COREPLAN-RECURSIVE-RECIPE-REAL-SHAPE-INTAKE-TASKBOARD-001.md
  - docs/development/current/main/phases/phase-296x/296x-1281-COREPLAN-CONTINUE-IN-STAGED-LOOP-FIXTURE-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# COREPLAN-REAL-SHAPE-TASKBOARD-REFRESH-003

## Decision

Keep recursive Recipe/CorePlan as the compiler-side direction, but continue to
open one concrete acceptance shape per row.

The reported `read_next_number_literal()` shape is not rejected. It remains the
target family:

```text
multi-stage scanner loop
break on terminator
continue on separator
loop-carried cursor / count / seen state
post-loop validity branch
possible nested break/continue interactions
```

However, the current minimal failing evidence is more specific:

```text
fixture=apps/tests/phase29bq_selfhost_blocker_read_number_continue_staged_min.hako
failure=mir/verify:dominator_violation
owner=loop_cond_break_continue partial carrier PHI handling
```

Therefore the immediate task is not a broad recursive Recipe rewrite. It is the
partial-carrier continue PHI fix.

## Current Queue

### Active

```text
COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
```

Goal:

```text
Accept a staged scanner loop where one continue branch updates only part of the
loop-carried state and preserves the remaining carriers without producing a
non-dominating PHI input.
```

Acceptance:

```text
gate_case=selfhost_read_number_continue_staged_min
expected_output=3
planner_required_green=1
method_name_branch=0
json_native_route_changed=0
single_acceptance_shape_added=1
```

### Queued After Active

```text
COREPLAN-READ-NEXT-NUMBER-LITERAL-MULTI-STAGE-LOOP-ACCEPTANCE-001
```

Goal:

```text
Return to the full read_next_number_literal-style staged loop only after the
partial-carrier continue fixture is green.
```

Scope:

```text
break-bearing multi-stage loop
separator continue
post-loop seen/value validation
EOF/error/value exit classification
```

This task should start with another minimal fixture or by promoting the current
captured fixtures into a combined scanner-shape fixture. It must not implement
`read_next_number_literal` by method name.

### Later Compiler Shapes

```text
COREPLAN-NESTED-BREAK-CONTINUE-001
COREPLAN-LOOP-CARRIED-PHI-SCANNER-SHAPE-001
COREPLAN-RETURN-BREAK-CONTINUE-INTERACTION-001
```

Each row needs a minimal failing fixture before implementation.

## Already Closed App-Front Shapes

These are not blockers for the current compiler Recipe/CorePlan lane:

```text
Rust while transport -> closed
Rust vec![...] literal transport -> closed
Rust Vec method call transport -> closed
Rust loop without break/continue transport -> closed
```

If Rust `break` / `continue` statement transport becomes necessary, it should be
opened as a RustSubset app-front row. It is separate from compiler acceptance of
`.hako` break/continue lowering.

## Unsupported App-Front Handoff Queue

These remain app-front or source-transport tasks, not compiler Recipe tasks:

```text
match semantics
for-loop semantics
trait/generic item support
break/continue RustSubset transport if a real adapter input requires it
```

## Stop Lines

```text
do not implement read_next_number_literal by name
do not reopen broad recursive Recipe rewriting while a narrower failing fixture exists
do not mix RustSubset source transport with compiler Recipe acceptance
do not change json_native app code to avoid compiler acceptance work
do not use token payload route recovery as compiler Recipe evidence
do not treat already closed while/Vec transport rows as active blockers
```

## Report

```text
output_contract=coreplan-real-shape-taskboard-refresh-v3
active_task=COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001
read_next_number_literal_family_kept=1
recursive_recipe_direction_kept=1
broad_recipe_rewrite_allowed=0
while_transport_closed=1
vec_literal_transport_closed=1
implementation_allowed_for_active_fixture_only=1
summary=ok
```
