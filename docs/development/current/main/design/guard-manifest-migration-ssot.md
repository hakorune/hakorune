---
Status: SSOT
Date: 2026-05-17
Scope: manifest-backed guard/proof-app runner migration and no-growth policy.
Related:
  - docs/tools/check-scripts-index.md
  - tools/checks/run_row_guard.sh
  - tools/checks/run_proof_app.sh
  - tools/checks/lib/manifest_runner.py
  - tools/checks/guard_rows.toml
  - tools/checks/proof_apps.toml
---

# Guard Manifest Migration SSOT

## Problem

`tools/checks/` has hundreds of stable shell entrypoints. Many proof app
guards and app-local `test.sh` files repeat the same shape:

```text
repo-root discovery
guard/proof id
file existence checks
local-run proof command
VM/MIR/EXE proof route
no gate-growth / no backend matcher checks
```

The stable shell names are useful for humans, docs, and existing scripts, but
the repeated command routing is now large enough to drift.

## Decision

Keep stable shell entrypoints, but move shared selection/routing to manifest
entries:

```text
tools/checks/run_row_guard.sh
  -> tools/checks/guard_rows.toml
  -> tools/checks/lib/manifest_runner.py

tools/checks/run_proof_app.sh
  -> tools/checks/proof_apps.toml
  -> tools/checks/lib/manifest_runner.py
```

`tools/checks/lib/manifest_runner.py` owns TOML parsing, entry validation,
selection by id/profile, dry-run/list output, and argv-array subprocess
dispatch. Shell wrappers must remain thin.

## Migration Rules

- Do not delete or rename existing guard entrypoints in a cleanup row.
- Convert app-local `test.sh` files first; they should delegate to
  `tools/checks/run_proof_app.sh --only <id>`.
- Convert `k2_wide_*_guard.sh` files later by family. During migration a guard
  may stay as the command owned by a manifest entry.
- Do not wire manifest pilot profiles into `dev_gate.sh` or allocator-wide by
  default until a separate closeout row accepts that policy.
- New simple proof apps should be manifest-backed before adding more local
  one-off routing.

## Current Schema

`tools/checks/proof_apps.toml` owns proof app routing:

```toml
[[proof_apps]]
id = "MIMAP-085A"
app = "apps/hako-alloc-segment-page-membership-scalar-proof"
label = "segment page membership scalar proof"
profiles = ["pilot", "hako-alloc-purge"]
cmd = ["bash", "tools/checks/k2_wide_hako_alloc_segment_page_membership_scalar_guard.sh"]
```

`tools/checks/guard_rows.toml` owns row guard routing:

```toml
[[rows]]
id = "current-state-pointer"
label = "current state pointer guard"
profiles = ["pilot", "quick-static"]
cmd = ["bash", "tools/checks/current_state_pointer_guard.sh"]
```

## No-Growth Contract

For manifest-backed proof apps:

```text
apps/<proof>/test.sh:
  must be executable
  must call tools/checks/run_proof_app.sh --only <id>
  must not directly call k2_wide_*_guard.sh
```

The guard for this contract is:

```text
tools/checks/proof_app_manifest_test_entry_guard.sh
```

## Stop Lines

- no all-at-once deletion of hundreds of guard entrypoints
- no shell `eval`
- no `shell=True` subprocess dispatch
- no hidden non-manifest proof app routing for manifest-backed apps
- no allocator behavior changes
- no compiler acceptance changes

