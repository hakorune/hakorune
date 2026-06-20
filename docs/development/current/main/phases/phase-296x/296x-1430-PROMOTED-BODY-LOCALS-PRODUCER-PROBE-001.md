# 296x-1430 PROMOTED-BODY-LOCALS-PRODUCER-PROBE-001

Status: open
Date: 2026-06-20

## Purpose

Fixture-guard trim and DigitPos producers as `promoted_body_locals` name
recorders only.

## Selected By

```text
296x-1429-POST-PROMOTED-BODY-LOCALS-INVENTORY-OWNER-SELECTION-001
```

## Scope

```text
producers:
  TrimRouteInfo::to_carrier_info
  DigitPos promotion

positive claim:
  records original LoopBodyLocal name into CarrierInfo.promoted_body_locals

negative claims:
  join_id_producer=0
  route_lowering_claim=0
  resolver_selection_owner=0
  converter_emission_added=0
```

## Expected Artifacts

```text
facts=docs/development/current/main/design/fixtures/rust-lifecycle/promoted-body-locals-producer-facts-v0.json
plan=docs/development/current/main/design/fixtures/rust-lifecycle/promoted-body-locals-producer-plan-v0.json
oracle=docs/development/current/main/design/fixtures/rust-lifecycle/promoted-body-locals-producer-oracle-vectors-v0.json
guard=tools/checks/rust_lifecycle_promoted_body_locals_producer_guard.sh
```

## Acceptance

```text
trim_records_promoted_body_local=1
digitpos_records_promoted_body_local=1
producer_facts_fixture=green
producer_plan_fixture=green
producer_oracle_vectors=green
join_id_producer=0
route_lowering_claim=0
general_resolver_implemented=0
backend_behavior_changed=0
```

Checks:

```bash
bash tools/checks/rust_lifecycle_promoted_body_locals_producer_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_resolve_join_id=1
do_not_implement_trim_or_digitpos_route_lowering=1
do_not_expand_converter_emitter=1
do_not_modify_Rust_behavior=1
```

