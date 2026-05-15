# 293x-380 MIMAP-020A OSVM/Page-Source Capability Pilot

Status: ready
Date: 2026-05-15

## Decision

`MIMAP-020A` is the next allocator row after `MIMAP-019A`. It may start a
capability-gated OSVM/page-source pilot only after the in-memory facade route
and read-only purge/reclaim policy route are stable.

## Scope

- Select one existing page-source / OSVM owner that already has an executable
  guard.
- Add or extend one narrow proof app and one guard only if the selected row
  needs a mimalloc-facing acceptance seam.
- Keep unsupported capability paths fail-fast; do not silently fall back to an
  in-memory page model.

## Stop Lines

- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No production allocator activation.
- No atomic/TLS/remote-free expansion.
- No page-map lookup unless the selected owner already owns that seam.
- No broad OSVM API surface beyond the selected capability row.
- No backend matcher shortcut; supported routes must remain MIR-owned route
  metadata / existing CoreMethodContract paths.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
