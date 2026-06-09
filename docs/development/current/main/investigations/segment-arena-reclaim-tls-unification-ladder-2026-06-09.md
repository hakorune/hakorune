# Segment Arena Reclaim TLS Unification Ladder

Status: Active
Date: 2026-06-09

This note keeps the remaining segment-arena reclaim/TLS work narrow. The
current selection row points at `segment_arena_reclaim_tls_unification`, but
the concrete read-only seam is still the composition of the existing scalar
proof owners:

- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_readiness_matrix_box.hako`
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_execution_support_gate_box.hako`
- `segment_arena_backing_modeled_allocation_ledger_release_recycle_pointer_derived_lookup_prerequisite_box.hako`
- `worker_tls_pilot_box.hako`

## Goal

Keep the segment arena reclaim/TLS work as a read-only proof surface until a
single integration owner can consume the matrix, support gate, lookup
prerequisite, and worker/TLS evidence without opening provider activation,
replacement, hooks, or global allocator claims.

## Narrow order

1. Read the readiness matrix and support gate as the primary scalar surface.
2. Keep pointer-derived lookup prerequisite evidence explicit.
3. Consume the worker/TLS pilot as a bounded worker identity surface.
4. Only then decide whether a new read-only integration owner is needed.

## Stop line

- No provider activation.
- No host allocator replacement.
- No hook installation.
- No worker scheduling surface.
- No new runtime mutation semantics.

## Notes

The existing current lane remains the selection row:

- `docs/development/current/main/phases/phase-296x/296x-645-HAKO-MIMALLOC-SEGMENT-ARENA-RECLAIM-TLS-UNIFICATION-SELECTION.md`

If this ladder grows into a code lane, the new owner should stay read-only and
report-only at first.
