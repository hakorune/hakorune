# 3233 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-RUNTIME-ROUTE-SHADOW-SWITCH-CONSULTATION-002

Status: active

Decision: SELECT_B_RUNTIME_ROUTE_ADJACENT_SHADOW_GUARD

## Scope

Record the post-3232 consultation decision for the runtime route shadow-switch
boundary.

3231 proved a dual-run shadow guard. 3232 expanded RecipeMatcher shadow parity
to four `LoopWithExit` rows. The next task may move the comparison adjacent to
runtime route selection, but must not switch runtime authority.

## Selected Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001
```

The selected next task is a runtime-adjacent shadow guard:

```text
after try_build_outcome(ctx)
before registry candidate selection
```

Rust ASTNode remains authority. ProgramJSON remains shadow-only.

## Rejected / Deferred

```text
C_LIMITED_PROGRAMJSON_AUTHORITY_SWITCH
  rejected for now

D_MORE_COVERAGE_BEFORE_ANY_RUNTIME_ADJACENT_WORK
  deferred as the coverage floor before any later authority-switch card

A_SHADOW_ONLY_CONTINUE
  not selected as primary because it risks returning to coverage-only progress
```

## Forbidden

```text
write ProgramJSON result into PlanBuildOutcome.recipe_contract
pass ProgramJSON result to route registry or predicates
compose CorePlan from ProgramJSON result
lower MIR from ProgramJSON result
mutate MIR from ProgramJSON result
allocate IDs from ProgramJSON result
fallback to Rust on ProgramJSON mismatch
claim ProgramJSON runtime route authority
claim runtime route switch
claim Source Selfhost
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_runtime_route_shadow_switch_consultation_002_guard.sh
```

Expected result:

```text
consultation_decision_recorded=1
selected_b_runtime_route_adjacent_shadow_guard=1
selected_next_card=MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001
runtime_authority=rust_astnode
programjson_runtime_route_authority=0
runtime_route_switch=0
recipe_matcher_input_authority=0
limited_programjson_authority_switch=0
route_selection=0
mir_lowering=0
mir_mutation=0
id_allocation=0
runtime_fallback=0
source_selfhost_claim=0
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RUNTIME-ROUTE-ADJACENT-SHADOW-GUARD-001
```
