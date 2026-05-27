---
Status: Current
Date: 2026-05-27
Scope: add keeper before/after diff adapter for source surface and measurement evidence.
Blocker: HAKO-MIMALLOC-KEEPER-BEFORE-AFTER-DIFF-ADAPTER-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-83-HAKO-CHECK-PERF-SURFACE-V1-MINIMAL.md
---

# 296x-84 Hako Mimalloc Keeper Before/After Diff Adapter

## Purpose

Add an adapter that compares before/after source perf-surface reports and
measurement evidence for one keeper. This is not hako_check core and it does
not implement keepers.

## Required Output

```text
output_contract=hako-mimalloc-keeper-before-after-diff-v0
keeper_id
source_surface_delta_ready=1
measurement_delta_ready=1
keeper_effect=accepted|no_effect|regressed|inconclusive
winner_claim=0
summary=ok
```

## Stop Line

Do not add MIR method shape here. That belongs to row 85.
