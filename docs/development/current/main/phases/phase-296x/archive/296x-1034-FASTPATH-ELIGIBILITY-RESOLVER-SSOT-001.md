Status: Done
Date: 2026-06-17
Scope: document and task the recursive fastpath eligibility resolver design.
Related:
  - docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-895-LOCAL-FASTPATH-ELIGIBILITY-SSOT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1033-FASTPATH-OPTIMIZATION-CHECKPOINT-001.md

# FASTPATH-ELIGIBILITY-RESOLVER-SSOT-001

## Purpose

Replace the growing per-front fastpath matcher model with a demand-driven
recursive eligibility resolver design.

This row is docs/tasking only. It does not add Rust vocabulary, change backend
consumers, or alter any route priority.

## Decision

The accepted design is:

```text
Plan freeze
  -> read-only recursive resolver
  -> LocalFastPathFact
  -> backend fact consumer
  -> post-hoc reachability ledger
```

The resolver returns a decision, not a boolean:

```text
Allow(LocalFastPathFact)
Deny(FastPathDenyReason)
```

Eligibility and reachability are separated:

```text
resolver decides semantic/planning eligibility
ReachabilityLedger reports selected/preempted/unreachable after backend route selection
preemption is not a Deny reason
```

## Fixed Invariants

```text
resolver_reads_frozen_plans_only=1
resolver_mutates_routeplan=0
resolver_mutates_objectplan=0
resolver_mutates_publication=0
fact_plan_epoch_required=1
reachability_is_posthoc=1
backend_reads_local_fastpath_fact_only=1
fallback_fact_enabled=0
unknown_policy=Deny
cycle_policy=Deny(CycleDetected)
v0_scope=KnownReceiverDirectCall
```

## Task Ladder

```text
FASTPATH-DECISION-VOCAB-000:
  Add passive FastPathDecision / FastPathDenyReason / PlanEpoch vocabulary.
  No lowering.

FASTPATH-DENY-OWNER-MAPPING-001:
  Add report vocabulary mapping Deny reason to next owner lane.
  No lowering.

FASTPATH-ALIAS-PUBLICATION-MVP-001:
  Build report-only resolver inputs for copy / PHI / simple alias and
  publication state.
  Include a 5-hop alias-chain fixture.
  No backend consumption.

FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001:
  Run resolver shadow for KnownReceiverDirectCall only.
  Produce Allow/Deny report.
  No backend consumption.

FASTPATH-REACHABILITY-LEDGER-POSTHOC-001:
  Report whether allowed facts are selected, preempted, or unreachable in the
  active backend route.
  No feedback into resolver.

FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001:
  If shadow + reachability are green, let backend consume resolver-produced
  KnownReceiverDirectCall facts.
```

## Deferred

```text
LocalI64MapGet
ArrayGet / ArraySet
String concat / dead text region
direct storage
HostHandle bypass
Arc elimination
global interprocedural fixed-point
```

## Stop Lines

```text
do not implement bool-only can_fast_path
do not let resolver mutate plans
do not make preemption an eligibility Deny reason
do not let backend read fallback evidence
do not expand v0 beyond KnownReceiverDirectCall
do not move representation decisions into MIRBuilder
```

## Contract

```text
output_contract=fastpath-eligibility-resolver-ssot-v0
design_ssot=docs/development/current/main/design/fastpath-eligibility-resolver-ssot.md
resolver_model=demand_driven_recursive
decision_shape=AllowFact_or_DenyReason
plan_freeze_required=1
plan_epoch_required=1
reachability_posthoc=1
preemption_deny_reason_enabled=0
backend_reads_fact_only=1
v0_kind=KnownReceiverDirectCall
implementation_started=0
next_task=FASTPATH-DECISION-VOCAB-000
summary=ok
```
