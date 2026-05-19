# 293x-916 CHECKS-PROOF-APPS-MANIFEST-FAMILY-SPLIT Proof Apps Manifest Family Split

Status: landed
Date: 2026-05-20

## Decision

Continue the proof-app manifest split by moving early hako_alloc families into
dedicated include files while preserving the exact `run_proof_app.sh --list`
output.

## Context

`293x-915` added schema-v0 `includes = [...]` support and moved the first purge
core proof-app family out of `tools/checks/proof_apps.toml`. The root manifest
still contained several complete early families before the current segment-map
readiness cadence rows.

This row uses the include seam to reduce root manifest size without changing
runner behavior or proof commands.

## Scope

- Keep `tools/checks/proof_apps.toml` as the stable root manifest.
- Move early families into:
  - `tools/checks/manifests/proof_apps/hako_alloc_osvm_fast_path.toml`
  - `tools/checks/manifests/proof_apps/hako_alloc_inventory.toml`
  - `tools/checks/manifests/proof_apps/hako_alloc_reclaim_core.toml`
  - `tools/checks/manifests/proof_apps/hako_alloc_reclaim_scheduler.toml`
  - `tools/checks/manifests/proof_apps/hako_alloc_segment_readiness.toml`
- Preserve exact proof-app list order.

## Non-Goals

- Do not change any proof-app command.
- Do not change `run_proof_app.sh` CLI behavior.
- Do not split all remaining proof-app families in this row.
- Do not change validation profiles or row cadence.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
bash tools/checks/run_proof_app.sh --list
diff -u before-list after-list
bash tools/checks/run_proof_app.sh --only MIMAP-042A --dry-run
bash tools/checks/run_proof_app.sh --only MIMAP-088A --dry-run
bash tools/checks/run_proof_app.sh --only MIMAP-149A --dry-run
bash tools/checks/proof_app_manifest_test_entry_guard.sh
bash tools/checks/manifest_runner_pilot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
