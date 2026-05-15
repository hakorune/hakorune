# 293x-428 DOCS-SLIM-021 Allocator Provider Boundary/Manifest/Task Breakdown Taskboard Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history real-app taskboard pins from the allocator provider
boundary/manifest/task breakdown guard band. Keep the README-only decoupling
from `DOCS-SLIM-017` and the 018-020 taskboard decouplings intact.

This row only thins mirror/history assertions. It does not move cards or
change provider boundary vocabulary, manifest vocabulary, or task breakdown
semantics.

## TODO

- [x] Remove the real-app taskboard pins from the M64-M66 provider guard band.
- [x] Keep card status, design taskboard, implementation checks, and
  check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain
  `REAL_APP_TASKBOARD` landed-history pins.

## Scope

- Guard dependency cleanup only.
- M64-M66 allocator provider guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change provider boundary vocabulary, manifest vocabulary, or task
  breakdown semantics in this row.
- Do not remove card / design taskboard / implementation / check-index
  assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_021_allocator_provider_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed real-app taskboard landed-history pins from the M64-M66 allocator
  provider guard band while keeping design taskboard assertions in place.
- Added `docs_slim_021_allocator_provider_taskboard_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_021_allocator_provider_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
