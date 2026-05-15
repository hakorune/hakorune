# 293x-422 DOCS-SLIM-015 Allocator Hook README Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history phase README pins from the allocator hook guard band.
Keep the taskboard assertions in place so this row only covers README-pin
decoupling.

This row only thins mirror/history assertions. It does not move cards or change
hook, dry-run, or activation semantics.

## TODO

- [x] Remove the `phase README must list` pins from the M52-M63 hook guard
  band.
- [x] Keep card status, taskboard rows, implementation checks, and check-index
  assertions.
- [x] Add a guard proving the converted scripts no longer contain direct phase
  README pins.

## Scope

- Guard dependency cleanup only.
- M52-M63 allocator hook guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change hook, dry-run, or activation semantics in this row.
- Do not remove card / taskboard / implementation / check-index assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_015_allocator_hook_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed phase README landed-history pins from the M52-M63 allocator hook
  guard band while keeping taskboard assertions in place.
- Added `docs_slim_015_allocator_hook_readme_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_015_allocator_hook_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
