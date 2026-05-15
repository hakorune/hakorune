# 293x-418 DOCS-SLIM-011 Record Metadata README Decoupling

Status: landed
Date: 2026-05-15

## Decision

Remove landed-history phase README pins from the record / metadata guard
cluster while keeping the record SSOT and the implementation checks as the real
contract.

This row only thins human-history assertions. It does not move cards or change
record, metadata, or packed-array semantics.

## TODO

- [x] Remove the `PHASE_README` landed-history assertions from the record /
  metadata guard cluster.
- [x] Keep the card status, implementation, record SSOT, and check-index
  assertions.
- [x] Add a guard proving the record / metadata guards no longer rely on phase
  README landed-history pins.

## Scope

- Guard dependency cleanup only.
- Record / metadata guard cluster only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change record, metadata, packed-array, or allocator semantics in this
  row.
- Do not remove card / SSOT / implementation / check-index assertions.

## Required Evidence

```text
bash tools/checks/docs_slim_011_record_metadata_readme_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Removed phase README landed-history assertions from the record / metadata
  guard cluster.
- Added `docs_slim_011_record_metadata_readme_decoupling_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_011_record_metadata_readme_decoupling_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
