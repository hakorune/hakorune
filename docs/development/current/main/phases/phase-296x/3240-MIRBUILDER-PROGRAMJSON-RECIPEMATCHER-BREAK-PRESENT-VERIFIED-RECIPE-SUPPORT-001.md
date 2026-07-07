# 3240 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-PRESENT-VERIFIED-RECIPE-SUPPORT-001

Status: landed

## Scope

Add the Break-present accepted-floor row after the Continue-present exact
BoxCount row:

```text
Loop.body = [
  If(cond, then=[Break], else=null),
  If(cond, then=[Return(Int)], else=null),
  Assignment(loop_var = loop_var + step)
]
```

This is one exact BoxCount slice. It does not claim a general or wider
loop-body sequence owner.

## Implementation

```text
LoopStmtHandler:
  reads the first loop-body If as an exit marker: Continue or Break
  builds Seq([If Exit(Break), If Exit(Return), Assignment]) for this row

ProgramJsonCanonicalLoopFactsInputSnapshotBox:
  continues to scan up to three loop-body statements
  computes exit_has_break from loop-body Break, not final top-level Return
```

## Expected Row

```text
row_id=local_loop_body_if_break_if_return_assignment
matched=1
contract_kind=LoopWithExit
has_break=1
has_continue=0
has_return=1
loop_cond_continue_with_return_present=0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_break_present_verified_recipe_support_gate.sh
```
