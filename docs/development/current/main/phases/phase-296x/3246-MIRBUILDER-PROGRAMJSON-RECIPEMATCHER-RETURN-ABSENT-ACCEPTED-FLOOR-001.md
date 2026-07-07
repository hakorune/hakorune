# 3246 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001

Status: landed

## Scope

Add the scoped Return-absent accepted-floor row for ProgramJSON RecipeMatcher
shadow parity.

Covered exact shape:

```text
Loop.body = [
  If(then=[Break], else=null),
  If(then=[Continue], else=null),
  Assignment(AddVarInt)
]
final top-level Return = present
```

Expected matcher result:

```text
matched = 1
contract_kind = LoopWithExit
has_break = 1
has_continue = 1
has_return = 0
```

## Boundary

This is still shadow-only. ProgramJSON does not write
`PlanBuildOutcome.recipe_contract`, does not feed route predicates, and does not
select/release routes.

## Non-Claims

```text
programjson_runtime_route_authority = 0
runtime_route_switch = 0
recipe_matcher_input_authority = 0
route_release_authority = 0
route_selection = 0
MIR lowering/mutation = 0
ID allocation = 0
Source Selfhost remains unclaimed.
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_return_absent_accepted_floor_gate.sh
```
