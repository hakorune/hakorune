# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Support Gate Diagnostics SSOT

Status: active
Decision: accepted
Date: 2026-05-20
Rows: MIMAP-325A

## Purpose

MIMAP-325A adds observer-only diagnostics for the MIMAP-324A model-only
release/recycle execution support gate. The diagnostic owner explains whether
the gate was present, accepted, closed, and blocked by unsupported outcome
facts.

This SSOT does not open real release/recycle execution.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostic_box.hako
```

The owner may observe one support gate inventory and one support gate report.
It must not record new support gate rows.

## Accepted Behavior

- Accept only reports where the inventory has at least one gate row, the report
  has `gate_present == 1`, and the report is accepted.
- Count accepted, rejected, closed, and blocked support gate observations.
- Publish missing-inventory and missing-report reject counters.
- Mirror byte facts as `usize` report fields.
- Copy would-execute flags from the observed gate report; they must remain `0`
  for MIMAP-324A inputs.

Reject reasons:

```text
1: missing support gate inventory row
2: missing support gate report
3: rejected support gate report
```

## Stop Lines

- No new support gate row recording from the diagnostic owner.
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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_diagnostics_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-325A
```

L3 EXE evidence is deferred to a future closeout pack.
