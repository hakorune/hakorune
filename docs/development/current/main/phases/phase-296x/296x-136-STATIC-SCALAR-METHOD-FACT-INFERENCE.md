---
Status: Landed
Date: 2026-05-28
Scope: infer static scalar method facts for the selected reason getter family without lowering calls.
Blocker: STATIC-SCALAR-METHOD-FACT-INFERENCE-296X-001
Related:
  - docs/development/current/main/design/nested-argument-single-evaluation-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-135-STATIC-SCALAR-METHOD-FACT-SELECTION.md
---

# 296x-136 Static Scalar Method Fact Inference

## Purpose

Add the first compiler-side fact surface for selected static scalar reason
getters. This row should record verified facts only; unsupported shapes must
keep the ordinary call path.

## Required Output

```text
output_contract=static-scalar-method-fact-inference-v0
input_contract=static-scalar-method-fact-selection-v0
fact_family
candidate_count
verified_fact_count
unverified_count
selected_next
summary=ok
```

## Stop Line

Do not lower calls to constants in this row.

## Evidence

Report:

```text
output_contract=static-scalar-method-fact-inference-v0
input_contract=static-scalar-method-fact-selection-v0
fact_family=object_lifecycle_facade_reason_zero_arg_return_literal_i64
candidate_count=19
verified_fact_count=19
unverified_count=0
proof=zero_arg_return_literal_only
generic_cse=0
whole_box_pure=0
const_lowering=0
failure_mode=keep_call
selected_next=static_scalar_call_lowering_selection
summary=ok
```

Rust surface:

```text
src/mir/builder/static_scalar_facts.rs
src/mir/builder/compilation_context.rs
src/mir/builder/declaration_indexer.rs
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_static_scalar_method_fact_inference_guard.sh
```
