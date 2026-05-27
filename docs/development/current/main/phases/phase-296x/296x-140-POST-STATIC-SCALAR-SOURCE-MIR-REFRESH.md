---
Status: Current
Date: 2026-05-28
Scope: refresh source/MIR observation after static-scalar call lowering measurement.
Blocker: POST-STATIC-SCALAR-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-138-STATIC-SCALAR-CALL-LOWERING-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-139-POST-STATIC-SCALAR-CALL-LOWERING-MEASUREMENT.md
---

# 296x-140 Post Static Scalar Source/MIR Refresh

## Purpose

Refresh source/MIR observation after static-scalar call lowering before choosing
another optimization keeper.

## Required Output

```text
output_contract=post-static-scalar-source-mir-refresh-v0
input_contract=post-static-scalar-call-lowering-measurement-v0
selected_method
remaining_call_surface
selected_next
summary=ok
```
