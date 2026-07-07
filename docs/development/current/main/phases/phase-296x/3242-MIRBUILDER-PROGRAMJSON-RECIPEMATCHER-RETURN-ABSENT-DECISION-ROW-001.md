# 3242 - MIRBUILDER-PROGRAMJSON-RECIPEMATCHER-RETURN-ABSENT-DECISION-ROW-001

Status: landed

## Scope

Stop before implementing a Return-absent accepted-floor row.

Rust `RecipeMatcher::try_match_loop` can describe:

```text
RecipeContractKind::LoopWithExit { has_break, has_continue, has_return=false }
```

But this is not just another ProgramJSON BoxCount row. The runtime route-entry
release boundary also has a special condition for:

```text
facts.exit_usage.has_break && facts.exit_usage.has_continue && !facts.exit_usage.has_return
```

The current ProgramJSON CanonicalLoopFacts snapshot also requires a final
top-level `Return`. That final return is outside the loop body and must not be
used as evidence for loop-body `has_return`.

## Decision Boundary

```text
unsafe shortcut:
  Treat final top-level Return absence/presence as equivalent to loop-body
  has_return=false.

why unsafe:
  has_return is loop-body exit usage.
  return_absent intersects runtime route release gating.
  accepting it now would mix shadow RecipeMatcher evidence with route authority
  semantics.
```

## Candidate Decisions

```text
A. Accept return_absent accepted-floor now
   state=RejectedForNow
   reason=would open a route-release-sensitive shape without runtime authority
   decision

B. Defer return_absent to route-release consultation
   state=RecommendedDefault
   reason=keeps accepted floor shadow-only and preserves the no-switch boundary

C. Add scan-only return_absent diagnostic
   state=ConsultationAlternative
   reason=diagnostic evidence may be useful but is not accepted-floor proof
```

## Non-Claims

```text
return_absent_green = 0
return_absent_accepted_floor = 0
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
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipematcher_return_absent_decision_row_guard.sh
```
