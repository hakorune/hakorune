# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Intent Marker SSOT

Decision: accepted

Status: active for MIMAP-316A

## Purpose

MIMAP-316A adds an explicit model-only release/recycle execution intent marker
after the execution readiness matrix closeout. Intent is accepted only from
accepted matrix evidence. Real release/recycle execution remains unsupported.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_box.hako
```

The owner may:

- observe one accepted MIMAP-312A execution readiness matrix report;
- record explicit model-only release/recycle execution intent;
- publish scalar intent facts and exact `usize` byte facts;
- publish unsupported-execution facts for later rows.

The owner must not:

- execute release/recycle behavior;
- create lifecycle generation tokens;
- open pointer residence or pointer-derived lookup;
- release or recycle real arena backing;
- mutate segment-map state;
- execute atomic bitmap, OSVM/page-source, worker, provider, hook, or backend
  matcher behavior.

## Validation

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_intent_marker_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-316A
```

L3/L4 evidence is deferred to a future closeout pack.
