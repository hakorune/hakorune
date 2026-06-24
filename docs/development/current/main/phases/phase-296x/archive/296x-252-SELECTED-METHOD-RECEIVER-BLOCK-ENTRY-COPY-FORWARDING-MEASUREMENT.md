---
Status: Landed
Date: 2026-05-29
Scope: measure selected-method receiver block-entry copy forwarding.
Blocker: SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-251-SELECTED-METHOD-RECEIVER-BLOCK-ENTRY-COPY-FORWARDING-IMPLEMENTATION.md
---

# 296x-252 Selected Method Receiver Block-Entry Copy Forwarding Measurement

## Purpose

Measure the row251 implementation before accepting it as a performance keeper.

The exact-EXE is built once and run five times to avoid mixing compile/link time
with workload body timing.

## Evidence

```text
output_contract=selected-method-receiver-block-entry-copy-forwarding-measurement-v0
input_contract=selected-method-receiver-block-entry-copy-forwarding-implementation-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=prebuilt_exact_exe_body_timing_after_receiver_forwarding
sample_count=5
sample_0_body_elapsed_ns=116000000
sample_1_body_elapsed_ns=115000000
sample_2_body_elapsed_ns=117000000
sample_3_body_elapsed_ns=116000000
sample_4_body_elapsed_ns=117000000
receiver_forwarding_body_elapsed_ns=116000000
previous_rmw_fusion_body_elapsed_ns=116000000
body_elapsed_delta_ns=0
keeper_acceptance_min_improvement_pct=3
keeper_effect=no_material_perf_effect
structural_ir_effect=selected_receiver_param0_adds_removed
remaining_param0_copy_add_count=1
post_measurement_action=owner_refresh
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
selected_owner_family=post_receiver_forwarding_owner_refresh
selected_reason=receiver_forwarding_removed_selected_ir_copies_but_did_not_move_body_median
next_row=post_receiver_forwarding_owner_refresh
optimization_open=0
```

The implementation is retained as a narrow structural cleanup, but not counted
as a performance keeper. The next row must refresh hot ownership before another
optimization attempt.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_selected_method_receiver_block_entry_copy_forwarding_measurement_guard.sh
```
