# 1804 - MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-GUARD-REALIGNMENT-001

## Token

```text
MIRBUILDER-MINIMAL-PATH-MAINLINE-READINESS-GUARD-REALIGNMENT-001
```

## Purpose

Realign the minimal-path mainline readiness resolver guard with the current
Source Selfhost maintenance phase.

The previous guard required the task-order prose phrase:

```text
same-state composed prefix evidence
```

That phrase was stale task-order vocabulary, not semantic evidence. This card
replaces that prose dependency with structured task-order evidence fields.

## Boundary

```text
does:
  update readiness resolver task-order needles
  regenerate the readiness resolution fixture
  keep the Source Selfhost design stop active

does not:
  select a new route family
  open route repair
  adopt a family
  claim Source Selfhost
```

## Structured Evidence Required

```text
mainline_readiness = Ready
mainline_readiness_decision = ReadyForMinimalPathMainlinePilot
mainline_next_unconsumed_edge = Closed
mainline_generated_hako_executable_closure = Closed
mainline_same_state_handoff_observed = 1
```

## Acceptance

```text
rust_lifecycle_mirbuilder_minimal_path_mainline_readiness_resolver_guard = green
current_blocker_token =
  SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001

manual_next_owner_selection = 0
manual_family_selection = 0
source_selfhost_claim = 0
runtime_fallback = 0
new_backend_route = 0
new_abi = 0
new_python_semantic_projector = 0
```

## Non-Claims

```text
no new semantic projection
no route repair
no family adoption decision
no wider route selection
no Source Selfhost claim
```
