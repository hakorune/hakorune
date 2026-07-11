# 176x-90: reachable-only DCE SSOT

Status: SSOT
Date: 2026-04-12
Scope: make the current pure-instruction DCE ignore uses that occur only in blocks unreachable from the function entry.

## Goal

- keep the current effect contract unchanged
- keep `used_values()` as the generic operand inventory
- make reachability from `entry` part of the DCE semantic use-site contract

## Diagnosis

Current `src/mir/passes/dce.rs` walks every block in the function when it:

- seeds live values from non-pure instructions
- reads terminator operands
- reads edge args
- backward-propagates liveness

That means values can stay alive only because some unreachable block still references them. This is the smallest real `cross-block` gap left in the current DCE lane.

## Fix

### 1. Keep DCE authority narrow

Do not add a new effect class or a second liveness vocabulary.

The pass should keep reading:

- `MirInstruction::effects()`
- `MirInstruction::used_values()`
- CFG reachability from `entry`

### 2. Restrict live-use seeding to reachable blocks

When DCE seeds `used_values`, it must consider only blocks that are reachable from the function entry.

That applies to:

- non-pure instructions
- no-dst instructions
- terminators
- edge args

### 3. Restrict backward propagation to reachable blocks

If a value is only used from unreachable blocks, it must not keep its defining pure instruction alive.

Backward propagation should therefore also walk reachable blocks only.

### 4. Do not widen this into unreachable-block cleanup

This cut must not:

- delete unreachable blocks
- delete non-pure unreachable instructions
- redefine effect safety

Those are separate follow-on tasks.

## Acceptance

- pure defs used only by unreachable blocks are eliminated
- pure instructions inside unreachable blocks may disappear if their defs become unused
- reachable edge-arg carrier values are still preserved
- current quick gate stays green

## Non-Goals

- no block-pruning pass
- no side-effect reasoning beyond current effect mask
- no partial effect-sensitive DCE
- no string lane work
