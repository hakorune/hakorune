# Hako Alloc Allocator Comparison C Mimalloc Explicit Runner Closeout SSOT

Status: accepted
Decision: accepted
Date: 2026-05-21
Owner: MIMAP-453A

## Decision: accepted

MIMAP-453A closes the explicit C mimalloc runner execution/evidence diagnostics
pack by reusing the already-defined MIMAP-451A and MIMAP-452A guards.

It does not add a new `.hako` owner. The purpose is to freeze the execution
tool contract, evidence ledger, and diagnostics vocabulary before a comparison
result ledger row is opened.

## Pack

```text
MIMAP-451A:
  explicit C mimalloc runner execution pilot

MIMAP-452A:
  explicit C mimalloc runner evidence diagnostics
```

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env or runtime discovery of mimalloc behavior.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI.
- No runtime sum materialization.

## Validation

Validation profile: closeout over MIMAP-451A / MIMAP-452A L2 guards.

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_execution_pilot_guard.sh --level L2
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh --level L2
```
