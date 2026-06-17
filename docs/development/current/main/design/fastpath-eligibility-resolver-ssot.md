---
Status: SSOT
Decision: accepted
Date: 2026-06-17
Scope: Demand-driven recursive fastpath eligibility for exact-AOT compiler
lowering.
Related:
  - docs/development/current/main/design/object-storage-plan-boundary-ssot.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-895-LOCAL-FASTPATH-ELIGIBILITY-SSOT-001.md
  - docs/development/current/main/phases/phase-296x/296x-1033-FASTPATH-OPTIMIZATION-CHECKPOINT-001.md
---

# FastPath Eligibility Resolver (SSOT)

## Decision

Use a demand-driven recursive eligibility resolver for local exact-AOT
fastpaths.

The resolver is not a general escape-analysis engine and not a backend emitter.
It is a read-only proof issuer that answers one question for a requested MIR
site:

```text
Can this site receive a backend-consumable LocalFastPathFact?
```

The answer is never a bare boolean.

```rust
enum FastPathDecision {
    Allow(LocalFastPathFact),
    Deny(FastPathDenyReason),
}
```

Short form:

```text
can_fast_path is a recursive proof issuer.
backend reads only LocalFastPathFact.
fallback evidence is report-only.
```

## Pipeline Order

The resolver must run only after all input plans have been fixed.

```text
Phase 1: freeze plans
  RoutePlan
  ObjectPlan / ObjectStoragePlan
  publication state
  alias classes
  backend capability table
  plan_epoch

Phase 2: resolve eligibility
  read-only
  demand-driven
  memoized
  cycle => Deny(CycleDetected)

Phase 3: backend emit
  consumes LocalFastPathFact only
  does not reinterpret MIR node/helper/source names

Phase 4: reachability ledger
  post-hoc selected/preempted/unreachable report
  does not feed back into resolver

Phase 5: closeout
  no route or plan mutation after resolver
```

This order is mandatory.

```text
resolver_reads_frozen_plans_only=1
resolver_mutates_routeplan=0
resolver_mutates_objectplan=0
resolver_mutates_publication=0
backend_reads_local_fastpath_fact_only=1
reachability_is_posthoc=1
```

## Why Reachability Is Not Deny

Eligibility and reachability are separate concerns.

Eligibility answers:

```text
Is a fastpath semantically and representationally allowed for this site?
```

Reachability answers:

```text
Did the active backend route actually select this consumer?
```

Old exact seeds can preempt a newer metadata consumer. That preemption must not
create a recursive dependency between route retirement and eligibility.

Therefore preemption is not an eligibility denial.

```text
do not use Deny(PreemptedByHigherPriorityRoute)
```

Use the hako_check post-hoc ledger instead:

```text
candidate_N_site_id=<site>
candidate_N_family=<family>
candidate_N_reachable=0|1
candidate_N_selected_route=<route|none>
candidate_N_preempted_by=<route|none>
```

Backend may emit only selected reachable facts.

## Plan Epoch

Every `LocalFastPathFact` produced by the resolver must carry the plan epoch it
was produced from.

```rust
struct LocalFastPathFact {
    site_id: FastPathSiteId,
    kind: LocalFastPathKind,
    plan_epoch: PlanEpoch,
    dependencies: Vec<FastPathDependency>,
}
```

If the plan epoch changes, cached resolver decisions and emitted facts are
stale and must be discarded.

```text
fact_plan_epoch_required=1
stale_fact_policy=discard
plan_mutation_after_resolver=0
```

## Resolver Termination

The resolver must terminate by construction:

```text
memoize same query
track in_progress queries
cycle => Deny(CycleDetected)
subqueries bounded by finite MIR sites and plan facts
```

The resolver may recursively ask about dependencies, but it must not trigger
new plan construction.

```text
resolver_creates_new_routeplan=0
resolver_creates_new_objectplan=0
resolver_runs_interproc_fixedpoint=0
```

## V0 Scope

V0 is deliberately narrow:

```text
LocalFastPathKind::KnownReceiverDirectCall
```

Allowed v0 shape:

```text
publication-before-call known receiver
direct RoutePlan target
same-module user function or intrinsic
backend supports direct-call fact
```

V0 does not include:

```text
LocalI64MapGet
ArrayGet / ArraySet
String concat / dead text region
direct storage
HostHandle bypass
Arc elimination
global interprocedural fixed-point
```

## V0 Allow Rule

`KnownReceiverDirectCall` may produce `Allow(LocalFastPathFact)` only if all
conditions hold:

```text
receiver alias class known
publication_state(alias_class, site) == Unpublished
route target known
target is same-module user function or intrinsic
plugin / extern / dynamic dispatch not required
backend supports KnownReceiverDirectCall
plan_epoch matches
```

Any unknown, maybe-published, dynamic, or open-world input must deny.

```text
unknown_policy=Deny
maybe_published_policy=Deny
open_world_policy=Deny
```

## Deny Reasons And Owner Mapping

Every deny reason must map to a next owner lane. Reasons without an owner become
diagnostic trash and invite guesswork.

```text
Deny(OpenWorld)
  owner=route/open-world boundary

Deny(UnknownValue)
  owner=value-origin inventory

Deny(AliasUnknown)
  owner=alias classifier

Deny(PublishedBeforeSite)
  owner=publication classifier

Deny(MaybePublishedBeforeSite)
  owner=publication classifier / PHI freshness

Deny(RoutePlanMissing)
  owner=route proof producer

Deny(DynamicRoute)
  owner=RoutePlan / BoxCallableRegistry

Deny(ObjectPlanMissing)
  owner=ObjectPlan producer

Deny(GenericStorage)
  owner=ObjectStoragePlan producer

Deny(BackendUnsupported)
  owner=backend consumer seam

Deny(CycleDetected)
  owner=recursive dependency / alias cycle inventory

Deny(PhiMergeNotProven)
  owner=PHI lifecycle / alias freshness

Deny(LoopCarriedNotProven)
  owner=loop-carried proof lane

Deny(InterprocSummaryMissing)
  owner=call summary lane
```

## PHI And Loop Policy

V0 is conservative.

PHI may allow only when all incoming values prove the same local fact shape:

```text
same alias class
same publication state
same route/storage dependencies
```

Otherwise:

```text
Deny(PhiMergeNotProven)
```

Loop-carried values deny unless a later row supplies a specific invariant proof:

```text
Deny(LoopCarriedNotProven)
```

## Interprocedural Policy

V0 closed-world scope is intentionally small:

```text
same-module user function
intrinsic
```

Everything else denies:

```text
plugin => Deny(OpenWorld)
extern => Deny(OpenWorld)
unknown call => Deny(InterprocSummaryMissing)
dynamic dispatch => Deny(DynamicRoute)
```

No interprocedural fixed-point is required for v0.

## Backend Boundary

Backend consumers may read:

```text
LocalFastPathFact
plan_epoch on the fact
fact dependencies required for emission
```

Backend consumers must not read:

```text
FastPathDenyReason
FallbackEvidence
ObjectPublicationInventory
helper symbol
benchmark name
source variable name
raw MIR node shape as a proof substitute
```

Backend emits by fact kind, not by re-running eligibility:

```text
emit_fastpath_fact(LocalFastPathFact)
```

## Task Ladder

```text
FASTPATH-ELIGIBILITY-RESOLVER-SSOT-001:
  this SSOT and task ladder

FASTPATH-DECISION-VOCAB-000:
  passive FastPathDecision / FastPathDenyReason / PlanEpoch vocabulary

FASTPATH-DENY-OWNER-MAPPING-001:
  deny reason -> owner lane report contract

FASTPATH-ALIAS-PUBLICATION-MVP-001:
  report-only alias/publication resolver inputs
  include 5-hop alias chain fixture

FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001:
  read-only resolver shadow for KnownReceiverDirectCall

FASTPATH-REACHABILITY-LEDGER-POSTHOC-001:
  post-hoc selected/preempted/unreachable report

FASTPATH-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001:
  backend consumes resolver-produced fact only after shadow + reachability pass
```

## Stop Lines

```text
do not implement bool-only can_fast_path
do not let resolver mutate RoutePlan/ObjectPlan/publication facts
do not let reachability/preemption feed back into eligibility
do not let backend read deny reasons or fallback evidence
do not run global interprocedural fixed-point in v0
do not expand v0 beyond KnownReceiverDirectCall
do not move representation decisions into MIRBuilder
```
