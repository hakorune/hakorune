---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after selected facade get/set fusion.
Blocker: POST-SELECTED-FACADE-GET-SET-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-231-SELECTED-FACADE-SAME-BLOCK-GET-SET-MEASUREMENT.md
---

# 296x-232 Post Selected Facade Get/Set Owner Refresh

## Purpose

Refresh the perf owner after row231 accepted the selected-facade same-block
get/set fusion keeper.

This row does not optimize. It decides the next diagnostic owner from current
perf evidence.

## Evidence

```text
output_contract=post-selected-facade-get-set-owner-refresh-v0
input_contract=selected-facade-same-block-get-set-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_exact_slot_helper_pct=64.99
perf_exact_slot_get_set_pct=61.59
perf_exact_slot_rmw_helper_pct=3.40
perf_legacy_field_helper_pct=0.00
perf_array_slot_backend_pct=12.42
perf_array_backend_hash_pct=16.13
perf_array_total_pct=28.55
perf_hako_method_pct=5.54
selected_boundary=post_facade_exact_slot_callsite_attribution_refresh
next_diagnostic=post_facade_exact_slot_callsite_attribution_refresh
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
perf_top_0_pct=15.19
perf_top_0_symbol=nyash.object.exact_slot_get_handle_hii
perf_top_1_pct=14.54
perf_top_1_symbol=core::hash::BuildHasher::hash_one
perf_top_2_pct=12.09
perf_top_2_symbol=nyash.object.exact_slot_get_i64_hii
perf_top_3_pct=12.03
perf_top_3_symbol=nyash.object.exact_slot_set_i64_hii
perf_top_4_pct=11.12
perf_top_4_symbol=nyash.object.exact_slot_get_u64_hii
perf_top_5_pct=10.84
perf_top_5_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
perf_top_6_pct=7.13
perf_top_6_symbol=nyash.object.exact_slot_set_u64_hiu
perf_top_7_pct=4.03
perf_top_7_symbol=nyash.object.exact_slot_set_handle_hii
perf_top_8_pct=3.40
perf_top_8_symbol=nyash.object.exact_slot_rmw_add_u64_hiii
perf_top_9_pct=1.59
perf_top_9_symbol=<core::hash::sip::Hasher<S> as core::hash::Hasher>::write
summary=ok
```

## Decision

```text
selected_owner_family=post_facade_exact_slot_callsite_attribution_refresh
selected_reason=exact_slot_get_set_helpers_remain_primary_after_selected_facade_fusion
next_row=post_facade_exact_slot_callsite_attribution_refresh
optimization_open=0
```

The selected facade fusion was accepted, but exact-slot get/set helpers still
dominate the sampled cycles. The next row should refresh exact-slot callsite
attribution with the current binary before selecting another keeper.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_selected_facade_get_set_owner_refresh_guard.sh
```
