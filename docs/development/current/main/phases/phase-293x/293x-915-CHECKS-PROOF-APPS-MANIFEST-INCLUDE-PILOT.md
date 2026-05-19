# 293x-915 CHECKS-PROOF-APPS-MANIFEST-INCLUDE-PILOT Proof Apps Manifest Include Pilot

Status: landed
Date: 2026-05-20

## Decision

Add schema-v0 manifest `includes = [...]` support to the shared manifest runner
and move the first hako_alloc purge proof-app family into a split manifest file.

## Context

`tools/checks/proof_apps.toml` had grown past one thousand lines. The runner was
already the SSOT for proof-app selection, but the manifest itself had no split
mechanism, so every proof-app family accumulated in one file.

This cleanup adds a small include seam before doing any broad manifest
reshaping. The first split keeps ordering identical and validates both included
and local entries through the existing runner.

## Scope

- Add repo-root relative `includes = [...]` support to
  `tools/checks/lib/manifest_runner.py`.
- Keep `schema_version = 0`.
- Load included entries before local entries so the root manifest can split
  early families without changing list order.
- Move `M197` through `M212` into
  `tools/checks/manifests/proof_apps/hako_alloc_purge_core.toml`.
- Update `proof_app_manifest_test_entry_guard.sh` so it checks included proof
  app entries too.
- Update the check scripts index to document manifest includes.

## Non-Goals

- Do not change any proof-app command.
- Do not change `run_proof_app.sh` CLI behavior.
- Do not migrate proof apps into `dev_gate.sh` or allocator-wide.
- Do not split all proof-app families in this row.
- Do not change MIMAP-308A behavior.

## Required Evidence

```text
python3 -m py_compile tools/checks/lib/manifest_runner.py
bash tools/checks/run_proof_app.sh --list
diff -u before-list after-list
bash tools/checks/run_proof_app.sh --only M197 --dry-run
bash tools/checks/run_proof_app.sh --only MIMAP-142A --dry-run
bash tools/checks/proof_app_manifest_test_entry_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
