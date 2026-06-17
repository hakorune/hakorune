Status: Done
Date: 2026-06-17
Scope: user-box method LocalFastPathFact producer design
Related:
  - docs/development/current/main/phases/phase-296x/296x-1059-FRESH-COMPILER-OWNER-SELECTION-006.md
  - docs/development/current/main/phases/phase-296x/296x-1063-LOCAL-FASTPATH-FACT-STORAGE-OPTIONAL-SURFACE-001.md
Artifact:
  - target/tmp/mimalloc_object_lifecycle.mir.json

# USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-DESIGN-001

## Purpose

Decide how to produce `LocalFastPathFact` rows from
`user_box_method_routes` without weakening the fact/fallback boundary.

The current target has direct same-module user-box method routes but zero local
fastpath facts:

```text
known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
```

## Current Evidence

The target method exposes 19 user-box direct method routes:

```text
function=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
route_count=19
all_route_reason=None
route_plan_label=user_box.method_call
```

However, current metadata does not expose publication / alias proof for those
receivers:

```text
local_fastpath_facts=0
receiver_snapshot_publication_plans=0
required_fastpath_regions=0
fastpath_obligations=0
```

So the evidence is route-positive, but publication-state unknown.

## Decision

Adopt A-lite:

```text
Fact requires:
  route_plan known
  route_plan_label known
  receiver alias known
  publication_state_before_site=Unpublished

Route-only evidence:
  does not create LocalFastPathFact
  may create a report-only candidate/deny row
```

Do not create `LocalFastPathFact` from `user_box_method_routes` alone.
`LocalFastPathFact` remains a positive backend-consumable proof, not a route
hint.

## Required Next Row

Before a user-box producer can emit facts, add a conservative publication
classifier for the current target:

```text
next_task=USER-BOX-METHOD-PUBLICATION-CLASSIFIER-A-LITE-001
```

The classifier must be conservative:

```text
unknown_alias -> no fact
maybe_published -> no fact
published_before_site -> no fact
unknown_call_with_receiver_or_alias -> no fact
```

V0 can be narrow. It only needs to classify enough current target receivers to
prove or deny the 19 route-positive gaps. It does not need a full escape
engine.

## Contract

```text
output_contract=user-box-method-local-fastpath-fact-producer-design-v0
source_evidence=296x-1059,target/tmp/mimalloc_object_lifecycle.mir.json

selected_policy=A-lite
route_only_fact_enabled=0
user_box_method_fact_requires_publication_proof=1
user_box_method_fact_requires_alias_class=1
user_box_method_fact_requires_route_plan_label=1

known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
publication_proof_available_for_target=0

route_only_candidate_report_allowed=1
route_only_candidate_backend_consumable=0
fallback_fact_enabled=0
backend_lowering_changed=0
route_priority_changed=0
winner_claim_allowed=0
implementation_started=0

next_task=USER-BOX-METHOD-PUBLICATION-CLASSIFIER-A-LITE-001
summary=ok
```

## Stop Lines

```text
do not emit user-box LocalFastPathFact from route-only evidence
do not assume PublicationState::Unpublished for user-box receivers
do not add dummy alias or storage proof
do not change backend lowering
do not change route priority
do not create fallback facts
```

## Validation

```text
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
