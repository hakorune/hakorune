# 293x-427 DOCS-SLIM-020 Allocator Provider Manifest/Readiness/Registry Taskboard Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history real-app taskboard pins from the allocator provider
manifest/readiness/registry guard band. Keep the README-only decoupling from
`DOCS-SLIM-017` and the 018/019 taskboard decouplings intact.

This row only thins mirror/history assertions. It does not move cards or
change provider manifest parser, manifest CLI, readiness preflight, registry
boundary, or combined dry-run semantics.

## TODO

- [x] Remove the real-app taskboard pins from the M67-M71 provider guard band.
- [x] Keep card status, phase README, implementation checks, and check-index
  assertions.
- [x] Add a guard proving the converted scripts no longer contain
  `REAL_APP_TASKBOARD` landed-history pins.

## Scope

- Guard dependency cleanup only.
- M67-M71 allocator provider guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change provider manifest parser, manifest CLI, readiness preflight,
  registry boundary, or combined dry-run semantics in this row.
- Do not remove card / phase README / implementation / check-index assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_020_allocator_provider_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed real-app taskboard landed-history pins from the M67-M71 allocator
  provider manifest/readiness/registry guard band while keeping phase README
  assertions in place.
- Added `docs_slim_020_allocator_provider_taskboard_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_020_allocator_provider_taskboard_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
