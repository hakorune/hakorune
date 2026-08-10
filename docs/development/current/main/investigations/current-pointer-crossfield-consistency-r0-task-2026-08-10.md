# CURRENT-POINTER-CROSSFIELD-CONSISTENCY-R0

Status: pointer/mirror repair closed; guard implementation parked P0
Date: 2026-08-10
Owner: `docs/development/current/main/CURRENT_STATE.toml`

## Change

Make the current pointer fail closed when its live fields disagree. This row
does not select or change a compiler lane.

```text
design_stop:
  current_design_stop = current_execution_row
  next_design_card = current_execution_row
  next_execution_card = none-until-Decision
  latest_card_path contains the exact current row

fast:
  next_execution_card = current_execution_row
  latest_card_path is the selected implementation card

all modes:
  current_execution_design exists
  current_execution_summary describes the selected row or is absent
  latest_card / latest_card_path agree
  active card contains current_execution_row
```

`10-Now.md` becomes a thin field-name mirror. It must not hand-copy a concrete
mode, row, or landed chronology from `CURRENT_STATE.toml`.

## Acceptance

- add table-driven positive and negative fixtures for `fast`, `design_stop`,
  and `closeout` to the existing pointer guard;
- stale row, stale card, wrong mode pairing, and missing active-card token each
  fail with a stable one-line diagnostic;
- stale `current_execution_summary` is rejected or the redundant field is
  removed from the live pointer;
- the current real state passes unchanged;
- no compiler, parser, Recipe, Home, Fault, or runtime behavior changes;
- do not add a per-row shell guard; extend
  `tools/checks/current_state_pointer_guard.sh` and its existing fixture owner.

## Landed docs subset

The live state now points `current_execution_design` at the active rolling
card, and `10-Now.md` contains field names only. The remaining P0 is the
cross-field guard plus its table-driven fixtures; do not rewrite the mirror or
move the compiler lane again when implementing it.

## Parked follow-up

`CURRENT-STATE-REGISTRY-COMPACTION-R1` later moves historical task paths out of
the live pointer. It is a separate docs-topology change and must preserve every
live scalar and restart path. Do not combine that migration with this guard
strengthening row.
