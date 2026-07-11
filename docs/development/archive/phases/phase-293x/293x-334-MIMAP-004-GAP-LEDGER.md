---
Status: Landed
Date: 2026-05-14
Row: MIMAP-004
Scope: substrate and representation gap ledger for mimalloc-shaped Hakorune allocator work.
Related:
  - docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md
  - docs/development/current/main/design/mimalloc-lifecycle-rewrite-blueprint-ssot.md
  - docs/development/current/main/investigations/mimalloc-source-concept-inventory.md
  - docs/development/current/main/phases/phase-293x/293x-mimalloc-port-taskboard.md
---

# MIMAP-004 Substrate and Representation Gap Ledger

## Summary

Converted the mimalloc concept inventory and lifecycle blueprint into explicit
capability, representation, semantic, and deferred-unsafe gap rows.

## Outputs

- Added `docs/development/current/main/design/mimalloc-substrate-representation-gap-ledger-ssot.md`.
- Defined fail-fast policy for unsupported `PackedArray<T>`, OSVM, atomic,
  raw buffer, and host allocator replacement routes.
- Seeded future rows for `uses`, OSVM, atomic, TLS, raw buffer, bitmap, const,
  random, packed metadata, and provider work.
- Filtered acceptable first executable slices to avoid hard substrate gaps.

## Main Decision

The port should choose a smaller executable slice when a required substrate or
representation is missing. It must not emulate unsupported capabilities with
silent fallback behavior.

## Next

`MIMAP-005A` should define the Hakorune brand/type vocabulary for allocator ids,
counts, generations, and sizes.
