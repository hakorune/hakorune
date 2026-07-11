# 177x-90: redundant KeepAlive pruning SSOT

Status: SSOT
Date: 2026-04-12
Scope: let DCE remove only those `KeepAlive` instructions that no longer contribute any new liveness.

## Goal

- keep `KeepAlive` as the language/liveness vocabulary
- avoid treating every pure no-dst instruction as removable
- land one narrow effect-sensitive DCE slice after `phase-176x`

## Diagnosis

Current DCE now ignores unreachable uses, but it still treats all reachable `KeepAlive { values }` as unconditional semantic use-sites.

That leaves one narrow redundancy:

- if every `KeepAlive` value is already live because of another reachable use
- then that `KeepAlive` contributes no extra liveness
- but the instruction still remains in MIR

This is the smallest safe effect-sensitive DCE cut because `KeepAlive` is already defined as a liveness-only, runtime-no-op instruction.

## Fix

### 1. Keep the base liveness pass unchanged

Continue to compute reachable liveness from:

- non-pure instructions
- terminators
- edge args
- propagated pure defs

### 2. Prune only redundant `KeepAlive`

After current liveness is known, remove a reachable `KeepAlive { values }` only when every listed value is already live without needing that instruction.

### 3. Keep necessary `KeepAlive`

If any listed value is live only because of the `KeepAlive`, keep the whole instruction.

This cut intentionally does not try to split or rewrite `KeepAlive` operands yet.

## Acceptance

- a `KeepAlive` whose values are already live because of `Return` / edge args / other reachable uses is removed
- a `KeepAlive` that is the only reason a pure def stays alive remains
- current reachable-only DCE behavior stays unchanged

## Non-Goals

- no operand-level `KeepAlive` trimming
- no `Debug` removal
- no `Safepoint` removal
- no generic pure no-dst cleanup
