---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-360A
Scope: provider boundary diagnostic vocabulary inventory with activation closed.
Related:
  - docs/development/current/main/phases/phase-293x/293x-976-MIMAP-360A-PROVIDER-BOUNDARY-DIAGNOSTIC-VOCABULARY.md
  - lang/src/hako_alloc/memory/provider_boundary_diagnostic_vocabulary_box.hako
  - apps/hako-alloc-provider-boundary-diagnostic-vocabulary-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_provider_boundary_diagnostic_vocabulary_guard.sh
---

# Hako Alloc Provider Boundary Diagnostic Vocabulary

## Decision

MIMAP-360A inventories the provider boundary diagnostic reason vocabulary
before provider readiness, provider selection, or provider activation rows are
opened.

This row consumes `HakoAllocProviderInactiveBoundaryInventoryReport` and
publishes scalar reason codes for provider-facing diagnostics. It does not
activate, select, or call a provider.

## Reasons

```text
0 = accepted
1 = missing provider boundary
2 = rejected provider boundary
3 = provider activation request
4 = host allocator replacement request
5 = hook/global allocator request
6 = backend matcher request
7 = worker/thread request
```

## Stop Lines

- No provider activation.
- No host allocator replacement.
- No hooks or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

```bash
bash tools/checks/k2_wide_hako_alloc_provider_boundary_diagnostic_vocabulary_guard.sh --level L2
```
