---
Status: landed
Date: 2026-05-14
Row: LOOP-003A
Scope: Stage1 LoopRange route decision.
Related:
  - docs/development/current/main/design/loop-range-stage1-route-ssot.md
  - docs/development/current/main/design/loop-range-parser-capsule-ssot.md
---

# LOOP-003A LoopRange Stage1 Route Decision

## Summary

`LoopRange` now has a Stage1 route contract before executable lowering.

The bridge accepts `LoopRange` JSON as metadata and fails before lowering with a
stable contract tag:

```text
[freeze:contract][json_v0_bridge/loop_range_route_open]
```

## Ownership

Stage0:

```text
parse loop i in start..end
transport LoopRange metadata
```

Stage1:

```text
entry-bound capture
continue-safe step
block-local read-only index
verifier facts
```

## Stop Line

This row does not implement executable LoopRange lowering.

The next row is `LOOP-003B Stage1 LoopRange lowering pilot`.

