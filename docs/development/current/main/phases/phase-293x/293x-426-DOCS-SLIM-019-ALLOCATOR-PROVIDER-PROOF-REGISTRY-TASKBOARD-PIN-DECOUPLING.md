# 293x-426 DOCS-SLIM-019 Allocator Provider Proof/Registry Taskboard Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history real-app taskboard pins from the allocator provider proof
/ registry guard band. Keep the README-only decoupling from `DOCS-SLIM-017`
and the 018 taskboard decoupling intact.

This row only thins mirror/history assertions. It does not move cards or change
provider proof, registry, activation-entry, selection, or proof-bundle
semantics.

## TODO

- [x] Remove the real-app taskboard pins from the M72-M79 provider guard band.
- [x] Keep card status, phase README, implementation checks, and check-index
  assertions.
- [x] Add a guard proving the converted scripts no longer contain
  `REAL_APP_TASKBOARD` landed-history pins.

## Scope

- Guard dependency cleanup only.
- M72-M79 allocator provider guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change provider proof, registry, activation-entry, selection, or
  proof-bundle semantics in this row.
- Do not remove card / phase README / implementation / check-index assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_019_allocator_provider_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed real-app taskboard landed-history pins from the M72-M79 allocator
  provider proof / registry guard band while keeping phase README assertions in
  place.
- Added `docs_slim_019_allocator_provider_taskboard_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_019_allocator_provider_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
