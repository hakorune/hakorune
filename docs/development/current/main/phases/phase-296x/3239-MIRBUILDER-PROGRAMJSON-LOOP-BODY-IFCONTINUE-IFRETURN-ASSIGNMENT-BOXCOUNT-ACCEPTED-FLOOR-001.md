# 3239 - MIRBUILDER-PROGRAMJSON-LOOP-BODY-IFCONTINUE-IFRETURN-ASSIGNMENT-BOXCOUNT-ACCEPTED-FLOOR-001

Status: landed

## Scope

Add the exact ProgramJSON RecipeMatcher accepted-floor row selected after the
Continue-present row-shape design stop:

```text
Loop.body = [
  If(cond, then=[Continue], else=null),
  If(cond, then=[Return(Int)], else=null),
  Assignment(loop_var = loop_var + step)
]
```

This is a single BoxCount slice. It does not claim a general loop-body sequence
owner.

## Implementation

```text
LoopStmtHandler:
  body_kind = if_continue_if_return_assignment
  verified recipe body = Seq([If Exit(Continue), If Exit(Return), Assignment])

ProgramJsonCanonicalLoopFactsInputSnapshotBox:
  reads the third statement as the update when present
  scans up to three loop-body statements for Continue/Return/Break/Loop
  computes exit_has_return from loop-body Return, not final top-level Return
```

## Expected Row

```text
row_id=local_loop_body_if_continue_if_return_assignment
matched=1
contract_kind=LoopWithExit
has_break=0
has_continue=1
has_return=1
loop_cond_continue_with_return_present=1
loop_cond_return_in_body_present=0
```

## Non-Claims

```text
general_loop_body_sequence_owner = 0
wider_loop_body_sequence_owner = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_loop_body_ifcontinue_ifreturn_assignment_boxcount_accepted_floor_gate.sh
```
