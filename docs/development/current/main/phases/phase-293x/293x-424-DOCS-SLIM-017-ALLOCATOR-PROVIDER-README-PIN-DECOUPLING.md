# 293x-424 DOCS-SLIM-017 Allocator Provider README Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history phase README pins from the allocator provider guard band.
Keep the taskboard-pin cleanup for a follow-up row.

This row only thins mirror/history assertions. It does not move cards or change
provider boundary, manifest, or task-breakdown semantics.

## TODO

- [x] Remove the phase README pins from the M64-M66 provider guard band.
- [x] Keep card status, design taskboard, implementation checks, and
  check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain direct phase
  README pins.

## Scope

- Guard dependency cleanup only.
- M64-M66 allocator provider guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change provider boundary, manifest, or task-breakdown semantics in
  this row.
- Do not remove card / design taskboard / implementation / check-index
  assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_017_allocator_provider_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed phase README landed-history pins from the M64-M66 allocator provider
  guard band while keeping taskboard assertions in place.
- Added `docs_slim_017_allocator_provider_readme_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_017_allocator_provider_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
