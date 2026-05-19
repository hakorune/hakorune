# 293x-890 MIMAP-287A Post Release-Intent Closeout Row Selection

Status: landed
Date: 2026-05-20

## Decision

Select the next narrow allocator model row after the segment arena backing
modeled allocation-ledger release-intent closeout.

## Context

MIMAP-284A / 285A / 286A closed the release-intent family in model space. Real
arena backing release, raw pointer residence, pointer-derived lookup,
segment-map mutation, atomic bitmap execution, OSVM/page-source execution,
provider activation, host allocator replacement, hooks, and `#[global_allocator]`
remain inactive.

## Scope

- Re-read the current allocator model chain after release-intent closeout.
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

`MIMAP-288A`:

```text
segment arena backing modeled allocation-ledger release apply inventory
```

Rationale:

- MIMAP-284A / 285A / 286A closed release-intent facts in scalar/model space.
- The next narrow behavior should consume an accepted release-intent report and
  record a model-only release-apply entry.
- This still does not release real arena backing, mutate segment-map state,
  execute atomic bitmap operations, call OSVM/page-source, or open raw pointer
  residence.

Validation profile:

```text
L2 daily
  VM proof
  MIR JSON emit
  route preflight

L3 EXE:
  deferred to a future release-apply closeout pack unless this row introduces
  a new backend route shape.
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
