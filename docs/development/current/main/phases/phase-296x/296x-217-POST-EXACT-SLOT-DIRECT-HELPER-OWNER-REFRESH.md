---
Status: Landed
Date: 2026-05-29
Scope: refresh the hot owner after exact-slot typed-object helper no-effect evidence.
Blocker: POST-EXACT-SLOT-DIRECT-HELPER-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-216-TYPED-OBJECT-EXACT-SLOT-DIRECT-HELPER-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-193-MIR-TYPED-FIELD-RESIDENCE-SSOT.md
---

# 296x-217 Post Exact-Slot Direct Helper Owner Refresh

## Purpose

Refresh the perf owner after row216 showed no material body-time improvement
from replacing generic typed-object helpers with exact-slot helpers.

This row does not optimize. It decides the next diagnostic owner from current
perf evidence.

## Evidence

```text
output_contract=typed-object-exact-slot-owner-refresh-v0
input_contract=typed-object-exact-slot-direct-helper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
perf_exact_slot_helper_pct=55.04
perf_legacy_field_helper_pct=0.00
perf_array_slot_backend_pct=19.70
perf_array_backend_hash_pct=18.69
perf_array_total_pct=38.39
perf_hako_method_pct=6.44
selected_boundary=mir_typed_field_direct_op_inventory
next_diagnostic=mir_typed_field_direct_op_net_inventory
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
perf_top_0_pct=19.73
perf_top_0_symbol=nyash.object.exact_slot_get_u64_hii
perf_top_1_pct=17.28
perf_top_1_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_store_i64
perf_top_2_pct=14.95
perf_top_2_symbol=nyash.object.exact_slot_set_i64_hii
perf_top_3_pct=14.59
perf_top_3_symbol=core::hash::BuildHasher::hash_one
perf_top_4_pct=7.04
perf_top_4_symbol=nyash.object.exact_slot_get_handle_hii
perf_top_5_pct=4.90
perf_top_5_symbol=nyash.object.exact_slot_set_u64_hiu
perf_top_6_pct=4.88
perf_top_6_symbol=nyash.object.exact_slot_get_i64_hii
perf_top_7_pct=4.10
perf_top_7_symbol=<core::hash::sip::Hasher<S> as core::hash::Hasher>::write
perf_top_8_pct=3.54
perf_top_8_symbol=nyash.object.exact_slot_set_handle_hii
perf_top_9_pct=2.42
perf_top_9_symbol=nyash_kernel::plugin::array_slot_backend::single_thread_load_encoded_i64
summary=ok
```

## Decision

```text
selected_owner_family=mir_typed_field_direct_op_inventory
selected_reason=exact_slot_helpers_are_now_the_primary_perf_owner_after_generic_validation_removed
next_row=mir_typed_field_direct_op_net_inventory
```

The next row should count whether typed-field helper calls can be erased with a
positive net helper-call delta across the currently hot methods. Do not retry a
selected-method residence transform until inventory proves helper calls can
actually disappear instead of moving to writeback.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_post_exact_slot_direct_helper_owner_refresh_guard.sh
```
