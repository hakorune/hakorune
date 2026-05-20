# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Applied-State Summary Diagnostics SSOT

Decision: accepted

Status: active for MIMAP-309A

## Purpose

MIMAP-309A adds an observer-only diagnostic owner for the MIMAP-308A
release/recycle applied-state summary. The owner reads scalar/model summary
facts and publishes diagnostic counters/reports before the closeout pack opens.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_diagnostic_box.hako
```

The owner may:

- observe one MIMAP-308A applied-state summary inventory/report pair;
- copy summary facts into a diagnostic report;
- count accepted summaries, rejected summaries, missing inventory, missing
  summary, rejected summary, and invalid token cases;
- mirror usize byte facts as exact `usize` fields.

The owner must not:

- record new applied-state summary rows;
- create real lifecycle generation tokens;
- create raw pointer residence or pointer-derived lookups;
- release or recycle real arena backing;
- mutate segment-map state;
- execute atomic bitmap, OSVM/page-source, worker, provider, hook, or backend
  matcher behavior.

## Validation

Daily validation is L2:

```text
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_applied_state_summary_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-309A
```

L3/L4 evidence is deferred to the MIMAP-310A closeout pack.

## Stop Lines

- No new summary row.
- No real lifecycle generation.
- No raw pointer residence or pointer-derived lookup.
- No real arena backing allocation, release, or recycle.
- No real segment-map mutation.
- No segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.
