Status: Done
Date: 2026-06-17
Scope: user-box method LocalFastPathFact producer preflight
Related:
  - docs/development/current/main/phases/phase-296x/296x-1065-USER-BOX-METHOD-PUBLICATION-CLASSIFIER-A-LITE-001.md
Artifact:
  - target/tmp/mimalloc_object_lifecycle.mir.json

# USER-BOX-METHOD-LOCAL-FASTPATH-FACT-PRODUCER-PREFLIGHT-001

## Purpose

Check whether the new report-only publication classifier gives the active
object-lifecycle front enough `Unpublished` receiver evidence to safely open a
user-box `LocalFastPathFact` producer.

## Evidence

Regenerated the MIR JSON from the active object-lifecycle app:

```bash
cargo run -q --bin hakorune -- --backend mir \
  --emit-mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
```

Focused preflight:

```bash
python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body
```

Result:

```text
known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
thin_entry_method_candidate_count=19

user_box_method_publication_classification_count=19
publication_fact_allowed_count=0
publication_maybe_published_count=19
publication_published_count=0

top_publication_blocker_proof=phi_merge_publication_not_proven
top_publication_blocker_count=13
```

Classifier breakdown for the target method:

```text
field_get_origin_is_published_object_state=2
phi_merge_publication_not_proven=13
call_result_requires_callee_publication_summary=2
param_origin_requires_interprocedural_publication_proof=2
```

## Decision

Do not implement the user-box method `LocalFastPathFact` producer for the
current target yet.

The route surface is positive, but the publication surface is still zero for
the active method:

```text
publication_fact_allowed_count=0
```

Opening the producer now would either produce zero target facts or tempt a
fallback/guess path. Both violate the fact/fallback boundary.

## Next Design Point

The next owner is the dominant publication blocker:

```text
next_task=USER-BOX-METHOD-PHI-PUBLICATION-OWNER-DESIGN-001
```

That design row should decide whether PHI publication can be handled by a
narrow alias-preserving rule or whether the current target needs a different
front/owner selection.

## Contract

```text
output_contract=user-box-method-local-fastpath-fact-producer-preflight-v0
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

known_receiver_direct_method_route_count=19
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
user_box_method_publication_classification_count=19
publication_fact_allowed_count=0
publication_maybe_published_count=19
top_publication_blocker_proof=phi_merge_publication_not_proven
top_publication_blocker_count=13

user_box_method_fact_producer_opened=0
backend_lowering_changed=0
route_priority_changed=0
fallback_fact_enabled=0
winner_claim_allowed=0

next_task=USER-BOX-METHOD-PHI-PUBLICATION-OWNER-DESIGN-001
summary=ok
```

## Stop Lines

```text
do not emit LocalFastPathFact when publication_fact_allowed_count=0
do not treat PHI as unpublished without a PHI publication design row
do not promote param/call/field_get origins in this preflight row
do not change backend lowering
do not change route priority
```

## Validation

```bash
cargo run -q --bin hakorune -- --backend mir \
  --emit-mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako

python3 tools/hako_check/fastpath_gap_inventory.py \
  --mir-json target/tmp/mimalloc_object_lifecycle.mir.json \
  --method 'HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1' \
  --front object_lifecycle_body

python3 -m unittest tools.hako_check.tests.test_fastpath_gap_inventory
```
