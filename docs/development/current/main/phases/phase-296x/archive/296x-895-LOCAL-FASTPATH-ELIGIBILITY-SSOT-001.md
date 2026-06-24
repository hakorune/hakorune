# 296x-895 LOCAL-FASTPATH-ELIGIBILITY-SSOT-001

Status: Landed
Date: 2026-06-16

## Contract

```text
output_contract=hako-local-fastpath-eligibility-ssot-v0
source_evidence=296x-894
row_kind=design

fastpath_model=observation_to_eligibility_to_fact_to_backend
local_fastpath_fact_backend_consumable=1
fallback_evidence_backend_consumable=0
fallback_fact_enabled=0
unknown_state_policy=fallback
maybe_published_policy=fallback

local_fastpath_fact_requires_unpublished=1
local_fastpath_fact_requires_alias_class=1
local_fastpath_fact_requires_routeplan=1
local_fastpath_fact_requires_objectstorageplan=1
local_fastpath_fact_requires_backend_support=1

backend_reads_local_fastpath_fact_only=1
backend_reads_fallback_evidence=0
backend_reads_helper_symbol=0
backend_reads_source_variable_name=0

full_escape_engine_required_for_v0=0
interprocedural_fixedpoint_required_for_v0=0
next_task=LOCAL-PUBLICATION-CLASSIFIER-000
summary=ok
```

## Decision

The next local-first optimization layer is a conservative fast-path decision
engine, not a full escape-analysis engine.

The backend may consume only a positive `LocalFastPathFact`. Fallback evidence,
observations, maybe-published states, helper names, and source variable names
are not backend proof.

## Model

```text
Observation
  -> Eligibility Decision
  -> LocalFastPathFact
  -> Backend consumer
```

`LocalFastPathFact` means the backend is allowed to emit a local fast path for a
single site. Its absence means the backend must use the product-compatible
fallback.

## Eligibility Rule

```text
can_fast_path(site, object) =
    closed_world_region(site)
 && alias_class_known(object)
 && publication_state_before(site, alias_class) == Unpublished
 && route_plan(site) is ClosedWorldDirect / Intrinsic / ExactKnown
 && storage_plan(object) is LocalExact / LocalIdentity / Scalarized / LocalNative
 && backend_supports(route_plan, storage_plan)
 && no dynamic Box API required before site
```

If any input is unknown, maybe-published, dynamic, generic, or unsupported, the
decision is fallback.

## V0 Scope

V0 intentionally stays smaller than a general escape-analysis engine.

Allowed:

```text
AliasClass:
  local assignment
  SSA copy
  PHI
  select
  simple receiver alias

PublicationState:
  Unpublished
  Published
  MaybePublished

CallSummary:
  known local/pure call does not publish
  known storing/publishing call publishes
  unknown call publishes Box-like args
```

Deferred:

```text
field-sensitive points-to
heap graph traversal
collection element alias
recursive object graph alias
global interprocedural fixed-point
HostHandle bypass
direct storage and direct call in one pilot
```

## Backend Boundary

Backend consumers may read:

```text
LocalFastPathFact
RoutePlan
ObjectStoragePlan
```

Backend consumers must not read:

```text
ObjectPublicationInventory
FallbackEvidence
helper symbol
source variable name
ArrayReceiverResidenceInput
PublicArrayBoxFallback
```

## Next Rows

```text
LOCAL-PUBLICATION-CLASSIFIER-000:
  passive PublicationState / fallback reason vocabulary

LOCAL-ALIAS-CLASS-MVP-001:
  copy / PHI / select / simple receiver alias report-only union classes

LOCAL-PUBLICATION-INVENTORY-V2-001:
  alias-class based publication state at candidate callsites

LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001:
  produce shadow LocalFastPathFact candidates only
```

## Stop Lines

- do not build a full escape engine first
- do not make fallback evidence backend-readable
- do not create fallback facts
- do not bypass HostHandle in Level 1
- do not combine direct storage and direct call in the same pilot
- do not infer from helper name, benchmark name, or source variable name
- do not move representation decisions into MIRBuilder
