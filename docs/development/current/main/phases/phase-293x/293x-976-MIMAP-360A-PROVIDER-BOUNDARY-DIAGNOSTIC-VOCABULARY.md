# 293x-976 MIMAP-360A Provider Boundary Diagnostic Vocabulary

Status: landed
Date: 2026-05-21

## Decision

Inventory provider boundary diagnostic reason vocabulary before provider
readiness, provider selection, or activation rows are opened.

## Scope

- Add `HakoAllocProviderBoundaryDiagnosticVocabulary`.
- Add a manifest-backed proof app.
- Add an L2 guard for static drift, VM proof output, MIR JSON shape, and route
  preflight.
- Keep provider activation and all host-facing replacement/hook behavior
  closed.

## Stop Lines

- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_boundary_diagnostic_vocabulary_guard.sh --level L2
bash tools/checks/run_proof_app.sh --only MIMAP-360A --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Landed. MIMAP-361A is selected as the next row-selection card.
