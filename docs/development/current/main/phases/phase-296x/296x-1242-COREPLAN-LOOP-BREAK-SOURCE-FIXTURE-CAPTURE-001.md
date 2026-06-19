---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Capture a minimal read_next_number_literal-style staged loop/break fixture without changing json_native parser routing.
Related:
  - apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
  - docs/development/current/main/phases/phase-296x/296x-1241-COREPLAN-LOOP-BREAK-RECIPE-BACKLOG-TASKIZATION-001.md
  - apps/rust-subset-to-hako/STATUS.md
---

# COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001

## Decision

Capture the staged scanner loop/break shape as a standalone compiler canary:

```text
apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
```

This fixture is intentionally not added to the default `phase29bq_fast_gate`
list yet. It is a source-shape capture for the compiler backlog, not a green
planner_required acceptance claim.

## Captured Shape

The fixture models the `read_next_number_literal()` family:

```text
loop(index < length)
  current char = text.substring(index, index + 1)
  staged local is_digit classification
  if not digit -> break
  loop-carried seen_digit / value / index
```

It avoids json_native parser restoration and by-name specialization. The source
shape is small enough for the next row to inspect planner_required behavior.

## Evidence

Default EXE/AOT route:

```bash
NYASH_FILEBOX_MODE=core-ro ./target/release/hakorune \
  --emit-exe /tmp/hako_read_next_number_literal_staged_loop_break_min \
  apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako

/tmp/hako_read_next_number_literal_staged_loop_break_min
```

Observed output:

```text
Result: 0
123
```

This proves the captured source is syntactically valid and executable on the
active app/selfhost validation route. It does not prove planner_required
Recipe/CorePlan acceptance because the local default release binary has
`vm-reference` disabled.

## Next Row

```text
COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001
```

Required work:

```text
run planner_required with an explicit vm-reference-capable binary
record first reject/freezer token
decide whether this is BoxCount or BoxShape
implementation_allowed=0
```

## Stop Lines

```text
do not add this canary to the default fast gate before planner_required evidence is recorded
do not restore json_native read_next_number_literal parser route in this row
do not use read_next_number_literal by-name branches
do not claim Recipe/CorePlan acceptance from EXE/AOT success alone
```

## Contract

```text
output_contract=coreplan-loop-break-source-fixture-capture-v0

fixture_exists=1
fixture_path=apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
json_native_route_changed=0
default_exe_aot_green=1
planner_required_claim=0
next_task=COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001

summary=ok
```
