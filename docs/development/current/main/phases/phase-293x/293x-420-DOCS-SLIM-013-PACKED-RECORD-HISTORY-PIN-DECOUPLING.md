# 293x-420 DOCS-SLIM-013 Packed Record History Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history phase README pins from the packed record guard cluster.
Keep the taskboard assertions in place for the follow-up row so the README-only
decoupling stays isolated.

This row only thins mirror/history assertions. It does not move cards or change
probe, pilot, packed-store, or backend semantics.

## TODO

- [x] Remove the `phase README must list` pins from the C207-C212 guard
  cluster.
- [x] Keep card status, plan status, record SSOT, taskboard rows, implementation
  checks, and check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain direct
  numbered phase card paths or landed-history phase README pins.

## Scope

- Guard dependency cleanup only.
- C207-C212 packed record guard cluster only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change probe, pilot, packed-store, or backend semantics in this row.
- Do not remove card / plan / record SSOT / taskboard / implementation /
  check-index assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_013_packed_record_history_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed phase README landed-history pins from the packed record guard cluster
  while keeping taskboard assertions in place.
- Added `docs_slim_013_packed_record_history_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_013_packed_record_history_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
