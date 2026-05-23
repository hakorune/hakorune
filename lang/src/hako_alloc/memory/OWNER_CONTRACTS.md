# Hako Alloc Memory Owner Contracts

Status: Active
Scope: compact index for owner-specific responsibility notes.
Related:
- `README.md`
- `MODULE_INDEX.md`

This file is the entry point for owner-specific responsibility and stop-line
notes. The detailed notes are split by family so the layer entry remains small.

## Contract Files

- `OWNER_CONTRACTS_CORE_PROVIDER.md`: core/object lifecycle/provider contracts.
- `OWNER_CONTRACTS_SEGMENT_ALLOCATION.md`: segment allocation and local-free modeled ledger contracts.
- `OWNER_CONTRACTS_ARENA_RECLAIM.md`: segment arena, worker/TLS, reclaim, and metadata contracts.

Rule: keep `README.md` as the layer entry and style contract only. Add new
owner-specific long-form notes to the matching family file, not to `README.md`.
