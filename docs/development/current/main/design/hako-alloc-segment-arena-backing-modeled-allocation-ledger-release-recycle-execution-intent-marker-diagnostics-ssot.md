# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Intent Marker Diagnostics SSOT

Decision: accepted

Status: active for MIMAP-317A

## Purpose

MIMAP-317A adds observer-only diagnostics for the MIMAP-316A model-only
release/recycle execution intent marker. The owner reads intent marker facts
and publishes scalar diagnostic counters before any closeout pack or real
execution row opens.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_diagnostic_box.hako
```

The owner may:

- observe one MIMAP-316A execution intent marker inventory/report pair;
- copy marker facts into a diagnostic report;
- count accepted marker, rejected marker, unsupported execution, missing
  inventory, and missing marker cases;
- mirror exact `usize` byte facts from the marker report.

The owner must not:

- record execution intent marker rows;
- execute release/recycle behavior;
- create lifecycle generation tokens;
- open pointer residence or pointer-derived lookup;
- release or recycle real arena backing;
- mutate segment-map state;
- execute atomic bitmap, OSVM/page-source, worker, provider, hook, or backend
  matcher behavior.

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-317A
```

L3/L4 evidence is deferred to a future closeout pack.
