# 293x-917 CHECKS-GUARD-ROWS-MANIFEST-FAMILY-SPLIT Guard Rows Manifest Family Split

Status: landed
Date: 2026-05-20

## Decision

Split the hako_alloc closeout block out of `tools/checks/guard_rows.toml` into
a repo-root relative include file, while preserving the exact `run_row_guard.sh`
behavior and wrapper coverage.

## Context

`tools/checks/guard_rows.toml` had grown into a large mixed manifest: quick
static rows at the top, a long hako_alloc closeout block in the middle, and
utility rows at the bottom. The manifest runner already supports schema-v0
`includes = [...]`, so the closeout family can move without changing row order
or wrapper dispatch.

This row keeps the root manifest readable while leaving the closeout family in a
dedicated include file.

## Scope

- Keep `tools/checks/guard_rows.toml` as the stable root manifest.
- Move the hako_alloc closeout block into
  `tools/checks/manifests/guard_rows/hako_alloc_closeout.toml`.
- Load included rows before local rows so row ordering stays identical.
- Keep public closeout wrappers thin and manifest-backed.
- Keep the guard inventory and wrapper inventory aligned with the split.

## Non-Goals

- Do not change any row command.
- Do not change `run_row_guard.sh` CLI behavior.
- Do not change the quick-static rows.
- Do not split the remaining non-closeout guard rows in this row.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
python3 -m py_compile tools/checks/guard_manifest_inventory.py
bash tools/checks/guard_manifest_inventory_guard.sh
bash tools/checks/k2_wide_manifest_wrapper_guard.sh
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
