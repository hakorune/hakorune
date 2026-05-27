---
Status: Current
Date: 2026-05-27
Scope: define the guard surface for single-pred PHI elision before implementation.
Blocker: HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-116-HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION.md
---

# 296x-117 Hako Mimalloc Single-Pred PHI Elision Guard Surface

## Purpose

Before changing `materialize_vars_single_pred_at_entry`, define the guard
surface and acceptance report for single-pred PHI elision.

This row is docs/tooling guard surface only.

## Required Output

```text
output_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0
input_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0
selected_owner_file=src/mir/builder/emission/phi.rs
guard_surface=single_pred_phi_elision
required_before_metric=single_incoming_phi_count
required_after_metric=single_incoming_phi_count
semantic_guard=current_state_pointer_guard
perf_guard=object_lifecycle_exact_exe_measurement
summary=ok
```

## Stop Line

Do not implement elision in this row.
