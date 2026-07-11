# 185x-90: dead local field-set write pruning SSOT

Status: SSOT
Date: 2026-04-12
Scope: land the next effect-sensitive DCE slice by removing dead `FieldSet` writes on definitely non-escaping local boxes, while keeping `Store`, `Load`, `Debug`, control-flow terminators, and broader effect-sensitive DCE separate.

## Goal

- keep canonical MIR as the only semantic owner
- consume the existing local-box escape/read analysis instead of inventing a new memory vocabulary
- reopen broader DCE one boundary class at a time after the earlier exact slices are already green

## Decision

- `FieldSet { base: _, value, .. }` may be removed when:
  - `base` resolves to a definitely non-escaping local box root, and
  - that root has no reachable `FieldGet` observers
- dead local write removal may also drop the alias chain feeding that write when nothing else keeps it live
- `Store` stays outside this cut
- `Load` stays outside this cut
- `Debug` stays outside this cut because it still carries debug effect
- terminators (`Return` / `Branch` / `Jump`) stay outside this cut

## Acceptance

- dead `FieldSet` writes on non-escaping local boxes disappear when otherwise unobserved
- live `FieldGet` observers keep the corresponding write in place
- the dead alias chain feeding such a write disappears too when nothing else keeps it live
- reachable edge-arg and return-driven liveness stays unchanged
- `Store`, `Load`, and `Debug` instructions remain in place
- `tools/checks/dev_gate.sh quick` stays green

## Exit

- dead local `FieldSet` write pruning is landed
- broader effect-sensitive / partial DCE remains separate backlog
