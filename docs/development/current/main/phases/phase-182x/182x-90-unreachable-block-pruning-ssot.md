# 182x-90: unreachable block pruning SSOT

Scope: delete basic blocks that are unreachable from the function entry after the current DCE liveness cuts have already stabilized.

## Why this slice exists

The current DCE pass already treats unreachable blocks as non-seeds for liveness, but the blocks themselves still remain in the MIR function map. That means CFG cleanup and dead-code cleanup are only half-complete.

This slice closes that gap by pruning dead blocks structurally after reachability has been computed.

## Decision

- prune blocks unreachable from `entry`
- keep the existing instruction-liveness contract unchanged
- do not reinterpret `Debug`, `KeepAlive`, or `Safepoint`
- keep CFG refresh local to MIR function cleanup; no backend/runtime policy move is introduced

## Contracts

- reachable blocks stay present
- unreachable blocks disappear from the function block map
- predecessor caches are rebuilt after pruning
- current instruction DCE semantics remain unchanged for reachable blocks

## Non-goals

- no new effect vocabulary
- no generic no-dst pure cleanup widening
- no Debug stripping
- no control-flow terminator semantic rewrite

## Exit Condition

- unreachable blocks are pruned by the DCE/CFG cleanup lane
- the current broader effect-sensitive / no-dst backlog remains separate
