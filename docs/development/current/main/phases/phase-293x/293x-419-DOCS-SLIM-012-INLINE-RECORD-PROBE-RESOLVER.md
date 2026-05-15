# 293x-419 DOCS-SLIM-012 Inline Record Probe Resolver

Status: landed
Date: 2026-05-15

## Decision

Adopt the phase-card resolver helper in the inline record probe guard trio.

This row removes direct numbered phase-card paths and landed-history phase
README pins from the C206b/C206d/C206e guard cluster without moving cards or
changing probe, plan, or metadata-store semantics.

## TODO

- [x] Convert `C206b` probe guard card paths to `guard_require_phase293x_card`.
- [x] Convert `C206d` plan probe guard card paths to
  `guard_require_phase293x_card`.
- [x] Convert `C206e` metadata store indexed-read guard card paths to
  `guard_require_phase293x_card`.
- [x] Remove the `phase README must list` landed-history pins from the trio.
- [x] Add a guard proving the trio no longer contains direct numbered
  phase-293x card paths.

## Scope

- Guard path cleanup only.
- C206b/C206d/C206e inline-record probe trio only.
- No card movement.

## Stop Lines

- Do not move numbered cards in this row.
- Do not change probe, plan, metadata-store, or backend semantics in this row.
- Do not remove card / implementation / check-index assertions.
- Do not wire the resolver helper into `dev_gate.sh` or allocator-wide.

## Required Evidence

```text
bash tools/checks/docs_slim_012_inline_record_probe_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Implementation

- Converted `k2_wide_arraybox_inline_record_probe_guard.sh`.
- Converted `k2_wide_arraybox_inline_record_plan_probe_guard.sh`.
- Converted `k2_wide_metadata_store_indexed_read_guard.sh`.
- Added `docs_slim_012_inline_record_probe_resolver_guard.sh`.

## Evidence

```text
bash tools/checks/docs_slim_012_inline_record_probe_resolver_guard.sh
bash tools/checks/current_state_pointer_guard.sh
```
