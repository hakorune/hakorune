# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Readiness Matrix Diagnostics SSOT

Decision: accepted

Status: active for MIMAP-313A

## Purpose

MIMAP-313A adds observer-only diagnostics for the MIMAP-312A release/recycle
execution readiness matrix. The owner reads model-only matrix facts and
publishes accepted/rejected/blocked diagnostic counters before any closeout
pack or real execution row opens.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostic_box.hako
```

The owner may:

- observe one MIMAP-312A matrix inventory/report pair;
- copy matrix facts into a diagnostic report;
- count accepted matrix, rejected matrix, blocked matrix, missing inventory,
  and missing matrix cases;
- mirror exact `usize` byte facts from the matrix report.

The owner must not:

- record execution readiness matrix rows;
- create lifecycle generation tokens;
- open pointer residence or pointer-derived lookup;
- release or recycle real arena backing;
- mutate segment-map state;
- execute atomic bitmap, OSVM/page-source, worker, provider, hook, or backend
  matcher behavior.

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-313A
```

L3/L4 evidence is deferred to the MIMAP-314A closeout pack.
