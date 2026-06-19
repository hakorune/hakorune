---
Status: Active
Decision: accepted
Date: 2026-06-19
Scope: Define and wire the B-lite loop resolver as a read-only shadow seam
  over existing loop route facts.
Related:
  - docs/development/current/main/phases/phase-296x/296x-1284-COREPLAN-LOOP-RESOLVER-REAGGREGATION-TASKBOARD-001.md
  - docs/development/current/main/phases/phase-296x/296x-1285-COREPLAN-LOOP-ROUTE-DEBT-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-1286-COREPLAN-CONTINUE-PARTIAL-CARRIER-PHI-001.md
---

# COREPLAN-LOOP-RESOLVER-B-LITE-SSOT

## Decision

Add a small B-lite loop legacy observer seam beside the existing named route
registry. It is not a new lowering route and it is not yet an independent
semantic resolver. It is a read-only observer that turns existing legacy
registry candidate facts into:

```text
Allow(LoopRouteFact)
or
Deny(LoopRouteDenyReason)
```

The observer exists to make route ownership debt visible before retiring named
routes. It must not become another route priority layer or be promoted as-is to
route selection.

## Contract

```text
facts_freeze_before_resolver=1
observer_reads_frozen_facts_and_legacy_registry=1
resolver_mutates_facts=0
resolver_returns_allow_or_deny=1
bool_only_predicate=0
reachability_feedback_to_resolver=0
unknown_or_overlap_in_strict=deny_or_freeze
backend_lowering_changed=0
```

## Implementation

Code seam:

```text
src/mir/builder/control_flow/joinir/route_entry/registry/legacy_observer.rs
```

Implemented vocabulary:

```text
LoopRouteFact
LoopRouteDecision
LoopRouteDenyReason
LoopRouteShadowReport
```

The resolver compares:

```text
legacy_matched_candidates:
  every registry entry whose predicate matches before registry-level suppression

legacy_effective_candidates:
  current registry candidates after existing suppression/priority filters

legacy_suppressed_candidates:
  legacy_matched minus legacy_effective
```

Decision policy:

```text
no facts:
  Deny(NoFacts)

zero effective candidates:
  Deny(NoCandidate)

one effective candidate:
  Allow(LoopRouteFact { selected_route })

more than one effective candidate:
  Deny(OverlappingNamedRoutes)
```

## Deny Reason Owner Map

```text
Deny(NoFacts):
  owner=recipe_fact_producer

Deny(NoCandidate):
  owner=fixture_inventory

Deny(OverlappingNamedRoutes):
  owner=loop_route_retire_selection
```

## Shadow Trace

When JoinIR debug logging is enabled under strict/planner-required candidate
checking, the router emits one stable line:

```text
[plan/trace:loop_legacy_observer]
```

The current line reports:

```text
decision=<allow|deny>
legacy_matched=<routes|none>
legacy_effective=<routes|none>
legacy_suppressed=<routes|none>
```

This trace is diagnostic only. The existing ordered registry still selects the
lowering route. It does not prove independent resolver parity because it still
observes legacy registry candidates.

## Acceptance

Verified:

```bash
cargo check --bin hakorune
```

Result:

```text
Finished dev profile target(s)
```

## Stop Lines

```text
do not change route selection
do not add a new named loop route
do not use resolver output for lowering yet
do not feed route reachability back into resolver
do not replace existing named-route predicates in this row
```

## Report

```text
output_contract=coreplan-loop-resolver-b-lite-ssot-v0
implementation_changed=1
behavior_changed=0
observer_reads_frozen_facts_and_legacy_registry=1
resolver_returns_allow_or_deny=1
legacy_matched_candidates_reported=1
legacy_effective_candidates_reported=1
legacy_suppressed_candidates_reported=1
reachability_feedback_to_resolver=0
route_selection_changed=0
summary=ok
```
