# 293x-804 MIMAP-279A Post Segment Arena Backing Modeled Allocation Ledger Closeout Row Selection

Status: landed
Date: 2026-05-19

## Decision

Select MIMAP-280A as the next narrow allocator row after the modeled
allocation-ledger closeout.

## Context

MIMAP-278A closed the modeled allocation-ledger family. The next row should
choose the next scalar/model bridge without opening real raw pointer residence,
real arena backing allocation, real segment-map mutation, atomic bitmap
execution, OSVM/page-source execution, or provider activation.

## Candidate Direction

MIMAP-280A adds a model-only allocation-ledger release candidate inventory.
It consumes accepted allocation-ledger facts and records scalar release-candidate
facts for later release/recycle modeling.

## Stop Lines

- No new allocator behavior in this planning row.
- No real raw pointer residence.
- No pointer-derived lookup or dereference.
- No real arena backing allocation.
- No real segment-map mutation.
- No real segment allocation/free execution.
- No atomic bitmap execution.
- No OSVM/page-source execution.
- No TLS, worker-local, worker scheduling, or source-level concurrency.
- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app or owner name.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Row

```text
MIMAP-280A
  segment arena backing modeled allocation-ledger release candidate inventory
```
