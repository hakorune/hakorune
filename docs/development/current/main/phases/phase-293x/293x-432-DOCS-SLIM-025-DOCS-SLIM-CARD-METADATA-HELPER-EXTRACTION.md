# 293x-432 DOCS-SLIM-025 Docs-Slim Card Metadata Helper Extraction

Status: landed
Date: 2026-05-16

## Decision

Extract the repeated docs-slim card metadata assertions into a shared helper
in `tools/checks/lib/guard_common.sh`. Keep the allocator-provider and
production-allocator-port guard semantics unchanged.

This row only thins card metadata boilerplate. It does not change the
allocator-provider or production allocator-port guard bands.

## TODO

- [x] Add a shared helper for docs-slim card metadata assertions.
- [x] Convert the DOCS-SLIM-022/023/024 guards to use the helper.
- [x] Keep the per-row landed-history pin assertions and gate-leak checks in
  place.

## Scope

- Guard helper extraction only.
- DOCS-SLIM-022/023/024 metadata checks only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change allocator-provider or production allocator-port semantics in
  this row.
- Do not remove the per-script landed-history pin assertions or gate-leak
  checks.
- Do not wire the helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_025_docs_slim_card_metadata_helper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Added `guard_require_docs_slim_card_metadata` to `tools/checks/lib/guard_common.sh`.
- Converted the DOCS-SLIM-022/023/024 guards to use the shared helper for
  card metadata assertions.

## Evidence

```text
bash tools/checks/docs_slim_025_docs_slim_card_metadata_helper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
