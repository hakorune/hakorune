# 293x-886 MIMAP-283A Post ReportFields Cleanup Row Selection

Status: landed
Date: 2026-05-20

## Decision

Return from the allocator `ReportFields` cleanup detour to the mimalloc modeled
lane and select the next narrow allocator row.

The cleanup detour closed the known allocator `ReportFields` owner inventory
under:

```text
docs/development/current/main/design/record-local-scalarization-ssot.md
```

## Scope

- Re-read the current mimalloc modeled lane after the ReportFields cleanup
  closeout.
- Select one narrow row that advances the allocator model without opening real
  raw pointer residence, real arena backing, real segment-map mutation, atomic
  bitmap execution, OSVM/page-source execution, provider activation, hooks,
  host allocator replacement, or `#[global_allocator]`.
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

## Completion Criteria

- The next mimalloc modeled row is selected with a single owner and bounded
  validation profile.
- The selected row documents whether it is an L2 daily row, first-pattern L3
  row, or closeout row.
- The ReportFields cleanup detour remains closed unless a new owner appears.

## Selected Row

`MIMAP-284A` is selected:

```text
segment arena backing modeled allocation-ledger release intent inventory
```

Rationale:

- MIMAP-280A / 281A / 282A closed the modeled allocation-ledger
  release-candidate family.
- The exact-`usize` field-group and allocator `ReportFields` cleanup detours
  are now closed.
- The next narrow allocator behavior should consume an accepted
  release-candidate report into a scalar/model release-intent ledger before any
  real arena backing release, pointer residence, segment-map mutation, OSVM,
  atomics, provider activation, host allocator replacement, hooks, or
  `#[global_allocator]` open.

Validation profile:

```text
L2 daily
  VM proof
  MIR JSON emit
  route preflight

L3 EXE:
  deferred to the future release-intent closeout pack unless this row
  unexpectedly introduces a new backend route shape.
```

## Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
