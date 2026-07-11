---
Status: landed
Date: 2026-05-14
Row: LOOP-003B
Scope: Stage1 JSON v0 LoopRange lowering pilot.
Related:
  - docs/development/current/main/design/loop-range-stage1-route-ssot.md
  - docs/development/current/main/design/loop-range-parser-capsule-ssot.md
---

# LOOP-003B LoopRange Lowering Pilot

## Summary

`LoopRange` now has a first executable JSON v0 bridge route.

The pilot lowers:

```hako
loop i in start..end {
    ...
}
```

as:

```text
evaluate start/end once
header PHI for i
compare i < end
body
step i + 1
continue -> step
break -> exit
```

## Guardrails

The pilot deliberately rejects body writes:

```text
[freeze:contract][json_v0_bridge/loop_range_index_write]
[freeze:contract][json_v0_bridge/loop_range_carrier_unsupported]
```

This keeps the first route safe while carrier variables and verifier facts are
split into the next row.

## Stop Line

This row does not implement LoopRange verifier facts, general loop-carried
variables, custom step, inclusive ranges, or array iteration.

The next row is `LOOP-003C Stage1 LoopRange verifier facts and carrier policy`.

