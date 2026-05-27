---
Status: Current
Date: 2026-05-27
Scope: refresh source/MIR observation after the select first-page cache keeper measurement.
Blocker: HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-99-HAKO-MIMALLOC-POST-SELECT-FIRST-PAGE-CACHE-KEEPER-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-100 Hako Mimalloc Post Select First-Page Cache Source/MIR Refresh

## Purpose

Refresh source/MIR observation after row99. The next action should be selected
from current method shape and exact-EXE counters, not from stale row97 risk.

## Required Output

```text
output_contract=hako-mimalloc-post-select-first-page-cache-source-mir-refresh-v0
input_contract=hako-mimalloc-post-select-first-page-cache-keeper-measurement-v0
method_count
confirmed_source_mir_risk_count
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.
