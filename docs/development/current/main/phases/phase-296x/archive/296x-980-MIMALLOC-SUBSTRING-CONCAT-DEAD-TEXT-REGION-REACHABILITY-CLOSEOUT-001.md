# 296x-980 MIMALLOC-SUBSTRING-CONCAT-DEAD-TEXT-REGION-REACHABILITY-CLOSEOUT-001

Status: Landed
Date: 2026-06-17

## Purpose

Close the substring-concat dead-text region lane after implementation
reachability review.

The generic backend consumer exists, but the active target front is still
preempted by the older function-level exact seed route. This closeout prevents
the lane from turning into forced reachability or benchmark-specific routing.

## Evidence

The target front still selects:

```text
substring_concat_loop_ascii
```

Proof command:

```bash
bash tools/smokes/v2/profiles/integration/phase137x/phase137x_direct_emit_substring_concat_route_contract.sh
```

Observed:

```text
PASS (... substring-concat exact seed is selected by function-level backend route tag)
```

## Result

```text
output_contract=hako-mimalloc-substring-concat-dead-text-region-reachability-closeout-v0
row_kind=closeout

generic_metadata_path_consumer_exists=1
target_front_reachable_by_new_consumer=0
target_front_preempted_by_exact_seed=1
forced_reachability_allowed=0
benchmark_name_branch_count=0
source_name_branch_count=0
helper_name_only_inference_count=0
runtime_helper_added=0
product_stringbox_storage_changed=0
winner_claim=0

selected_next=MIMALLOC-FRESH-FRONT-SELECTION-AFTER-SUBSTRING-CONCAT-CLOSEOUT-001
summary=ok
```

## Decision

Keep the `StringDeadTextRegionPlan` and generic-path backend consumer as a
metadata-backed seam, but do not continue this front for performance work until
the exact seed route is deliberately retired or bypassed by a separate design
row.

The next optimization step should select a fresh front/owner rather than force
this new consumer into the existing exact-seed path.

## Stop Line

```text
do not delete exact seed route as a drive-by
do not add a benchmark/source/helper-name branch to force the generic consumer
do not claim body-time or ASM win from the new consumer on the current front
do not change product StringBox storage
```
