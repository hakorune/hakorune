---
Status: Landed
Date: 2026-05-28
Scope: close out post-BoxShape correctness and return to keeper selection.
Blocker: MIR-BUILDER-POST-BOXSHAPE-CORRECTNESS-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-143-MIR-BUILDER-MEMBER-CALL-ROUTE-PLAN-PILOT.md
  - docs/development/current/main/phases/phase-296x/296x-144-MIR-BUILDER-FIELD-PROPERTY-RECEIVER-FACTS-CLEANUP.md
---

# 296x-145 MIR Builder Post BoxShape Correctness Closeout

## Purpose

Rerun the correctness and observation surfaces after the member-call and
field/property BoxShape cleanup before returning to mimalloc keeper selection.

## Required Output

```text
output_contract=mir-builder-post-boxshape-correctness-closeout-v0
input_contract=mir-builder-field-property-receiver-facts-cleanup-v0
build_ok
single_eval_surface_ok
small_alloc_helper_copy_probe_ok
post_boxshape_next
summary=ok
```

## Evidence

```text
output_contract=mir-builder-post-boxshape-correctness-closeout-v0
input_contract=mir-builder-field-property-receiver-facts-cleanup-v0
build_ok=1
single_eval_surface_ok=1
small_alloc_helper_copy_probe_ok=1
helper_call_count=16
helper_copy_count=62
receiver_copy_count=38
arg_copy_count=15
result_copy_count=9
local_ssa_copy_count=44
dominant_callee_family=facade_result_helpers
helper_copy_post_boxshape_status=unchanged
generic_cse_opened=0
post_boxshape_next=page_array_dynamic_weight_probe
winner_claim=0
replacement_active=0
summary=ok
```

Interpretation:

```text
The BoxShape cleanup preserved correctness but did not change the helper-copy
shape. Return to keeper selection via dynamic page-array weight measurement
before choosing compiler helper lowering or .hako page-model work.
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_mir_builder_post_boxshape_correctness_closeout_guard.sh
```
