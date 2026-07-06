# 3219 - MIRBUILDER-PROGRAMJSON-RECIPEBODIES-POST-VERIFIER-BOUNDARY-DECISION-001

Status: active

## Decision

Adopt 3218 recommended option:

```text
A_MORE_DTO_COVERAGE_ROWS
```

Next card:

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001
```

## Why

This keeps the next step inside the already proven ProgramJSON DTO and
RecipeVerifier boundary. It does not open runtime `RecipeBodies` publication,
full RecipeMatcher execution, route selection, lowering, mutation, ID
allocation, or runtime route switching.

## Acceptance For Next Card

```text
must reuse ProgramJsonRecipeBodiesVerifierBoundarySnapshotBox
must add more parity rows
must keep runtime_recipe_bodies_publication=0
must keep full_recipe_matcher_execution=0
must keep runtime_route_switch=0
```

## Still Requires New Decision

```text
B_RUNTIME_RECIPEBODIES_PUBLICATION_BRIDGE
C_FULL_RECIPEMATCHER_EXECUTION
```

## Evidence

```bash
bash tools/checks/rust_lifecycle_mirbuilder_programjson_recipebodies_post_verifier_boundary_decision_guard.sh
```

## Next

```text
MIRBUILDER-PROGRAMJSON-RECIPEBODIES-VERIFIER-BOUNDARY-EXPANDED-DTO-COVERAGE-PARITY-001
```
