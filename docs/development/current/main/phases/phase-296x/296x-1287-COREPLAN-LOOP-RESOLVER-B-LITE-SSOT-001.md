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

Add a small B-lite loop resolver seam beside the existing named route
registry. It is not a new lowering route. It is a read-only observer that
turns existing route facts into:

```text
Allow(LoopRouteFact)
or
Deny(LoopRouteDenyReason)
```

The resolver exists to make route ownership debt visible before retiring named
routes. It must not become another route priority layer.

## Contract

```text
facts_freeze_before_resolver=1
resolver_reads_facts_only=1
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
src/mir/builder/control_flow/joinir/route_entry/registry/resolver.rs
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
raw_candidates:
  every registry entry whose predicate matches before suppression

effective_candidates:
  current registry candidates after existing suppression/priority filters

suppressed_candidates:
  raw minus effective
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
[plan/trace:loop_resolver_b_lite]
```

The line reports:

```text
decision=<allow|deny>
raw=<routes|none>
effective=<routes|none>
suppressed=<routes|none>
disagreement=<0|1>
```

This trace is diagnostic only. The existing ordered registry still selects the
lowering route.

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
resolver_reads_facts_only=1
resolver_returns_allow_or_deny=1
raw_candidates_reported=1
effective_candidates_reported=1
suppressed_candidates_reported=1
reachability_feedback_to_resolver=0
route_selection_changed=0
summary=ok
```
