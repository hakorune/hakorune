Status: Done
Date: 2026-06-17
Scope: make KnownReceiverDirectCall shadow rows expose FastPathDecision.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1037-FASTPATH-ALIAS-PUBLICATION-MVP-001.md

# FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001

## Purpose

Wire the existing report-only `LocalKnownReceiverDirectCallShadowRow` to the new
decision vocabulary.

This is still shadow-only. The backend continues to read existing
`LocalFastPathFact` metadata only when already produced by prior rows. This row
does not run a recursive resolver and does not change backend lowering.

## Change

`LocalKnownReceiverDirectCallShadowRow` now carries:

```text
decision: FastPathDecision
```

The row still preserves:

```text
candidate_fact: Option<LocalFastPathFact>
fallback_reason: Option<LocalFastPathFallbackReason>
```

Missing route/storage inputs now map to more precise passive deny reasons:

```text
missing route -> Deny(RoutePlanMissing)
missing storage/object plan -> Deny(ObjectPlanMissing)
maybe published -> Deny(MaybePublishedBeforeSite)
```

## Validation

```text
cargo test -q local_known_receiver_direct_call_shadow_row_creates_fact_only_when_all_inputs_are_positive --lib
cargo test -q object_storage_plan --lib
```

Both passed.

## Contract

```text
output_contract=fastpath-known-receiver-direct-call-shadow-v0
local_known_receiver_direct_call_shadow_decision_defined=1
shadow_outputs_allow_or_deny=1
resolver_execution_enabled=0
backend_behavior_changed=0
route_priority_changed=0
implementation_scope=shadow_only
next_task=FASTPATH-REACHABILITY-LEDGER-POSTHOC-001
summary=ok
```

## Stop Lines

```text
do not enable recursive resolver execution
do not make shadow deny reasons backend-consumable
do not change backend route priority
do not expand v0 beyond KnownReceiverDirectCall
```
