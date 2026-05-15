# 293x-431 DOCS-SLIM-024 Production Allocator Port and Mimalloc Closeout README Pin Decoupling

Status: landed
Date: 2026-05-16

## Decision

Remove landed-history phase README pins from the production allocator port
and mimalloc closeout guard band. Keep the design taskboard and real-app
taskboard assertions in place.

This row only thins mirror/history assertions. It does not move cards or
change production allocator port entry/closeout or mimalloc allocator closeout
semantics.

## TODO

- [x] Remove the phase README pins from the M38/M44/M45/M51 guard band.
- [x] Keep card status, design taskboard, real-app taskboard, implementation
  checks, and check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain direct phase
  README pins.

## Scope

- Guard dependency cleanup only.
- M38/M44/M45/M51 allocator closeout band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change production allocator port entry/closeout or mimalloc allocator
  closeout semantics in this row.
- Do not remove card / design taskboard / real-app taskboard / implementation
  / check-index assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_024_production_allocator_port_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed phase README landed-history pins from the M38/M44/M45/M51 allocator
  port and closeout guards while keeping design and real-app taskboard
  assertions in place.
- Added `docs_slim_024_production_allocator_port_readme_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_024_production_allocator_port_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
