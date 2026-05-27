---
Status: Current
Date: 2026-05-27
Scope: refresh source/MIR observation after the active field fast path rollback.
Blocker: HAKO-MIMALLOC-POST-ROLLBACK-SOURCE-MIR-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-104-HAKO-MIMALLOC-POST-ROLLBACK-ACTIVE-FIELD-FAST-PATH-MEASUREMENT.md
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
---

# 296x-105 Hako Mimalloc Post Rollback Source/MIR Refresh

## Purpose

Refresh source/MIR observation after the active field fast path rollback. The
next keeper should avoid the row101 non-keeper pattern and should account for
the row104 measurement noise.

## Required Output

```text
output_contract=hako-mimalloc-post-rollback-source-mir-refresh-v0
input_contract=hako-mimalloc-post-rollback-active-field-fast-path-measurement-v0
method_count
confirmed_source_mir_risk_count
rejected_keeper=select_single_page_active_field_fast_path
selected_method
selected_risk_kind
next_keeper
summary=ok
```

## Stop Line

Do not implement another keeper in this refresh row. Keep provider activation,
replacement, hooks, globals, and winner claims closed.
