---
Status: Landed
Date: 2026-05-29
Scope: close small exact-slot helper hunting and select representation/direct lowering design.
Blocker: MICRO-HELPER-LANE-CLOSEOUT-AND-REPRESENTATION-DIRECT-LOWERING-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-296-ALLOC-RESULT-CAPSULE-OWNER-SELECTION-AFTER-RECORD-SUCCESS-HELPER-FUSION.md
  - docs/development/current/main/phases/phase-296x/296x-284-POST-RECORD-SUCCESS-HELPER-FUSION-OWNER-REFRESH.md
---

# 296x-297 Micro Helper Lane Closeout And Representation Direct Lowering Selection

## Purpose

Close the small exact-slot helper hunting lane and select the next design lane.

Rows 286-296 exhausted the row284 exact-slot owner table without finding a new
promising small keeper:

```text
page_queue_helpers:
  recent no-effect keeper in row241

object_lifecycle_facade:
  selected same-block fusion already landed in row231
  positive-net surface remained 4 in rows286-287

page_model_hotpath:
  acquire_usize receiver forwarding had no material effect in row252
  releaseKnownLive RMW had no effect in row268

release_result_capsule:
  recordSuccess helper fusion already landed in row282/283
  birth is setup-shaped

alloc_result_capsule:
  reset batching already landed in row259
  recordSuccess helper fusion already landed in row282/283
  birth is setup-shaped
```

## Evidence

```text
output_contract=micro-helper-lane-closeout-and-representation-direct-lowering-selection-v0
input_contract=alloc-result-capsule-owner-selection-after-record-success-helper-fusion-v0
workload_id=representative-object-lifecycle-small-block-v0
row284_exact_slot_get_set_pct=50.97
row284_family_count=5
excluded_family_0=page_queue_helpers
excluded_reason_0=row241_recent_no_effect
excluded_family_1=object_lifecycle_facade
excluded_reason_1=row231_selected_facade_fusion_already_landed_and_positive_net_surface_still_4
excluded_family_2=page_model_hotpath
excluded_reason_2=row252_acquire_no_material_and_row268_release_no_effect
excluded_family_3=release_result_capsule
excluded_reason_3=row282_record_success_already_landed_and_birth_setup_shaped
excluded_family_4=alloc_result_capsule
excluded_reason_4=row259_reset_and_row282_record_success_already_landed_and_birth_setup_shaped
remaining_small_helper_keeper_count=0
selected_owner=representation_direct_lowering_ssot
selected_reason=helper_calls_remain_large_but_small_helper_owner_table_is_exhausted
next_row=representation_direct_lowering_ssot
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner=representation_direct_lowering_ssot
next_row=representation_direct_lowering_ssot
optimization_open=0
```

Do not add another exact-slot helper or fusion row from the row284 table. The
next durable move is a representation/direct-lowering SSOT that defines how
hot `.hako` object/capsule fields can lower toward C-like scalar/direct access
with runtime helpers only at escape, materialization, or fallback boundaries.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_micro_helper_lane_closeout_and_representation_direct_lowering_selection_guard.sh
```
