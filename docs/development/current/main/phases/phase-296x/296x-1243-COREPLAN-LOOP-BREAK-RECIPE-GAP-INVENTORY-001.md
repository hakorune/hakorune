---
Status: Done
Decision: accepted
Date: 2026-06-19
Scope: Inventory planner_required behavior for the read_next_number_literal staged loop/break canary.
Related:
  - apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
  - docs/development/current/main/phases/phase-296x/296x-1242-COREPLAN-LOOP-BREAK-SOURCE-FIXTURE-CAPTURE-001.md
---

# COREPLAN-LOOP-BREAK-RECIPE-GAP-INVENTORY-001

## Decision

The captured `read_next_number_literal` staged loop/break canary does not expose
a new Recipe/CorePlan gap.

It is accepted by existing planner-required infrastructure:

```text
[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile
[flowbox/adopt box_kind=Loop features=break via=shadow]
```

Therefore:

```text
new_recipe_acceptance_required=0
recursive_recipe_implementation_allowed=0
```

## Evidence

Command:

```bash
source tools/smokes/v2/lib/test_runner.sh
source tools/smokes/v2/lib/joinir_planner_first_gate.sh
source tools/smokes/v2/lib/vm_route_pin.sh
require_env
export_vm_route_pin
run_planner_first_gate \
  coreplan_loop_break_gap_inventory \
  apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako \
  123 \
  '[joinir/planner_first rule=LoopSimpleWhile] label=LoopSimpleWhile|[flowbox/adopt box_kind=Loop features=break via=shadow]' \
  0 \
  20
```

Result:

```text
PASS
```

The fixture is now registered in:

```text
tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_cases.tsv
```

Case id:

```text
selfhost_read_next_number_literal_staged_loop_break_min
```

## Interpretation

The original blocker was not reproduced by the minimal staged loop/break shape.
The previous json_native WIP likely had an additional owner beyond this minimal
shape, such as:

```text
json_native-specific scanner/token payload interaction
loop-carried PHI shape not present in the canary
parser route restoration detail
```

Do not implement a new Recipe/CorePlan rule from this canary alone.

## Next Row

Proceed to:

```text
COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001
```

Scope:

```text
restore/probe the json_native read_next_number_literal route behind a narrow row
if it fails, capture the new minimal failing shape before implementation
```

## Stop Lines

```text
do not add recursive Recipe/CorePlan code for a green canary
do not claim every break/continue scanner shape is accepted
do not remove token payload stability route in this row
do not restore json_native parser route without a dedicated probe row
```

## Contract

```text
output_contract=coreplan-loop-break-recipe-gap-inventory-v0

canary_fixture=apps/tests/phase29bq_selfhost_blocker_read_next_number_literal_staged_loop_break_min.hako
planner_required_green=1
selected_rule=LoopSimpleWhile
flowbox_adopt_features=break
new_recipe_acceptance_required=0
fast_gate_case_registered=1
next_task=COREPLAN-LOOP-BREAK-JSON-NATIVE-RESTORE-PROBE-001

summary=ok
```
