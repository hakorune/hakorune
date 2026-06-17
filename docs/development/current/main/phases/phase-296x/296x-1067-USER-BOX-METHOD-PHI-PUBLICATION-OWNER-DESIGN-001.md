Status: Done
Date: 2026-06-17
Scope: user-box method publication blocker owner selection
Previous:
  - docs/development/current/main/phases/phase-296x/296x-1066-USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-PREFLIGHT-001.md

# USER-BOX-METHOD-PHI-PUBLICATION-OWNER-DESIGN-001

## Purpose

Decide whether the dominant `phi_merge_publication_not_proven` blocker should
be opened as the next implementation owner for user-box method
`LocalFastPathFact` production.

## Evidence

The active object-lifecycle preflight reported:

```text
known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19

user_box_method_publication_classification_count=19
publication_fact_allowed_count=0
publication_maybe_published_count=19
top_publication_blocker_proof=phi_merge_publication_not_proven
top_publication_blocker_count=13
```

Inspecting the target MIR shows most PHI receivers are pass-through merges of
the same incoming values across nearby branches, for example:

```text
%93  = phi [ %0, 591 ], [ %0, 594 ]   # facade receiver from param
%98  = phi [ %100, 591 ], [ %99, 594 ] # alloc_result receiver copied from field state
%101 = phi [ %103, 591 ], [ %102, 594 ] # queue receiver copied from field state
%211 = phi [ %111, 606 ], [ %111, 612 ] # page receiver from selectPage call
```

The PHI shape is therefore often simple, but the incoming roots are not
necessarily local unpublished objects:

```text
param origin:
  requires interprocedural publication proof

field_get origin:
  is already product object state

call_result origin:
  requires callee publication summary
```

## Decision

Do not open a broad PHI publication implementation row as the next owner.

Only this narrow PHI rule is allowed later:

```text
phi_publication_b_lite:
  all incoming values resolve to the same origin
  and every incoming origin is already classified Unpublished by the existing
  local publication classifier
  and no alias publication occurs before the callsite
```

This rule is safe, but it is not expected to unlock the active target by
itself because the current PHI roots are mostly param, field_get, or call
results.

## Design Finding

The stronger blocker is probably not PHI ownership itself. It is the current
assumption that `KnownReceiverDirectCall` requires an unpublished receiver.

That assumption is valid for storage/HostHandle/Arc bypass. It may be too
strict for a call-route-only fastpath:

```text
KnownReceiverDirectCall:
  route known
  method target known
  receiver carrier remains product-compatible
  no storage bypass
  no HostHandle bypass
  no Arc retirement
```

If the backend still passes the same product-compatible receiver carrier, then
published object state may still be eligible for direct method dispatch. That
is a different proof from local unpublished storage.

## Next Design Point

Before implementing PHI proof, decide whether user-box method direct-call facts
need `publication_state=Unpublished`.

```text
next_task=USER-BOX-METHOD-DIRECT-CALL-PUBLICATION-REQUIREMENT-DESIGN-001
```

That row should choose one of:

```text
A. Keep current rule:
   KnownReceiverDirectCall requires Unpublished.
   Then PHI-B-lite is safe but likely low value for the active target.

B. Split call-route fastpath from storage-publication fastpath:
   KnownReceiverDirectCall may accept product-compatible published receivers
   when RoutePlan proves the method target and no storage/handle bypass occurs.
   This is more likely to unlock the active target, but needs a precise
   backend-carrier contract.

C. Stop user-box method fastpath for this front:
   switch owner to a different active hot boundary.
```

## Contract

```text
output_contract=user-box-method-phi-publication-owner-design-v0

target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

dominant_blocker=phi_merge_publication_not_proven
dominant_blocker_count=13

phi_b_lite_allowed=1
phi_b_lite_requires_same_origin=1
phi_b_lite_requires_incoming_unpublished=1
phi_b_lite_expected_to_unlock_active_target=0

broad_phi_publication_engine_allowed=0
interprocedural_publication_summary_opened=0
field_get_publication_reclassification_opened=0
backend_lowering_changed=0
local_fastpath_fact_producer_opened=0

next_task=USER-BOX-METHOD-DIRECT-CALL-PUBLICATION-REQUIREMENT-DESIGN-001
summary=ok
```

## Stop Lines

```text
do not treat same-root PHI as unpublished unless incoming roots are already Unpublished
do not open a broad CFG/PHI publication engine from this row
do not add interprocedural callee publication summaries in this row
do not reclassify field_get product state as local unpublished in this row
do not emit user-box LocalFastPathFact from publication MaybePublished rows
do not change backend lowering or route priority
```
