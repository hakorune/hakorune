Status: Done
Date: 2026-06-17
Scope: add passive fastpath decision vocabulary for the recursive eligibility
resolver.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1034-FASTPATH-ELIGIBILITY-RESOLVER-SSOT-001.md

# FASTPATH-DECISION-VOCAB-000

## Purpose

Add the passive code vocabulary needed before implementing the recursive
fastpath eligibility resolver.

This row does not run the resolver, change route selection, or change backend
lowering.

## Change

Added:

```text
PlanEpoch
FastPathDecision::Allow(LocalFastPathFact)
FastPathDecision::Deny(LocalFastPathFallbackReason)
LocalFastPathFact.plan_epoch
```

The default constructor for `LocalFastPathFact::known_receiver_direct_call`
assigns `PlanEpoch::INITIAL` so existing producers stay behavior-compatible.

MIR JSON metadata now exports:

```text
local_fastpath_facts[*].plan_epoch
```

## Validation

```text
cargo test -q object_storage_plan --lib
cargo test -q build_mir_json_root_emits_local_fastpath_facts --lib
```

Both passed.

## Contract

```text
output_contract=fastpath-decision-vocab-v0
fastpath_decision_vocabulary_defined=1
fastpath_decision_shape=AllowFact_or_DenyReason
fastpath_plan_epoch_vocabulary_defined=1
local_fastpath_fact_plan_epoch_required=1
resolver_execution_enabled=0
backend_behavior_changed=0
route_priority_changed=0
implementation_scope=passive_vocabulary_only
next_task=FASTPATH-DENY-OWNER-MAPPING-001
summary=ok
```

## Stop Lines

```text
do not implement resolver execution in this row
do not let backend read deny reasons
do not treat deny reasons as facts
do not change route priority
do not expand v0 beyond KnownReceiverDirectCall
```
