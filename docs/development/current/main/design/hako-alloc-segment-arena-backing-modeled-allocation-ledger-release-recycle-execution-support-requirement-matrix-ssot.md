# Hako Alloc Segment Arena Backing Modeled Allocation-Ledger Release/Recycle Execution Support Requirement Matrix SSOT

Status: active
Decision: accepted
Date: 2026-05-20
Rows: MIMAP-328A

## Purpose

MIMAP-328A records the model-only requirements that must be satisfied before
real release/recycle execution support can open. It consumes a closed
MIMAP-324A support gate report and records the blockers as explicit scalar
matrix facts.

This SSOT does not satisfy any requirement and does not open real
release/recycle execution.

## Owner

```text
lang/src/hako_alloc/memory/segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_box.hako
```

## Requirements

The matrix records these currently unsatisfied requirements:

```text
lifecycle generation
pointer residence
pointer lookup
arena backing release
arena backing recycle
segment-map mutation
atomic bitmap
OSVM/page-source
worker/TLS substrate
provider activation
backend matcher / lowering support
```

`requirements_satisfied` must remain `0` in this row.

## Accepted Behavior

- Accept only reports where `gate_present == 1`, `accepted == 1`, and
  `execution_gate_open == 0`.
- Record one matrix row with all requirement flags set to `1`.
- Publish scalar counters for accepted rows and reject reasons.
- Mirror byte facts as `usize` report fields.
- Keep all would-execute flags at `0`.

Reject reasons:

```text
1: missing support gate report
2: rejected support gate report
3: support gate already open
```

## Stop Lines

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
bash tools/checks/k2_wide_hako_alloc_segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_requirement_matrix_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-328A
```

L3 EXE evidence is deferred to a future closeout pack.
