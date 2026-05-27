---
Status: Current
Date: 2026-05-27
Scope: decide the next action from refreshed gap taxonomy evidence.
Blocker: HAKO-MIMALLOC-PERF-REFRESHED-TAXONOMY-DECISION-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-49-HAKO-MIMALLOC-PERF-GAP-TAXONOMY-REFRESH.md
---

# 296x-50 Hako Mimalloc Refreshed Taxonomy Decision

## Purpose

Decide whether refreshed taxonomy can enter a first keeper optimization, or
whether the measurement/owner evidence still requires another diagnostic.

## Required Input

```text
output_contract=hako-mimalloc-gap-taxonomy-v0
sample_count=5
gap_owner=<one primary owner>
evidence_quality=stable|noisy
gap_confidence=low|medium|high
next_diagnostic
winner_claim=0
```

## Stop Line

Do not optimize, claim parity, activate providers, replace the process
allocator, install hooks, or select hakozuna in this row.
