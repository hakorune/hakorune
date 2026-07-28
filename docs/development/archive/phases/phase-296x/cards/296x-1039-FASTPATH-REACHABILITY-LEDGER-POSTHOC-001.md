Status: Done
Date: 2026-06-17
Scope: add passive post-hoc fastpath reachability ledger vocabulary.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-1038-FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001.md

# FASTPATH-REACHABILITY-LEDGER-POSTHOC-001

## Purpose

Keep route preemption out of eligibility decisions.

This row adds passive reachability ledger vocabulary so selected/preempted/
unreachable status can be reported after route selection without feeding back
into the recursive resolver.

## Change

Added:

```text
FastPathReachability
FastPathReachability::selected(...)
FastPathReachability::preempted(...)
FastPathReachability::unreachable(...)
```

`preempted` is not a deny reason. It is a post-hoc route-selection state and
does not mutate eligibility.

## Validation

```text
cargo test -q fastpath_reachability_is_posthoc_and_not_a_deny_reason --lib
cargo test -q object_storage_plan --lib
```

Both passed.

## Contract

```text
output_contract=fastpath-reachability-ledger-posthoc-v0
fastpath_reachability_ledger_vocabulary_defined=1
fastpath_reachability_is_posthoc=1
fastpath_preemption_is_deny_reason=0
fastpath_reachability_feedback_to_resolver=0
resolver_execution_enabled=0
backend_behavior_changed=0
route_priority_changed=0
implementation_scope=passive_vocabulary_only
next_task=FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001
summary=ok
```

## Stop Lines

```text
do not feed reachability back into resolver
do not add Deny(PreemptedByHigherPriorityRoute)
do not change route priority in this row
do not enable backend consumption in this row
```
