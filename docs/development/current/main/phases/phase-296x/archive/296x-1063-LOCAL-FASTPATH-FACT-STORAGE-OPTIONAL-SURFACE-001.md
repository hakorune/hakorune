Status: Done
Date: 2026-06-17
Scope: LocalFastPathFact storage-plan optional surface
Related:
  - docs/development/current/main/phases/phase-296x/296x-1062-LOCAL-FASTPATH-FACT-ROUTE-LABEL-SURFACE-001.md

# LOCAL-FASTPATH-FACT-STORAGE-OPTIONAL-SURFACE-001

## Purpose

Prepare `LocalFastPathFact` for user-box known receiver direct-call facts
without inventing fake object-storage proof.

`KnownReceiverDirectCall` is a call-route fastpath. It does not by itself
authorize field layout, HostHandle bypass, Arc removal, or local storage
access. Therefore `ObjectStoragePlanId` must not be mandatory for that fact
kind.

## Implementation

Changed:

```text
LocalFastPathFact.storage_plan:
  ObjectStoragePlanId -> Option<ObjectStoragePlanId>
```

`KnownReceiverDirectCall` now permits `storage_plan=None`.

Map-derived facts still attach storage plan evidence through:

```text
LocalFastPathFact::with_storage_plan(...)
```

so existing map local fastpath metadata remains compatible with current Python
backend consumers.

## Contract

```text
output_contract=local-fastpath-fact-storage-optional-surface-v0
source_evidence=296x-1062

known_receiver_direct_call_storage_plan_required=0
known_receiver_direct_call_requires_route_plan=1
known_receiver_direct_call_authorizes_storage_bypass=0

local_storage_access_storage_plan_required=1
local_field_access_storage_plan_required=1

map_repr_fact_storage_plan_preserved=1
user_box_method_fact_storage_placeholder_allowed=0
fake_object_storage_plan_for_call_route_allowed=0

mir_json_storage_plan_id_nullable=1
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
do not add dummy ObjectStoragePlanId for call-route facts
do not change backend lowering
do not change route priority
do not create fallback facts
```

## Validation

```text
cargo test -q object_storage_plan --lib
cargo test -q map_repr_plan --lib
cargo test -q build_mir_json_root_emits_local_fastpath_facts --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
