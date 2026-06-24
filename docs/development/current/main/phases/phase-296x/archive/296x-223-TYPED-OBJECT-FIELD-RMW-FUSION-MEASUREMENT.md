---
Status: Landed
Date: 2026-05-29
Scope: measure the selected-method typed-object field RMW fusion keeper.
Blocker: TYPED-OBJECT-FIELD-RMW-FUSION-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-222-TYPED-OBJECT-FIELD-RMW-FUSION-KEEPER.md
---

# 296x-223 Typed-Object Field RMW Fusion Measurement

## Purpose

Measure the fused typed-object exact-slot RMW helper on the object-lifecycle
exact-EXE workload and close the keeper decision before selecting the next hot
owner.

This row does not open helper-free direct typed-field access, MIR typed-field
residence, provider activation, replacement, hooks, globals, or winner claims.

## Evidence

```text
output_contract=typed-object-field-rmw-fusion-measurement-v0
input_contract=typed-object-field-rmw-fusion-keeper-v0
base_measurement_contract=typed-object-exact-slot-direct-helper-measurement-v0
workload_id=representative-object-lifecycle-small-block-v0
measurement_scope=object_lifecycle_exact_exe_after_typed_object_field_rmw_fusion
sample_count=3
typed_object_backend=single_thread_exact
array_slot_backend=single_thread_exact
single_thread_exact_floor_body_elapsed_ns=119000000
rmw_fusion_body_elapsed_ns=116000000
body_elapsed_delta_ns=3000000
rmw_fusion_body_ratio_pct=97
keeper_acceptance_min_improvement_pct=3
single_thread_exact_floor_external_elapsed_ms=120
rmw_fusion_external_elapsed_ms=120
keeper_effect=accepted
rmw_fusion_keeper=1
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Decision

```text
keeper_effect=accepted
rmw_fusion_keeper=1
next_diagnostic=post_rmw_fusion_owner_refresh
optimization_open=0
```

The body-time win is small but meets the existing 3% keeper threshold for this
diagnostic lane. The next row must refresh the current hot owner before another
optimization.

## Guard

```bash
bash tools/checks/k2_wide_phase296x_typed_object_field_rmw_fusion_measurement_guard.sh
```
