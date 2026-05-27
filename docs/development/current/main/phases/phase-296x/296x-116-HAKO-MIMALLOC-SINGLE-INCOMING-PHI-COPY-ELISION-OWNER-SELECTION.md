---
Status: Landed
Date: 2026-05-27
Scope: select the MIR builder owner for single-incoming phi/copy elision.
Blocker: HAKO-MIMALLOC-SINGLE-INCOMING-PHI-COPY-ELISION-OWNER-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-115-HAKO-MIMALLOC-SMALL-ALLOC-PHI-COPY-LOWERING-PROBE.md
---

# 296x-116 Hako Mimalloc Single-Incoming Phi/Copy Elision Owner Selection

## Purpose

Row115 classified the dominant `objectLifecycleSmallAlloc` lowered shape as
`local_copy_churn`, with 61 single-incoming phi nodes and 94 copies.

This row should select the smallest MIR-builder owner to inspect or change.
Do not edit lowering yet.

## Required Output

```text
output_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0
input_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0
selected_owner_file
selected_owner_module
candidate_change_kind=analysis_only|mirbuilder_elision|hako_shape|unknown
next_action=probe_owner|implement_guarded_elision|stop_line
summary=ok
```

## Stop Line

No MIR builder implementation in this row. Select owner and guard surface only.

## Landed Evidence

```text
output_contract=hako-mimalloc-single-incoming-phi-copy-elision-owner-selection-v0
input_contract=hako-mimalloc-small-alloc-phi-copy-lowering-probe-v0
selected_owner_file=src/mir/builder/emission/phi.rs
selected_owner_module=crate::mir::builder::emission::phi::materialize_vars_single_pred_at_entry
supporting_phi_helper_file=src/mir/utils/phi_helpers.rs
supporting_phi_helper=MirBuilder::insert_phi_single
supporting_copy_owner_file=src/mir/builder/ssa/local.rs
supporting_copy_owner=crate::mir::builder::ssa::local::ensure
candidate_change_kind=mirbuilder_elision
next_action=probe_owner
next_diagnostic=single_pred_phi_elision_guard_surface
next_row=HAKO-MIMALLOC-SINGLE-PRED-PHI-ELISION-GUARD-SURFACE-296X-001
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Guard:

```bash
bash tools/checks/k2_wide_phase296x_hako_mimalloc_single_incoming_phi_copy_elision_owner_selection_guard.sh
```
