---
Status: Current
Date: 2026-05-28
Scope: add a broader MIR builder single-evaluation surface sweep.
Blocker: MIR-BUILDER-SINGLE-EVAL-SURFACE-SWEEP-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-128-MIR-BUILDER-NESTED-ARGUMENT-SINGLE-EVAL-OWNER-FIX.md
  - docs/development/current/main/phases/phase-296x/296x-130-MIR-BUILDER-NESTED-FIELD-SINGLE-EVAL-OWNER-FIX.md
  - docs/development/current/main/phases/phase-296x/296x-132-MIR-BUILDER-ENV-METHOD-SINGLE-EVAL-OWNER-FIX.md
---

# 296x-134 MIR Builder Single Eval Surface Sweep

## Purpose

Add a broader fixture sweep over MIR builder surfaces that can accidentally
re-lower expressions while probing:

```text
field assignment
index read/write
print fallback
typeop method
constructor args / field init
```

This row is observation/guard only unless a new duplicate-eval bug is found.

## Required Output

```text
output_contract=mir-builder-single-eval-surface-sweep-v0
input_contract=hako-mimalloc-post-single-eval-fixes-measurement-v0
surface_count
failing_surface_count
selected_next
summary=ok
```

## Stop Line

Do not implement new MIR builder fixes in this row unless the sweep finds a
specific failing surface and opens a dedicated owner-fix row.
