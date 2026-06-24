---
Status: Landed
Date: 2026-05-28
Scope: classify the selected field/Array runtime lowering boundary before choosing a keeper.
Blocker: FIELD-ARRAY-RUNTIME-LOWERING-BOUNDARY-PROBE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-186-OBJECT-LIFECYCLE-LARGE-OWNER-REALITY-CHECK.md
---

# 296x-187 Field/Array Runtime Lowering Boundary Probe

## Purpose

Classify the exact lowering boundary behind the row186 perf hot symbols. This
row keeps optimization closed and chooses the next single keeper family.

## Evidence

Probe:

```bash
tools/allocator/hako_mimalloc_field_array_runtime_boundary_probe.py \
  --mir-json /tmp/hako_row186_mir.json \
  --perf-report /tmp/hako_row186_perf_report.txt
```

Report summary:

```text
output_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0
field_static_total=80
field_dynamic_estimate=30072832
array_method_static_total=7
array_method_dynamic_estimate=2121728
perf_field_helper_pct=72.96
perf_array_helper_pct=26.19
selected_boundary=typed_object_field_helper_lowering
secondary_boundary=array_runtime_slot_helper_lowering
next_diagnostic=typed_object_field_helper_fast_lane_selection
summary=ok
```

Top symbols:

```text
23.86% nyash.object.field_set_hii
18.64% nyash.object.field_get_hii
17.19% nyash.object.field_get_u64_hii
16.45% array_runtime_set_idx_i64
13.27% nyash.object.field_set_u64_hiu
```

## Selection

```text
selected_boundary=typed_object_field_helper_lowering
selected_reason=field helpers take 72.96% of exact-EXE samples
secondary_boundary=array_runtime_slot_helper_lowering
secondary_reason=array helpers take 26.19% of exact-EXE samples
rejected_boundary=remaining_copy_cleanup
rejected_reason=copy reductions did not close the Hako/C body gap
confidence=high
```

## Next Row

```text
TYPED-OBJECT-FIELD-HELPER-FAST-LANE-SELECTION-296X-001
```

The next row should choose one narrow typed-object field helper lane. Candidate
families:

```text
1. numeric i64/u64 typed-object field helper fast lane
2. typed-object hot handle cache / lock reduction
3. MIR-side scalar field residence for proven non-escaping hako_alloc objects
```

Do not mix these families in one row.

## Non-Goals

```text
- Do not edit compiler/runtime lowering in this probe row.
- Do not optimize ArrayBox and typed-object helpers in the same keeper row.
- Do not add generic CSE, generic copy coalescing, or by-name special cases.
- Do not open provider activation, allocator replacement, hooks, globals, or
  winner claims.
```

## Acceptance

```text
selected_boundary=typed_object_field_helper_lowering
next_diagnostic=typed_object_field_helper_fast_lane_selection
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```
