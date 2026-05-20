---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-362A
Scope: provider readiness preflight with provider activation closed.
Related:
  - docs/development/current/main/phases/phase-293x/293x-978-MIMAP-362A-PROVIDER-READINESS-PREFLIGHT.md
  - lang/src/hako_alloc/memory/provider_readiness_preflight_box.hako
  - apps/hako-alloc-provider-readiness-preflight-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_provider_readiness_preflight_guard.sh
---

# Hako Alloc Provider Readiness Preflight

## Decision

MIMAP-362A preflights provider readiness after provider boundary diagnostic
vocabulary inventory while keeping provider activation closed.

This row consumes `HakoAllocProviderBoundaryDiagnosticVocabularyReport`, checks
the vocabulary and inactive boundary facts, and publishes a scalar readiness
token for later provider selection inventory.

## Reasons

```text
0 = accepted
1 = missing provider diagnostic vocabulary
2 = rejected provider diagnostic vocabulary
3 = invalid provider diagnostic vocabulary
4 = provider activation not inactive
5 = host/hook/backend inactive boundary broken
6 = closed execution request
7 = invalid readiness token
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
bash tools/checks/k2_wide_hako_alloc_provider_readiness_preflight_guard.sh --level L2
```
