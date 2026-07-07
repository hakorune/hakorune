# 3241 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-BREAK-CONTINUE-PRESENT-VERIFIED-RECIPE-SUPPORT-001

Status: landed

## Scope

Add the combined Break+Continue accepted-floor row:

```text
Loop.body = [
  If(cond, then=[Break], else=null),
  If(cond, then=[Continue], else=null),
  If(cond, then=[Return(Int)], else=null),
  Assignment(loop_var = loop_var + step)
]
```

This is one exact BoxCount slice. It does not claim a general or wider
loop-body sequence owner.

## Implementation

```text
LoopStmtHandler:
  body_kind = if_break_if_continue_if_return_assignment
  verified recipe body = Seq([If Exit(Break), If Exit(Continue), If Exit(Return), Assignment])

ProgramJsonCanonicalLoopFactsInputSnapshotBox:
  reads the fourth statement as the update when present
  scans up to four loop-body statements for Break / Continue / Return / Loop
```

## Expected Row

```text
row_id=local_loop_body_if_break_if_continue_if_return_assignment
matched=1
contract_kind=LoopWithExit
has_break=1
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_break_continue_present_verified_recipe_support_gate.sh
```
