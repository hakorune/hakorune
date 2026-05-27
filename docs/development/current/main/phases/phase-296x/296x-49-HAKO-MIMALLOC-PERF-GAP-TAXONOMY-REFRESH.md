---
Status: Current
Date: 2026-05-27
Scope: refresh gap taxonomy after measurement hygiene evidence.
Blocker: HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-48-HAKO-MIMALLOC-PERF-POST-DIAGNOSTIC-DECISION.md
---

# 296x-49 Hako Mimalloc Gap Taxonomy Refresh

## Purpose

Run the taxonomy adapter again over the measurement hygiene evidence instead of
starting optimization from noisy scout evidence.

## Required Input

```text
output_contract=hako-mimalloc-post-diagnostic-decision-v0
decision=refresh_gap_taxonomy_after_hygiene
selected_next_row=HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH-296X-001
optimization_started=0
winner_claim=0
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
