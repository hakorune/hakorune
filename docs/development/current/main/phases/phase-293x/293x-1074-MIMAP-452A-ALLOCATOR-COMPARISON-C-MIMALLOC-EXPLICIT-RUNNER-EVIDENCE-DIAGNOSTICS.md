# 293x-1074 MIMAP-452A Allocator Comparison C Mimalloc Explicit Runner Evidence Diagnostics

Status: landed
Date: 2026-05-21

## Purpose

Add observer diagnostics for the MIMAP-451A explicit C mimalloc runner evidence
report before any repeated benchmark or comparison result ledger is opened.

## Scope

- Consume the MIMAP-451A explicit runner execution pilot report.
- Classify missing runner invocation, missing output, missing memory-use
  evidence, missing stable output contract, non-zero runner result, and invalid
  run-count evidence.
- Keep C mimalloc execution as an explicit tool boundary, not Hakorune runtime
  behavior.
- Keep provider package / DLL generation parked.

## Design

SSOT:

```text
docs/development/current/main/design/hako-alloc-allocator-comparison-c-mimalloc-explicit-runner-evidence-diagnostics-ssot.md
```

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Planned validation profile: `scalar-mir`.

MIMAP-452A should reuse MIMAP-451A evidence shape, add diagnostics in `.hako`
model space, and defer any repeated C mimalloc benchmark pack to a later
closeout row.

Required guard:

```bash
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostics_guard.sh --level L2
```

## Completed

- Added the MIMAP-452A evidence diagnostic SSOT.
- Added the `.hako` diagnostic owner:
  `allocator_comparison_c_mimalloc_explicit_runner_evidence_diagnostic_box.hako`.
- Added the proof app and manifest row for MIMAP-452A.
- Kept C runner re-execution, process allocator replacement, hooks, backend
  matcher additions, `#[global_allocator]`, provider package / DLL generation,
  hidden runtime discovery, and worker/thread execution closed.

## Next

MIMAP-453A should close out the explicit runner execution/evidence diagnostics
pack and decide the next comparison-result ledger row.
