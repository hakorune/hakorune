---
Status: landed
Date: 2026-05-14
Row: LOOP-003D
Scope: LoopRange carrier policy.
Related:
  - docs/development/current/main/design/loop-range-stage1-route-ssot.md
---

# LOOP-003D LoopRange Carrier Policy

## Summary

LoopRange now distinguishes body-local writes from loop-carried writes.

Allowed:

```text
fresh body-local bindings
```

Rejected:

```text
range index writes
pre-loop variable writes / loop-carried locals
```

## Metadata

`loop_range_facts` now expose:

```text
body_local_writes_supported = true
loop_carried_writes_supported = false
body_writes_supported = false
```

## Stop Line

This row does not implement full loop-carried PHI support.

The next row is `PACKED-003 source PackedArray direct-read consumption`.

