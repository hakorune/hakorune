---
Status: landed
Date: 2026-05-14
Row: LOOP-003C
Scope: LoopRange function metadata facts.
Related:
  - docs/development/current/main/design/loop-range-stage1-route-ssot.md
---

# LOOP-003C LoopRange Facts

## Summary

The JSON v0 LoopRange lowering route now emits function-level
`loop_range_facts` metadata.

The fact surface records:

```text
index name
start/end values
index PHI value
preheader/header/body/step/exit blocks
fixed step 1
end-exclusive range
read-only index
carrier-write support disabled
```

## Ownership

This row owns metadata publication only.

Verifier/backend rows must read these facts instead of scanning raw CFG shape.

## Stop Line

This row does not implement loop-carried body writes.

The next row is `LOOP-003D LoopRange carrier policy`.

