# 293x-647 GUARD-MANIFEST-012 Batch Migration Inventory

Status: landed
Date: 2026-05-18

## Decision

`GUARD-MANIFEST-012` is a BoxShape cleanup row for the guard manifest migration.

The thin-wrapper pilot is valid, but the remaining public `k2_wide_*` guard
surface is too large to migrate one file at a time without creating a long
manual tail. This row should add a small inventory contract and guard that makes
the remaining migration state explicit before any broader generator or
declarative-spec row is selected.

## Scope

- Update the guard manifest migration SSOT with a batch migration direction.
- Add a lightweight inventory tool for guard manifest counts and closeout
  wrapper coverage.
- Add a row guard for the inventory contract.
- Register the guard in `tools/checks/guard_rows.toml` and
  `docs/tools/check-scripts-index.md`.
- Keep stable public shell entrypoints intact.

## Stop Lines

- No allocator behavior.
- No compiler route behavior.
- No source syntax change.
- No deletion or rename of existing public guard entrypoints.
- No all-at-once conversion of hundreds of guards.
- No shell `eval` or `shell=True` subprocess dispatch.
- No `dev_gate.sh` / allocator-wide policy change.
- No backend `.inc` matcher.
- No silent fallback.

## Required Evidence

```text
bash tools/checks/guard_manifest_inventory_guard.sh
bash tools/checks/run_row_guard.sh --only guard-manifest-inventory
bash tools/checks/k2_wide_manifest_wrapper_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Result

`GUARD-MANIFEST-012` landed the guard manifest inventory owner and guard:

```text
tools/checks/guard_manifest_inventory.py
tools/checks/guard_manifest_inventory_guard.sh
tools/checks/guard_rows.toml row id guard-manifest-inventory
```

The row keeps public guard entrypoints stable and does not move existing guard
bodies. It makes hako_alloc closeout wrapper coverage fail-fast through manifest
counts so the remaining migration can batch by family.

`GUARD-MANIFEST-012` selects `GUARD-MANIFEST-013`.
