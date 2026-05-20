# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Support Requirement Matrix Diagnostics SSOT

Status: active
Decision: accepted
Date: 2026-05-20
Rows: MIMAP-329A

## Purpose

MIMAP-329A adds observer-only diagnostics for the MIMAP-328A model-only
release/recycle execution support requirement matrix. It reports the
unsatisfied requirements without satisfying them.

This SSOT does not open real release/recycle execution.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostic_box.hako
```

The owner may observe one requirement matrix inventory and one requirement
matrix report. It must not record new requirement matrix rows.

## Accepted Behavior

- Accept only reports where the inventory has at least one matrix row, the
  report has `matrix_present == 1`, and the report is accepted.
- Count accepted, rejected, and unsatisfied matrix observations.
- Publish missing-inventory and missing-report reject counters.
- Mirror byte facts as `usize` report fields.
- Copy would-execute flags from the observed matrix report; they must remain
  `0` for MIMAP-328A inputs.

Reject reasons:

```text
1: missing requirement matrix inventory row
2: missing requirement matrix report
3: rejected requirement matrix report
```

## Stop Lines

- No new requirement matrix row recording from the diagnostic owner.
- No real release/recycle execution.
- No real lifecycle generation token.
- No raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.

## Validation

Daily validation is L2:

```bash
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-329A
```

L3 EXE evidence is deferred to a future closeout pack.
