# 293x-242 D198 Proof App Runner Pilot

Status: Complete

## Purpose

D198 starts reducing proof-app `test.sh` duplication without moving guard
assertions or deleting app-local entrypoints. It adds a proof-app manifest and a
runner that dispatches proof app ids to existing guard scripts.

The existing guard scripts remain authoritative. The app-local `test.sh` files
remain present and executable; the pilot only makes a small set of recent proof
apps call the shared runner.

## Decision

Decision: accepted.

Add:

```text
tools/checks/proof_apps.toml
tools/checks/run_proof_app.sh
```

`proof_apps.toml` stores command-level entries for proof apps:

```toml
[[proof_apps]]
id = "M200"
app = "apps/hako-alloc-decommitted-page-reuse-precondition-proof"
label = "decommitted page reuse precondition proof"
profiles = ["pilot", "hako-alloc-purge"]
cmd = ["bash", "tools/checks/k2_wide_hako_alloc_decommitted_page_reuse_precondition_guard.sh"]
```

`run_proof_app.sh`:

- parses the manifest with Python stdlib `tomllib`
- runs commands as argv arrays, never through shell eval
- supports `--list`, `--profile <name>`, `--only <id,id,...>`, and positional ids
- validates that each manifest app directory and command entry exists
- runs from the repository root
- stops on first failed proof app and returns the child exit code

## Stop Lines

- Do not delete app-local `test.sh` files in this row.
- Do not delete or rewrite existing guard scripts in this row.
- Do not move pure-first EXE or MIR JSON assertions into TOML yet.
- Do not wire the runner into `dev_gate.sh` or `k2_wide_allocator_gate.sh`.
- Do not convert non-wrapper custom proof apps in this pilot.

## Acceptance

- `tools/checks/run_proof_app.sh --list` prints manifest entries.
- `tools/checks/run_proof_app.sh M200` executes the M200 proof guard.
- `tools/checks/run_proof_app.sh --profile pilot --dry-run` lists pilot rows
  without executing them.
- Converted app-local `test.sh` files still run successfully.
- Existing guard scripts still run directly.
- `docs/tools/check-scripts-index.md` documents the new runner.

## Verification

```bash
tools/checks/run_proof_app.sh --list
tools/checks/run_proof_app.sh --profile pilot --dry-run
tools/checks/run_proof_app.sh M200
apps/hako-alloc-decommitted-page-reuse-precondition-proof/test.sh
bash tools/checks/k2_wide_hako_alloc_decommitted_page_reuse_precondition_guard.sh
```
