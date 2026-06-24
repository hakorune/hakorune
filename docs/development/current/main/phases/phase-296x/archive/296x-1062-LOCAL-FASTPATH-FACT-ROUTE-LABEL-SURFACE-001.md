Status: Done
Date: 2026-06-17
Scope: LocalFastPathFact route-plan label surface
Related:
  - docs/development/current/main/phases/phase-296x/296x-1060-LOCAL-FASTPATH-FACT-PRODUCER-GAP-DESIGN-001.md
  - docs/development/current/main/phases/phase-296x/296x-1061-LOCAL-FASTPATH-FACT-AGGREGATOR-SURFACE-001.md

# LOCAL-FASTPATH-FACT-ROUTE-LABEL-SURFACE-001

## Purpose

Remove the hardcoded MIR JSON `route_plan=map_repr.generic_hash_runtime`
assumption from `LocalFastPathFact` export before adding non-map producers.

This row keeps existing map fact behavior unchanged and does not add user-box
facts.

## Implementation

Added an explicit route-plan label to `LocalFastPathFact`:

```text
LocalFastPathFact.route_plan_label
```

`LocalKnownReceiverDirectCallShadowRow` now requires a route-plan label before
it can emit `FastPathDecision::Allow(LocalFastPathFact)`. Missing labels deny
with the existing `RoutePlanMissing` reason, so no backend-consumable fact is
created with an ambiguous route owner.

The MIR JSON exporter now emits:

```text
route_plan=fact.route_plan_label
```

instead of a map-specific constant.

## Contract

```text
output_contract=local-fastpath-fact-route-label-surface-v0
source_evidence=296x-1060,296x-1061

local_fastpath_fact_route_plan_label_defined=1
mir_json_route_plan_label_hardcoded_map_repr=0
mir_json_route_plan_label_source=LocalFastPathFact.route_plan_label

map_repr_fact_route_plan_label=map_repr.generic_hash_runtime
user_box_fact_route_plan_label_reserved=user_box.method_call
user_box_method_fact_producer_enabled=0

missing_route_plan_label_allows_fact=0
missing_route_plan_label_deny_reason=RoutePlanMissing

backend_lowering_changed=0
route_priority_changed=0
fallback_fact_enabled=0
winner_claim_allowed=0

next_task=USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-001
summary=ok
```

## Stop Lines

```text
do not add user-box facts in this row
do not change backend lowering
do not change route priority
do not create fallback facts
do not infer route_plan label from helper/source/benchmark names
```

## Validation

```text
cargo test -q object_storage_plan --lib
cargo test -q map_repr_plan --lib
cargo test -q build_mir_json_root_emits_local_fastpath_facts --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
