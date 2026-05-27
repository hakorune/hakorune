---
Status: Landed
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

## Landed Evidence

```text
output_contract=hako-mimalloc-single-pred-phi-elision-guard-surface-v0
input_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0
selected_owner_file=src/mir/builder/emission/phi.rs
selected_owner_module=crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry
guard_surface=single_pred_phi_elision
required_before_metric=single_incoming_phi_count
required_before_value=61
required_after_metric=single_incoming_phi_count
required_after_max=15
semantic_guard=current_state_pointer_guard
shape_guard=small_alloc_phi_copy_lowering_probe
perf_guard=object_lifecycle_exact_exe_measurement
implementation_gate=cargo_build_release_hakorune
next_action=implement_guarded_elision
next_row=HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-IMPLEMENTATION-296X-001
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_single_pred_phi_elision_guard_surface_guard.sh
```
