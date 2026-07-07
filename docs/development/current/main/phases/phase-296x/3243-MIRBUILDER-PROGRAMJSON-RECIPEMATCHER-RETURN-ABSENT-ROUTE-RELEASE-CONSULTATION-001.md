# 3243 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ROUTE-RELEASE-CONSULTATION-001

Status: landed

## Scope

Record the consultation result for Return-absent ProgramJSON RecipeMatcher work.

The selected decision is:

```text
B_DEFER_RETURN_ABSENT_TO_ROUTE_RELEASE_CONSULTATION
```

Return-absent is not a normal accepted-floor row because it overlaps the
runtime route-entry release condition:

```text
facts.exit_usage.has_break && facts.exit_usage.has_continue && !facts.exit_usage.has_return
```

`has_return=false` remains loop-body exit usage only. A final top-level
`Return` is not evidence for loop-body `has_return`.

## Selected Sequence

```text
1. MIRBUILDER-PROGRAMJSON-LOOP-BODY-RETURN-ABSENT-SCAN-ONLY-DIAGNOSTIC-001
   diagnostic-only proof that final top-level Return does not set loop-body
   has_return.

2. MIRBUILDER-PROGRAMJSON-CANONICAL-LOOP-FACTS-FINAL-TOPLEVEL-RETURN-DECOUPLE-SNAPSHOT-BOUNDARY-001
   decouple final top-level Return presence from loop-body exit usage.

3. MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-ACCEPTED-FLOOR-001
   only after route-release semantics are stable.
```

## Non-Claims

```text
return_absent_green = 0
return_absent_accepted_floor = 0
matcher_result_equal = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_return_absent_route_release_consultation_guard.sh
```
