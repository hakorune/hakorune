---
Status: Landed
Date: 2026-05-28
Scope: define the body timing and exact C pair measurement contract for the object-lifecycle workload.
Blocker: OBJECT-LIFECYCLE-BODY-TIMING-AND-EXACT-C-PAIR-CONTRACT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-170-POST-ROLLBACK-GAP-TAXONOMY-REFRESH.md
  - tools/allocator/hako_mimalloc_object_lifecycle_body_timing_and_exact_c_pair_contract.py
---

# 296x-171 Object Lifecycle Body Timing And Exact C Pair Contract

## Purpose

Stop source-level and MIR-builder optimization until the row can compare the
same object-lifecycle workload through both `.hako` exact-EXE and C mimalloc
explicit runners with body timing available on both sides.

This row is a measurement contract row only. It does not implement the C runner,
change the `.hako` proof app, or reopen provider activation, replacement,
hooks, global allocator selection, or winner claims.

## Required Output

```text
output_contract=hako-mimalloc-object-lifecycle-body-timing-and-exact-c-pair-contract-v0
exact_c_pair_required=1
hako_body_elapsed_ns_required=1
c_body_elapsed_ns_required=1
body_elapsed_comparable_required=1
next_optimization_allowed=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-object-lifecycle-body-timing-and-exact-c-pair-contract-v0
input_contract=hako-mimalloc-post-rollback-gap-taxonomy-refresh-v0
workload_id=representative-object-lifecycle-small-block-v0
operation_family=small-block
operation_sequence_id=representative-object-lifecycle-small-block-v0-seq
free_order_id=even-odd-release-v0
required_hako_subject=hako_exact_exe_object_lifecycle
required_c_subject=c_mimalloc_explicit_object_lifecycle
required_in_process_operation_repeat=8192
required_allocation_count=524288
required_free_count=524288
required_requested_bytes=272416768
hako_body_elapsed_ns_required=1
c_body_elapsed_ns_required=1
body_elapsed_comparable_required=1
body_elapsed_role=primary_hot_loop_diagnostic
external_elapsed_role=secondary_process_runtime_evidence
exact_c_pair_required=1
exact_c_pair_status=missing
hako_body_timing_status=missing
measurement_contract_gap_open=1
next_diagnostic=object_lifecycle_exact_c_runner_first_pattern
next_optimization_allowed=0
optimization_started=0
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Interpretation:

```text
Row170 selected measurement_contract_gap because external elapsed and MIR shape
alone were not enough to choose the next keeper. Row171 fixes the acceptance
surface: a future optimization row must have an exact C object-lifecycle subject
and comparable body_elapsed_ns fields, with external elapsed demoted to
secondary process/runtime evidence.
```

## Next

```text
row172:
  object_lifecycle_exact_c_runner_first_pattern

Goal:
  add the missing C mimalloc explicit object-lifecycle workload first, matching
  operation_sequence_id, free_order_id, allocation_count, free_count, and
  requested_bytes before changing the .hako timing app.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_object_lifecycle_body_timing_and_exact_c_pair_contract_guard.sh
```
