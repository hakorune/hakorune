# Observer/Control DCE Owner Contract SSOT

Status: SSOT
Date: 2026-04-12
Owner: `phase-202x`

## Purpose

- fix lane C ownership before any `Debug` or terminator-adjacent operand/control liveness cleanup code widening
- keep observer/control reasoning separate from generic memory lane B and local-field lane A

## Lane C Inventory

### C1 Observer Policy

- `Debug { value, message }`

Current rule:

- `Debug` is kept by effect ownership
- `Debug` is not part of lane A or lane B
- `Debug` is not a generic no-dst pure cleanup target

Decision to make in C1:

- keep `Debug` as a permanent observer anchor
- or allow removal only under a separately documented diagnostic-off policy

### C2 Terminator / Control Policy

- `Branch`
- `Jump`
- `Return`

Current rule:

- these instructions are pure in effect terms but still anchor liveness and CFG shape
- DCE keeps them via explicit control-anchor handling, not via generic pure-value liveness

Decision to make in C2:

- whether any terminator-adjacent operand/control liveness cleanup should happen inside the DCE lane
- or whether all structural control cleanup should stay under the later simplification bundle (`SimplifyCFG` / jump-threading)
- `Branch` / `Jump` / `Return` deletion itself is not the target of C2

## Explicit Non-goals

- lane C does not own `Load` / `Store`
- lane C does not own `FieldGet` / `FieldSet`
- lane C does not own `KeepAlive`
- lane C does not reopen `Safepoint`
- lane C does not mix exception/control effect instructions (`Throw`, `Catch`) into the first observer inventory cut

## Current Reading

- lane A: local field reasoning
- lane B: generic private-carrier memory reasoning
- lane C: observer/control anchors only

## Immediate Sequence

1. `C0` docs-only inventory
2. `C1` `Debug` policy decision
3. `C2` terminator-adjacent operand/control liveness cleanup decision
