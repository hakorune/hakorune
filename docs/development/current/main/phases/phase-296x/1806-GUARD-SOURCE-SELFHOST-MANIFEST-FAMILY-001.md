# 1806 - GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001

## Token

```text
GUARD-SOURCE-SELFHOST-MANIFEST-FAMILY-001
```

## Purpose

Add a reusable Source Selfhost family guard manifest so new Source Selfhost
rows do not need to duplicate card / fixture / non-claim / current blocker
checks.

Historical row guards remain callable for traceability. New current work should
prefer the family guard unless a row executes nontrivial code, build, or perf
validation.

## Output

```text
manifest:
  docs/development/current/main/design/fixtures/rust-lifecycle/
    source-selfhost-family-guard-manifest-v0.json

guard:
  tools/checks/rust_lifecycle_source_selfhost_family_guard.sh
```

## Acceptance

```text
family_guard_manifest = green
current_state_pointer_guard = green
current_blocker_token =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

historical_row_guards_callable = 1
manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
runner_semantic_owner = 0
```

## Non-Claims

```text
no row guard deletion
no route repair
no family adoption decision
no wider route selection
no Source Selfhost claim
```
