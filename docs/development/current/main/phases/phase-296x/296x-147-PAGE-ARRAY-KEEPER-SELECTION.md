---
Status: Current
Date: 2026-05-28
Scope: select the next page-array keeper from dynamic weight evidence.
Blocker: PAGE-ARRAY-KEEPER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-146-PAGE-ARRAY-DYNAMIC-WEIGHT-PROBE.md
---

# 296x-147 Page Array Keeper Selection

## Purpose

Select exactly one next keeper from page-local ArrayBox dynamic weight evidence.
Do not mix compiler helper-call lowering into the same row; it remains a
secondary owner after page-array keeper selection.

## Required Output

```text
output_contract=page-array-keeper-selection-v0
input_contract=page-array-dynamic-weight-probe-v0
selected_keeper
keeper_owner
expected_dynamic_weight_reduction
fallback_preservation
summary=ok
```
