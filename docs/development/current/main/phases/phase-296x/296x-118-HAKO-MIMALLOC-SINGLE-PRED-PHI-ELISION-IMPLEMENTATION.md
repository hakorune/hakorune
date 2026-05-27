---
Status: Current
Date: 2026-05-27
Scope: implement guarded single-pred PHI elision in the selected MIR builder owner.
Blocker: HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-117-HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE.md
---

# 296x-118 Hako Mimalloc Single-Pred PHI Elision Implementation

## Purpose

Implement the smallest guarded MIR builder change for
`materialize_vars_single_pred_at_entry`, then verify object-lifecycle exact-EXE
shape and measurement.

## Required Output

```text
output_contract=hako-mimalloc-single-pred-phi-elision-implementation-v0
input_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0
selected_owner_file=src/mir/builder/emission/phi.rs
single_pred_phi_elision_enabled=1
before_single_incoming_phi_count=61
after_single_incoming_phi_count
semantic_summary=ok
measurement_summary=ok
summary=ok
```

## Stop Line

Do not combine this with unrelated `.hako` keeper work.
