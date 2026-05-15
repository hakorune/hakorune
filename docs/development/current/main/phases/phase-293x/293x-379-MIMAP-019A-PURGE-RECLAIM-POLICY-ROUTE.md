# 293x-379 MIMAP-019A Purge/Reclaim Policy Route

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-019A` is the next allocator row after `MIMAP-018A`. It should integrate a
small purge/reclaim/decommit policy route only through existing lifecycle and
stats observers.

## Scope

- Select the smallest existing hako_alloc purge/reclaim policy owner that can be
  composed from stable lifecycle/stats observers.
- Add one proof app and one guard for the selected route.
- Keep the row read-only or policy-only unless the selected existing owner
  already has a bounded execution contract.

## Stop Lines

- No OSVM/page-source activation unless this card is split into the explicit
  capability row.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend matcher shortcut.
- No page-map lookup unless the selected existing owner already owns that seam
  and the proof is explicitly scoped to it.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
