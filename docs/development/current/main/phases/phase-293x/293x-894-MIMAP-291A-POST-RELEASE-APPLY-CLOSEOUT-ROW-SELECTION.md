# 293x-894 MIMAP-291A Post Release-Apply Closeout Row Selection

Status: landed
Date: 2026-05-20

## Decision

Select the next narrow allocator model row after the segment arena backing
modeled allocation-ledger release-apply closeout.

## Context

MIMAP-288A / 289A / 290A closed the release-apply family in model space. Real
arena backing release, raw pointer residence, pointer-derived lookup,
segment-map mutation, atomic bitmap execution, OSVM/page-source execution,
provider activation, host allocator replacement, hooks, and `#[global_allocator]`
remain inactive.

## Scope

- Re-read the current allocator model chain after release-apply closeout.
- Select one narrow row that advances release/recycle modeling without opening
  real runtime/backend seams.
- Keep validation profile selection explicit before implementation.

## Stop Lines

- No implementation in this selection row.
- No new compiler acceptance shape in this selection row.
- No broad cleanup detour unless a concrete blocker is found.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No real raw pointer residence, pointer-derived lookup, real arena backing,
  real segment-map mutation, atomic bitmap execution, OSVM/page-source
  execution, provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selected Row

`MIMAP-292A`:

```text
segment arena backing modeled allocation-ledger release-applied recycle inventory
```

Rationale:

- MIMAP-288A / 289A / 290A closed release-apply facts in scalar/model space.
- The next narrow behavior should consume an accepted release-apply report and
  record a model-only release-applied recycle entry.
- This continues the existing release/recycle cadence used by prior modeled
  local-free reuse lanes while keeping real arena backing release, segment-map
  mutation, atomics, OSVM/page-source, provider activation, and raw pointer
  residence closed.

Validation profile:

```text
L2 daily
  VM proof
  MIR JSON emit
  route preflight

L3 EXE:
  deferred to a future release-applied recycle closeout pack unless this row
  introduces a new backend route shape.
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
