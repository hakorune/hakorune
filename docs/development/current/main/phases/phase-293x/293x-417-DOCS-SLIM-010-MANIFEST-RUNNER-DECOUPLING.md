# 293x-417 DOCS-SLIM-010 Manifest Runner Decoupling

Status: landed
Date: 2026-05-15

## Decision

Decouple the manifest runner pilot guard from landed-history phase README and
old real-app taskboard pins.

This row keeps the D199 card, shared runner, wrapper, manifest, and check-index
contracts, while removing assertions whose only purpose was to prove that a
landed row still appears in historical mirrors.

## TODO

- [x] Remove the phase README `293x-243` landed-history assertion from
  `manifest_runner_pilot_guard.sh`.
- [x] Remove the old real-app taskboard `D199 manifest runner library cleanup`
  landed-history assertion from `manifest_runner_pilot_guard.sh`.
- [x] Resolve the D199 card through `guard_require_phase293x_card`.
- [x] Add a guard proving the manifest runner pilot guard no longer depends on
  landed-history README/taskboard pins.

## Scope

- `manifest_runner_pilot_guard.sh` only.
- Guard dependency cleanup only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change `manifest_runner.py`, runner manifests, or runner behavior in
  this row.
- Do not remove card / check-index / wrapper / shared-runner assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_010_manifest_runner_decoupling_guard.sh
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Thinned `manifest_runner_pilot_guard.sh` to D199's durable contracts.
- Added `docs_slim_010_manifest_runner_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_010_manifest_runner_decoupling_guard.sh
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
