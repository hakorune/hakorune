# 296x-899 LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-known-receiver-direct-call-shadow-v0
source_evidence=296x-898
row_kind=passive_shadow_vocabulary

local_known_receiver_direct_call_shadow_defined=1
local_known_receiver_direct_call_shadow_backend_consumable=0
local_known_receiver_direct_call_shadow_fact_optional=1
local_known_receiver_direct_call_shadow_requires_routeplan=1
local_known_receiver_direct_call_shadow_requires_objectstorageplan=1

local_fastpath_fact_backend_consumable=1
fallback_evidence_backend_consumable=0
fallback_fact_enabled=0
backend_new_lowering_enabled=0
object_storage_plan_execution_enabled=0
next_task=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001
summary=ok
```

## Implementation

`LocalKnownReceiverDirectCallShadowRow` is passive vocabulary in
`src/object_storage_plan.rs`.

It combines:

```text
LocalPublicationInventoryRow
RoutePlanId
ObjectStoragePlanId
```

When all inputs are positive, it can carry an optional
`LocalFastPathFact::KnownReceiverDirectCall` candidate. Otherwise it records a
fallback reason.

## Decision

This row still does not enable backend lowering. The shadow row exists so the
next implementation row can consume a positive `LocalFastPathFact` instead of
re-reading observations, fallback evidence, helper names, or source variable
names.

## Tests

```bash
cargo test --lib object_storage_plan -- --nocapture
```

## Stop Lines

- no backend consumption of shadow rows
- no helper-name / source-variable-name inference
- no HostHandle bypass
- no direct storage enablement
- no fallback facts
- no MIRBuilder representation ownership
