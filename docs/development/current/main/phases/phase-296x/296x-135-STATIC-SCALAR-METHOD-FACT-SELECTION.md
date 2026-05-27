---
Status: Landed
Date: 2026-05-28
Scope: select the first static scalar method fact boundary after single-eval correctness fixes.
Blocker: STATIC-SCALAR-METHOD-FACT-SELECTION-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-134-MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP.md
---

# 296x-135 Static Scalar Method Fact Selection

## Purpose

Select the first narrow static-scalar method fact boundary. This is not generic
CSE. The candidate must be verified by method body shape, such as same-module
zero-arg static method returning a literal scalar with no calls, fields,
allocation, branches, or safepoints.

## Required Output

```text
output_contract=static-scalar-method-fact-selection-v0
input_contract=mir-builder-single-eval-surface-sweep-v0
candidate_family
selection
selected_next
summary=ok
```

## Stop Line

Do not lower calls to constants in this row.

## Evidence

Report:

```text
output_contract=static-scalar-method-fact-selection-v0
input_contract=mir-builder-single-eval-surface-sweep-v0
candidate_family=object_lifecycle_facade_reason_zero_arg_return_literal_i64
selection=verified_static_method_return_literal_shape
scope=same_source_static_box_only
generic_cse=0
whole_box_pure=0
const_lowering=0
failure_mode=keep_call
candidate_count=19
selected_next=static_scalar_method_fact_inference
summary=ok
```

Boundary:

```text
Do not mark the whole reason box pure.
Do not add generic MIR CSE.
Do not lower verified calls to constants until the next lowering row.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_static_scalar_method_fact_selection_guard.sh
```
