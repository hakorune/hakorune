# 293x-421 DOCS-SLIM-014 Packed Record Taskboard Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history taskboard pins from the packed record guard cluster.
Keep the README-only decoupling from `DOCS-SLIM-013` intact.

This row only thins mirror/history assertions. It does not move cards or change
probe, pilot, packed-store, or backend semantics.

## TODO

- [x] Remove the `taskboard must mark` pins from the C207-C212 guard cluster.
- [x] Keep card status, plan status, record SSOT, implementation checks, and
  check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain direct
  numbered phase card paths or landed-history taskboard pins.

## Scope

- Guard dependency cleanup only.
- C207-C212 packed record guard cluster only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change probe, pilot, packed-store, or backend semantics in this row.
- Do not remove card / plan / record SSOT / implementation / check-index
  assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_014_packed_record_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed taskboard landed-history pins from the packed record guard cluster.
- Added `docs_slim_014_packed_record_taskboard_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_014_packed_record_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
