---
Status: Landed
Date: 2026-05-28
Scope: lower verified zero-arg static-scalar calls to constants through the selected route.
Blocker: STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-136-STATIC-SCALAR-METHOD-FACT-INFERENCE.md
  - docs/development/current/main/phases/phase-296x/296x-137-STATIC-SCALAR-CALL-LOWERING-SELECTION.md
---

# 296x-138 Static Scalar Call Lowering Implementation

## Purpose

Consume verified static-scalar facts in the selected zero-arg static receiver
route, replacing those calls with constants while preserving ordinary calls for
missing or unverified facts.

## Required Output

```text
output_contract=static-scalar-call-lowering-implementation-v0
input_contract=static-scalar-call-lowering-selection-v0
lowered_static_scalar_const_count
remaining_reason_call_count
missing_fact_keep_call_count
summary=ok
```

## Stop Line

Do not add generic MIR CSE or whole-box pure markers in this row.

## Evidence

Report:

```text
output_contract=static-scalar-call-lowering-implementation-v0
input_contract=static-scalar-call-lowering-selection-v0
source_reason_call_count=25
lowered_static_scalar_const_count=25
remaining_reason_call_count=0
missing_fact_keep_call_count=0
generic_cse=0
whole_box_pure=0
selected_next=post_static_scalar_call_lowering_measurement
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_static_scalar_call_lowering_implementation_guard.sh
```
