---
Status: Landed
Date: 2026-05-28
Scope: select the first lowering row for verified static scalar method facts.
Blocker: STATIC-SCALAR-CALL-LOWERING-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-135-STATIC-SCALAR-METHOD-FACT-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-136-STATIC-SCALAR-METHOD-FACT-INFERENCE.md
---

# 296x-137 Static Scalar Call Lowering Selection

## Purpose

Select how verified static-scalar facts may be consumed by call lowering. This
row must choose the exact call route and guard surface before replacing any call
with a constant.

## Required Output

```text
output_contract=static-scalar-call-lowering-selection-v0
input_contract=static-scalar-method-fact-inference-v0
lowering_route
guard_surface
selected_next
summary=ok
```

## Stop Line

Do not lower calls to constants in this row.

## Evidence

Report:

```text
output_contract=static-scalar-call-lowering-selection-v0
input_contract=static-scalar-method-fact-inference-v0
lowering_route=handle_static_method_call_zero_arg_before_emit_unified_call
guard_surface=object_lifecycle_reason_static_receiver_zero_arg
required_fact=verified_static_scalar_method_fact
arg_policy=zero_args_only
generic_cse=0
whole_box_pure=0
fallback_on_missing_fact=keep_call
selected_next=static_scalar_call_lowering_implementation
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_static_scalar_call_lowering_selection_guard.sh
```
