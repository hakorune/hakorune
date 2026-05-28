---
Status: Landed
Date: 2026-05-29
Scope: implement selected releaseLocalKnownLive single-use RMW fusion.
Blocker: PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-266-PAGE-MODEL-RELEASE-KNOWN-LIVE-SINGLE-USE-RMW-GUARD-SURFACE.md
---

# 296x-267 Page Model Release Known Live Single-Use RMW Implementation

## Purpose

Implement the selected `releaseLocalKnownLive/1` single-use RMW fusion by
reusing the existing C ABI same-module typed-field RMW lowering.

This row does not add a runtime helper and does not change `.hako` source. It
only extends the selected page-model RMW target list so the existing matcher can
fuse the two single-use `usize` counter pairs from row266.

## Evidence

```text
output_contract=page-model-release-known-live-single-use-rmw-implementation-v0
input_contract=page-model-release-known-live-single-use-rmw-guard-surface-v0
workload_id=representative-object-lifecycle-small-block-v0
implementation_owner=c_abi_same_module_typed_field_rmw_fusion
selected_method=HakoAllocPageModel.releaseLocalKnownLive/1
selected_field_0=local_free_count
selected_field_1=retire_count
selected_slot_0=11
selected_slot_1=17
existing_helper_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
new_runtime_helper_added=0
hako_source_change=0
same_module_page_model_rmw_target_list=updated
semantic_proof_summary=ok
planned_net_helper_call_delta=2
multi_use_rmw_fused=0
array_bridge_fused=0
generic_typed_field_residence_open=0
generic_cse_open=0
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
selected_owner_family=page_model_release_known_live_single_use_rmw_measurement
selected_reason=selected_lowering_change_is_landed_and_semantic_proof_remains_ok
next_row=page_model_release_known_live_single_use_rmw_measurement
optimization_open=0
```

The next row must measure before accepting this as a keeper. It must compare
against the row259/row260 exact-EXE floor and keep winner/replacement lanes
closed.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_page_model_release_known_live_single_use_rmw_implementation_guard.sh
```
