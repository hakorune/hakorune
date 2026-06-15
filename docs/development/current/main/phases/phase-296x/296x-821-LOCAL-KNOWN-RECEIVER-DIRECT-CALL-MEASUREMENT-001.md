---
Status: Landed
Date: 2026-06-16
Task: LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001
Scope: Measure the current object-lifecycle body after the local known-receiver
  direct-call pilot reached the existing generic RoutePlan backend seam.
Related:
  - docs/development/current/main/phases/phase-296x/296x-820-LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001.md
  - tools/allocator/hako_local_known_receiver_direct_call_measurement.py
---

# LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001

## Purpose

`LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001` did not add new lowering code.  It
proved that the current measured route already has the generic RoutePlan
consumer needed for local known-receiver direct calls.  This row measures the
current front before selecting any further implementation owner.

## Measurement

Command:

```bash
bash tools/allocator/hako_mimalloc_direct_exact_pair.sh \
  --out target/phase296x-local-known-receiver-direct-call/pair_65536.out \
  --in-process-repeat 65536
```

Secondary granularity sample:

```bash
bash tools/allocator/hako_mimalloc_direct_exact_pair.sh \
  --out target/phase296x-local-known-receiver-direct-call/pair.out \
  --in-process-repeat 8192
```

## Result

```text
output_contract=hako-local-known-receiver-direct-call-measurement-v0
source_evidence=296x-820
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
selected_shape=local_known_receiver_direct_call
pilot_status=already_satisfied_existing_generic_route
primary_in_process_repeat=65536
hako_body_elapsed_ns=24000000
c_body_elapsed_ns=28710275
body_elapsed_gap_ns=-4710275
body_elapsed_ratio=0.836
hako_not_slower_than_c=1
measurement_interpretation=current_front_no_longer_hako_slower
new_backend_lowering_code_added=0
page_specific_rule_enabled=0
method_name_special_case_enabled=0
helper_symbol_inference_enabled=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
winner_claim=1
selected_next=LOCAL-KNOWN-RECEIVER-DIRECT-CALL-CLOSEOUT-001
secondary_in_process_repeat=8192
secondary_hako_body_elapsed_ns=3000000
secondary_c_body_elapsed_ns=3761888
secondary_body_elapsed_ratio=0.797
summary=ok
```

## Interpretation

The current direct-exact object-lifecycle front is no longer Hako-slower on the
body timing pair.  This is not a claim that this row added a new speedup; the
pilot classified an existing generic RoutePlan direct-call seam as already
satisfied.  The next row should close this local known-receiver direct-call
lane before selecting any new owner.

## Stop Line

```text
do not add lowering after the measurement
do not attribute this result to a new code change
do not reopen page/method/helper-specific branches
do not open storage direct lowering
do not bypass HostHandle
do not retire Arc
do not change product default runtime behavior
```

## Proof

```bash
python3 -m py_compile tools/allocator/hako_local_known_receiver_direct_call_measurement.py
bash tools/checks/k2_wide_phase296x_local_known_receiver_direct_call_measurement_guard.sh
```
