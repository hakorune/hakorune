---
Status: Current
Date: 2026-05-28
Scope: measure object-lifecycle facade exact-EXE after static-scalar call lowering.
Blocker: POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-133-HAKO-MIMALLOC-POST-SINGLE-EVAL-FIXES-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION.md
---

# 296x-139 Post Static Scalar Call Lowering Measurement

## Purpose

Rerun the object-lifecycle facade exact-EXE measurement after verified
static-scalar reason calls lower to constants.

## Required Output

```text
output_contract=post-static-scalar-call-lowering-measurement-v0
input_contract=static-scalar-call-lowering-implementation-v0
elapsed_median_ms
previous_checkpoint_hako_elapsed_median_ms
static_scalar_lowering_effect
winner_claim=0
summary=ok
```
