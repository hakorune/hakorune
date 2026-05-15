# 293x-423 DOCS-SLIM-016 Allocator Hook Taskboard Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history real-app taskboard pins from the allocator hook guard
band. Keep the README-only decoupling from `DOCS-SLIM-015` intact.

This row only thins mirror/history assertions. It does not move cards or change
hook, dry-run, or activation semantics.

## TODO

- [x] Remove the real-app taskboard pins from the allocator hook guard band.
- [x] Keep card status, design taskboard, implementation checks, and
  check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain
  `REAL_APP_TASKBOARD` landed-history pins.

## Scope

- Guard dependency cleanup only.
- M52-M63 allocator hook guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change hook, dry-run, or activation semantics in this row.
- Do not remove card / design taskboard / implementation / check-index
  assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_016_allocator_hook_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed real-app taskboard landed-history pins from the M52-M63 allocator
  hook guard band while keeping design taskboard assertions in place.
- Added `docs_slim_016_allocator_hook_taskboard_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_016_allocator_hook_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
