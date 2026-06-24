---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001
Scope: Rerun object-lifecycle body timing and MIR copy/route classification
  after the call-operand route-carrier lane closed, then decide whether another
  implementation owner is justified.
Related:
  - docs/development/current/main/phases/phase-296x/296x-765-CALL-OPERAND-ROUTE-CARRIER-CLOSEOUT-AND-FRESH-OWNER-SELECTION-001.md
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001

## Result

```text
output_contract=hako-mimalloc-body-timing-rebaseline-after-call-operand-closeout-v0
source_evidence=296x-765,296x-753
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
measurement_tmpdir=/tmp/hakorune_row766_rebaseline.6rqz21
measurement_pair_report=/tmp/hakorune_row766_rebaseline.6rqz21/pair.out
taxonomy_report=/tmp/hakorune_row766_rebaseline.6rqz21/taxonomy.out
position_report=/tmp/hakorune_row766_rebaseline.6rqz21/position.out
hako_body_elapsed_ns=7000000
c_body_elapsed_ns=3274627
body_elapsed_gap_ns=3725373
body_elapsed_ratio=2.138
hako_external_elapsed_ms=10
c_external_elapsed_ms=10
external_elapsed_ratio=1.000
gap_owner=measurement_harness
gap_confidence=low
evidence_quality=single_sample_small_gap
gap_reason=body_gap_not_large_enough_for_owner
copy_count=51
local_like_copy_count=20
backend_route_carrier_copy_count=19
route_aware_candidate_copy_count=19
dominant_position=phi_edge
dominant_local_like_position=block_entry
dominant_route_carrier_role=call_operand
call_operand_route_carrier_copy_count=13
call_operand_receiver_route_carrier_copy_count=2
call_operand_arg_route_carrier_copy_count=11
call_result_route_carrier_copy_count=7
field_base_route_carrier_copy_count=7
field_set_value_route_carrier_copy_count=9
receiver_lane_closed=1
arg_lane_closed=1
call_operand_route_carrier_lane_closed=1
fresh_high_confidence_implementation_owner_selected=0
selected_owner=none
selected_owner_reason=body_gap_not_large_enough_for_compiler_lowering_owner
selected_next_action=measurement_hygiene_refresh
implementation_allowed=0
measurement_required=1
winner_claim=0
startup_lane_reopened=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
product_default_changed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

Commands:

```bash
bash tools/allocator/hako_mimalloc_direct_exact_pair.sh \
  --out /tmp/hakorune_row766_rebaseline.6rqz21/pair.out \
  --tmp-keep

python3 tools/allocator/hako_mimalloc_object_lifecycle_body_timing_gap_taxonomy.py \
  --input /tmp/hakorune_row766_rebaseline.6rqz21/pair.out \
  --out /tmp/hakorune_row766_rebaseline.6rqz21/taxonomy.out

source tools/allocator/mimalloc_direct_exact_env.sh
target/release/hakorune --backend mir \
  --emit-mir-json /tmp/hakorune_row766_rebaseline.6rqz21/app.mir.json \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako
python3 tools/checks/pure_first_route_preflight.py \
  /tmp/hakorune_row766_rebaseline.6rqz21/app.mir.json
python3 tools/allocator/mir_local_ssa_copy_position_probe.py \
  --mir-json /tmp/hakorune_row766_rebaseline.6rqz21/app.mir.json \
  --out /tmp/hakorune_row766_rebaseline.6rqz21/position.out
```

## Decision

Do not open another compiler-lowering implementation row from this evidence.
The previous large product-route body gap is not reproduced by the canonical
current direct-exact pair:

```text
previous_body_elapsed_ratio=79.586
current_body_elapsed_ratio=2.138
gap_owner=measurement_harness
gap_confidence=low
```

The MIR still contains copy/route-carrier residue, but the current body timing
no longer proves that residue is the active hot owner. The correct next move is
measurement hygiene, not another LocalSSA / PHI / call-operand implementation
patch.

## Stop Line

```text
do not implement from this rebaseline row
do not patch LocalSSA::ensure_fallback_copy
do not reopen PHI-edge, block-entry, receiver, or arg forwarding
do not add broad copy coalescing
do not special-case source names, helper names, or benchmark names
do not move Box/Object management into MIRBuilder
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001:
  explain the 79.586 -> 2.138 ratio shift, decide whether row753 used stale
  or mismatched measurement state, and only then choose a fresh optimization
  owner if high-confidence evidence reappears
```
