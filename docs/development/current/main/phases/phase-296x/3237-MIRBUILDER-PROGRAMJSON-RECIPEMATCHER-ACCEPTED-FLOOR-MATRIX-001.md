# 3237 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-ACCEPTED-FLOOR-MATRIX-001

Status: landed

## Scope

Record the accepted-floor matrix required before any ProgramJSON RecipeMatcher
authority switch can be considered.

This card does not add a new runtime authority path. It fixes the current
matrix state and selects the first implementation capability needed to make the
floor larger than the existing return-only rows.

## Current Green Rows

```text
local_loop_body_if_branch_return
local_loop_body_if_branch_return_alt_names
local_loop_body_if_branch_return_i_wide_step
local_loop_body_if_branch_return_count_wide_step
```

These rows are accepted LoopWithExit shadow rows:

```text
matched=1
contract_kind=LoopWithExit
has_break=0
has_continue=0
has_return=1
runtime_authority=rust_astnode
```

## Blocked Accepted Axes

```text
continue_present:
  blocked_by=verified_recipe_missing
  owner_gap=LoopStmtHandler does not build Exit(Continue) verified recipe items.

break_present:
  blocked_by=verified_recipe_missing
  owner_gap=LoopStmtHandler does not build Exit(Break) verified recipe items.

break_and_continue_present:
  blocked_by=continue_present + break_present
  owner_gap=requires both Exit(Continue) and Exit(Break) verified recipe rows.

return_absent_decision_row:
  blocked_by=semantic_decision_required
  owner_gap=CanonicalLoopFacts snapshot currently requires final Return and
  hardcodes accepted LoopWithExit return presence.

nested_loop_decision_row:
  blocked_by=semantic_decision_required
  owner_gap=Nested Loop is observable but rejected before RecipeMatcher input.
```

## Selected Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001
```

The next implementation slice must be one BoxCount row:

```text
ProgramJSON Loop body contains Continue and still has a Return exit.
The `.hako` verified-recipe path must produce a read-only matcher snapshot with
has_continue=1 and has_return=1.
```

Do not add Break or nested-loop support in the same slice.

## Non-Claims

```text
ProgramJSON does not write PlanBuildOutcome.recipe_contract.
ProgramJSON does not feed route registry predicates.
ProgramJSON does not select routes.
ProgramJSON does not lower or mutate MIR.
ProgramJSON does not allocate IDs.
runtime_route_switch = 0
programjson_runtime_route_authority = 0
recipe_matcher_input_authority = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_accepted_floor_matrix_guard.sh
```

Expected result:

```text
accepted_floor_matrix=1
current_green_return_only_rows=4
continue_present_status=blocked_verified_recipe_missing
break_present_status=blocked_verified_recipe_missing
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001
programjson_runtime_route_authority=0
runtime_route_switch=0
source_selfhost_claim=0
```
