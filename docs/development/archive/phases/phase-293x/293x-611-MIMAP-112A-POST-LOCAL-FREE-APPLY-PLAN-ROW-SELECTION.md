# 293x-611 MIMAP-112A Post-Local-Free-Apply-Plan Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-112A` is the planning row selected by `MIMAP-111A`.

The segment allocation modeled lane now has:

```text
modeled consume
  -> allocation ledger
  -> release span facts
  -> released-span ledger
  -> local-free candidate ledger
  -> local-free apply-plan ledger
```

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

## Result

`MIMAP-112A` selects:

```text
MIMAP-113A segment allocation modeled local-free scalar lane closeout guard
```

Validation cadence:

```text
L4 closeout row:
  manifest-backed row guard via run_row_guard.sh --only <row-id>
  public k2_wide wrapper
```

The selected row should freeze the scalar local-free chain through
`MIMAP-111A` before any later page-local free-list mutation row is selected.

## Scope

- Review `MIMAP-111A` evidence and the current segment allocation modeled lane.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No real segment allocation/free execution.
- No free-list mutation.
- No page state mutation.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No atomic bitmap execution.
- No page-source or OSVM execution.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `112A.1` | Review current evidence after MIMAP-111A. | selected row cites validation level. | no behavior |
| `112A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `112A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
