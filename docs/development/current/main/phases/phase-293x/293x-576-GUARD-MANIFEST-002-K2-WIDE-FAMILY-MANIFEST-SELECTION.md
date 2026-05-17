# 293x-576 GUARD-MANIFEST-002 K2-Wide Family Manifest Selection

Status: landed
Date: 2026-05-17

## Decision

`GUARD-MANIFEST-002` is the planning row selected by
`GUARD-MANIFEST-001`.

The proof app test entrypoints now delegate through `run_proof_app.sh`. The
next cleanup should select exactly one `k2_wide_*` guard family to migrate
toward manifest-backed thin wrappers.

Selected family:

```text
recent hako_alloc segment closeout guards:
  tools/checks/k2_wide_hako_alloc_segment_arena_bitmap_inventory_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_segment_lifecycle_scalar_state_closeout_guard.sh
  tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_closeout_guard.sh
```

Next selected row:

```text
GUARD-MANIFEST-003
```

`GUARD-MANIFEST-003` should keep the top-level `k2_wide_*` entrypoints stable
while moving the thick bodies behind manifest-owned implementation commands.

## Scope

- Review recent `k2_wide_hako_alloc_*` proof guards and choose one family.
- Prefer a family with repeated app/card/file/proof-output structure.
- Keep existing guard script names as stable entrypoints during migration.
- Decide whether the row needs schema extension, shared helper extraction, or
  only manifest entries plus thin shell wrappers.

## Stop Lines

- No allocator `.hako` behavior.
- No compiler acceptance change.
- No deletion of old guard entrypoints in the selection row.
- No dev-gate / allocator-wide manifest pilot wiring.
- No all-family batch migration.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `GM002.1` | Inventory repeated `k2_wide_*` guard families. | one family is selected with evidence. | no implementation bundle |
| `GM002.2` | Decide schema/helper need. | selected row says table extension vs wrapper-only. | no broad generator |
| `GM002.3` | Create next implementation card. | next card exists and is selected current. | no behavior |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Selected the recent hako_alloc segment closeout family for the first
  `k2_wide_*` manifest-backed thin-wrapper migration.
- Chose wrapper-safe migration: keep public script names stable, move thick
  bodies behind implementation commands, and add manifest rows pointing to the
  implementation commands.
- No schema change is required for `guard_rows.toml`.

