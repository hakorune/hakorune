# 293x-619 MIMAP-120A Post-Local-Free-Integration Row Selection

Status: landed
Date: 2026-05-18

## Decision

`MIMAP-120A` is the planning row selected by `MIMAP-119A`.

The segment allocation modeled lane now owns the local-free composition:

```text
released-span report
  -> candidate ledger
  -> apply-plan ledger
  -> explicit page-model apply
```

This row should select exactly one next allocator behavior, closeout,
substrate, or narrow Hakorune acceptance row using the mimalloc validation
cadence.

## Result

`MIMAP-120A` selects:

```text
MIMAP-121A segment allocation modeled local-free integration closeout guard
```

Validation cadence:

```text
L4 closeout row:
  manifest-backed row guard via run_row_guard.sh --only <row-id>
  public k2_wide wrapper
```

The selected row should freeze the MIMAP-119A integration owner, proof app,
guard, SSOT, module export, README entry, and stop-line set before any later
row opens broader segment execution, segment-map lookup, raw pointer residence,
arena backing, atomic bitmap execution, page-source / OSVM calls, provider
activation, or backend matchers.

## Scope

- Review `MIMAP-119A` evidence and the local-free integration route.
- Apply
  `docs/development/current/main/design/mimalloc-row-validation-cadence-ssot.md`.
- Select exactly one next row.

## Stop Lines

- No new `.hako` behavior.
- No cleanup bundle.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No arena backing allocation.
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
| `120A.1` | Review current evidence after MIMAP-119A. | selected row cites validation level. | no behavior |
| `120A.2` | Pick exactly one next row. | new selected card exists. | no bundle |
| `120A.3` | Update current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
