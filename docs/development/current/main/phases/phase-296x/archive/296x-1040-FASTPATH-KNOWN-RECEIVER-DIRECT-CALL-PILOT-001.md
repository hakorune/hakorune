Status: Done
Date: 2026-06-17
Scope: wire known-receiver local fastpath facts through the Allow decision path.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1038-FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001.md
  - docs/development/current/main/phases/phase-296x/296x-1039-FASTPATH-REACHABILITY-LEDGER-POSTHOC-001.md

# FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001

## Purpose

Close the B-lite pilot for known-receiver direct-call facts.

This row does not add a new backend route. It tightens the producer contract:
exported `LocalFastPathFact` rows must come from
`FastPathDecision::Allow(LocalFastPathFact)`.

## Change

Changed the map representation fastpath producer from direct fact construction
to the existing shadow-row decision path:

```text
LocalPublicationInventoryRow
  -> LocalKnownReceiverDirectCallShadowRow
  -> FastPathDecision::Allow(LocalFastPathFact)
  -> local_fastpath_facts
```

Denied decisions remain report-only and are not exported to MIR JSON.
`fallback_reason` stays as a compatibility metadata field and remains `null`
for emitted facts.

## Contract

```text
output_contract=fastpath-known-receiver-direct-call-pilot-b-lite-v0
local_known_receiver_direct_call_pilot_b_lite_enabled=1
local_fastpath_fact_allow_decision_source=1
local_fastpath_deny_mir_json_export_enabled=0
local_fastpath_fact_fallback_reason_compat_null=1
backend_behavior_changed=0
route_priority_changed=0
hosthandle_bypass_enabled=0
storage_direct_enabled=0
product_default_changed=0
next_task=FASTPATH-VOCAB-SLIM-CLOSEOUT-001
summary=ok
```

## Stop Lines

```text
do not export Deny decisions to MIR JSON
do not make fallback evidence backend-consumable
do not add page/method/helper-name special cases
do not change route priority in this row
do not bypass HostHandle or storage representation in this row
```

