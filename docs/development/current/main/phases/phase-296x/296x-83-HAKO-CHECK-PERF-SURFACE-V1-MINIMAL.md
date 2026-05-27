---
Status: Current
Date: 2026-05-27
Scope: add minimal source-level hako_check perf-surface v1 fields.
Blocker: HAKO-CHECK-PERF-SURFACE-V1-MINIMAL-296X-001
Related:
  - docs/development/current/main/design/hako-check-mir-observation-boundary-ssot.md
  - tools/hako_check/perf_surface_inventory.py
---

# 296x-83 hako_check Perf-Surface v1 Minimal

## Purpose

Extend the source-level `hako_check perf-surface` report without adding rewrite,
optimizer, or MIR ownership.

## Required Output

```text
output_contract=hako-check-perf-surface-v1
input_contract=hako-check-perf-surface-contract-v0
loop_field_get_count
loop_field_set_count
loop_array_get_count
loop_array_length_count
allocation_like_in_loop_count
suggested_next_kind=box_count|box_shape|mir_diagnostic|none
confidence=low|medium|high
winner_claim=0
replacement_active=0
summary=ok
```

## Stop Line

MIR method shape observation remains separate. Keeper diff adapter belongs to
row 84.
