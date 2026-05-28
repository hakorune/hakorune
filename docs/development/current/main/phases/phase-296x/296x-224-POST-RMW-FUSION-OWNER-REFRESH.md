---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after typed-object field RMW fusion.
Blocker: POST-RMW-FUSION-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-223-TYPED-OBJECT-FIELD-RMW-FUSION-MEASUREMENT.md
---

# 296x-224 Post RMW Fusion Owner Refresh

## Purpose

Refresh the perf owner after row223 accepted the selected-method typed-object
field RMW fusion keeper.

This row does not optimize. It decides the next diagnostic owner from current
perf evidence.

## Evidence

```text
output_contract=typed-object-post-rmw-fusion-owner-refresh-v0
input_contract=typed-object-field-rmw-fusion-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_exact_slot_helper_pct=62.27
perf_exact_slot_get_set_pct=59.97
perf_exact_slot_rmw_helper_pct=2.30
perf_legacy_field_helper_pct=0.00
perf_array_slot_backend_pct=10.80
perf_array_backend_hash_pct=17.82
perf_array_total_pct=28.62
perf_hako_method_pct=9.05
selected_boundary=typed_object_exact_slot_callsite_attribution
next_diagnostic=typed_object_exact_slot_callsite_attribution
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
perf_top_0_pct=16.00
perf_top_0_symbol=nyash.object.exact_slot_set_i64_hii
perf_top_1_pct=13.98
perf_top_1_symbol=core::hash::BuildHasher::hash_one
perf_top_2_pct=11.42
perf_top_2_symbol=nyash.object.exact_slot_get_u64_hii
perf_top_3_pct=9.53
perf_top_3_symbol=nyash.object.exact_slot_get_handle_hii
perf_top_4_pct=9.20
perf_top_4_symbol=nyash.object.exact_slot_set_u64_hiu
perf_top_5_pct=8.47
perf_top_5_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
perf_top_6_pct=6.94
perf_top_6_symbol=nyash.object.exact_slot_get_i64_hii
perf_top_7_pct=6.88
perf_top_7_symbol=nyash.object.exact_slot_set_handle_hii
perf_top_8_pct=3.84
perf_top_8_symbol=<core::hash::sip::Hasher<S> as core::hash::Hasher>::write
perf_top_9_pct=2.33
perf_top_9_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
summary=ok
```

## Decision

```text
selected_owner_family=typed_object_exact_slot_callsite_attribution
selected_reason=exact_slot_get_set_helpers_remain_primary_after_rmw_fusion
next_row=typed_object_exact_slot_callsite_attribution
```

The fused RMW helper is only 2.30% of the sampled cycles. The remaining
exact-slot get/set helpers are still the primary owner, so the next row should
attribute exact-slot helper calls by callsite/method family before another
keeper.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_rmw_fusion_owner_refresh_guard.sh
```
