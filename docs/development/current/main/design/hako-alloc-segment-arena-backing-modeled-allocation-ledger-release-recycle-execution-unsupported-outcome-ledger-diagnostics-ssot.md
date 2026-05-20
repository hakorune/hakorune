# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Unsupported Outcome Ledger Diagnostics SSOT

Decision: accepted

Status: active for MIMAP-321A

## Purpose

MIMAP-321A adds observer-only diagnostics for the MIMAP-320A model-only
unsupported release/recycle execution outcome ledger. The owner reads outcome
ledger facts and publishes scalar diagnostic counters before any closeout pack
or real execution row opens.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_diagnostic_box.hako
```

The owner may:

- observe one MIMAP-320A unsupported outcome ledger/report pair;
- copy outcome facts into a diagnostic report;
- count accepted outcome, rejected outcome, unsupported outcome, missing
  ledger, and missing report cases;
- mirror exact `usize` byte facts from the outcome report.

The owner must not:

- record unsupported outcome rows;
- execute release/recycle behavior;
- create lifecycle generation tokens;
- open pointer residence or pointer-derived lookup;
- release or recycle real arena backing;
- mutate segment-map state;
- execute atomic bitmap, OSVM/page-source, worker, provider, hook, or backend
  matcher behavior.

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_unsupported_outcome_ledger_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-321A
```

L3/L4 evidence is deferred to a future closeout pack.
