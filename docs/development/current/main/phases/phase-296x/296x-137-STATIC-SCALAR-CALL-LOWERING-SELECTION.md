---
Status: Current
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
