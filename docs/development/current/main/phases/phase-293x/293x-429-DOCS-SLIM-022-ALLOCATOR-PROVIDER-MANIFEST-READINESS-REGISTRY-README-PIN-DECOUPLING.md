# 293x-429 DOCS-SLIM-022 Allocator Provider Manifest/Readiness/Registry README Pin Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history phase README pins from the allocator provider
manifest/readiness/registry guard band. Keep the design taskboard assertions in
place.

This row only thins mirror/history assertions. It does not move cards or
change provider manifest parser, manifest CLI, readiness preflight, registry
boundary, or combined dry-run semantics.

## TODO

- [x] Remove the phase README pins from the M67-M71 provider guard band.
- [x] Keep card status, design taskboard, implementation checks, and
  check-index assertions.
- [x] Add a guard proving the converted scripts no longer contain direct phase
  README pins.

## Scope

- Guard dependency cleanup only.
- M67-M71 allocator provider guard band only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change provider manifest parser, manifest CLI, readiness preflight,
  registry boundary, or combined dry-run semantics in this row.
- Do not remove card / design taskboard / implementation / check-index
  assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_022_allocator_provider_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed phase README landed-history pins from the M67-M71 allocator provider
  guard band while keeping design taskboard assertions in place.
- Added `docs_slim_022_allocator_provider_readme_pin_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_022_allocator_provider_readme_pin_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
