# 293x-919 CHECKS-PROOF-APPS-MANIFEST-FAMILY-SPLIT-3 Proof Apps Manifest Family Split 3

Status: landed
Date: 2026-05-20

## Decision

Split the hako_alloc arena-backing release-lifecycle proof-app block out of
`tools/checks/proof_apps.toml` into a repo-root relative include file while
preserving the exact `run_proof_app.sh --list` output.

## Context

`293x-915`, `293x-916`, and `293x-918` already moved earlier proof-app blocks
into include files. The remaining root manifest still carried the large
arena-backing release/recycle family, so the root file was still larger than it
needed to be.

This row continues the same manifest seam by moving the arena-backing
release-lifecycle block into a dedicated include file without changing runner
behavior or proof commands.

## Scope

- Keep `tools/checks/proof_apps.toml` as the stable root manifest.
- Move the arena-backing release-lifecycle block into
  `tools/checks/manifests/proof_apps/hako_alloc_segment_arena_backing_release_lifecycle.toml`.
- Preserve exact proof-app list order.
- Keep the runner behavior unchanged.

## Non-Goals

- Do not change any proof-app command.
- Do not change `run_proof_app.sh` CLI behavior.
- Do not change validation profiles or row cadence.
- Do not split the remaining proof-app families in this row.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --list
diff -u before-list after-list
bash tools/checks/run_proof_app.sh --only MIMAP-220A --dry-run
bash tools/checks/run_proof_app.sh --only MIMAP-224A --dry-run
bash tools/checks/run_proof_app.sh --only MIMAP-304A --dry-run
bash tools/checks/proof_app_manifest_test_entry_guard.sh
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
