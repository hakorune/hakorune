# 202x-90 Observer/Control Docs Inventory SSOT

Status: SSOT

## Goal

- finish lane C0 from `phase190x`
- freeze the owner split before `Debug` / terminator policy changes start

## Split

1. `C0`
   - docs-only inventory
   - no code widening
2. `C1`
   - `Debug` policy decision
   - decide whether `Debug` stays a permanent observer anchor
3. `C2`
   - terminator-adjacent operand/control liveness cleanup
   - decide whether any of that belongs in DCE or should stay deferred to later `SimplifyCFG` / jump-threading

## Current Owner Read

- `Debug` is currently kept by effect ownership
- `Branch` / `Jump` / `Return` are currently kept by explicit control-anchor handling
- `Throw` / `Catch` stay outside this first lane-C split

## Next

1. `C1 Debug policy decision`
2. `C2 terminator-adjacent operand/control liveness cleanup`
