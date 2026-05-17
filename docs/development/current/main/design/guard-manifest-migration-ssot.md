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
  -> tools/checks/impl/<stable-command>.sh when a public guard wrapper is
     manifest-backed

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
  may stay as the command owned by a manifest entry. When making a public
  guard a thin wrapper, keep the public path at `tools/checks/k2_wide_*.sh` and
  move the thick body to `tools/checks/impl/`.
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

For manifest-backed public `k2_wide_*` wrappers:

```text
tools/checks/k2_wide_*_guard.sh:
  must be executable
  must call tools/checks/run_row_guard.sh --only <id>
  must not embed guard_common / rg / python / mktemp guard bodies

tools/checks/guard_rows.toml:
  cmd must point at tools/checks/impl/<stable-command>.sh
```

The guard for the first selected wrapper family is:

```text
tools/checks/k2_wide_manifest_wrapper_guard.sh
```

For the hako_alloc closeout cleanup burst, the no-growth source of truth is the
manifest profile, not a second hard-coded wrapper list:

```text
tools/checks/guard_rows.toml rows with profiles += ["hako-alloc-closeout"]
  -> cmd must be bash tools/checks/impl/<public-wrapper-name>
  -> public wrapper is derived as tools/checks/<public-wrapper-name>
```

`tools/checks/k2_wide_manifest_wrapper_guard.sh` derives the expected public
wrappers from that profile, rejects public hako_alloc closeout wrappers that are
not manifest-backed, and keeps the public wrapper body thin.

## Batch Migration Inventory

`GUARD-MANIFEST-012` fixes the next cleanup shape: do not hand-migrate hundreds
of public guards one wrapper at a time without an inventory owner.

The stable inventory owner is:

```text
tools/checks/guard_manifest_inventory.py
```

It reads `tools/checks/guard_rows.toml` and the `tools/checks/` filesystem
without executing any guards. It reports:

```text
guard_rows
top_level_check_sh
public_k2_wide
impl_sh
hako_alloc_closeout_rows
manifest_backed_hako_alloc_closeout_wrappers
non_manifest_hako_alloc_closeout_wrappers
missing_manifest_hako_alloc_closeout_wrappers
profile_*_rows
```

The inventory row guard is:

```text
tools/checks/guard_manifest_inventory_guard.sh
```

It must keep this contract true:

```text
non_manifest_hako_alloc_closeout_wrappers=0
missing_manifest_hako_alloc_closeout_wrappers=0
```

This is intentionally an inventory/no-growth row, not the full declarative guard
spec generator. A later row may introduce a declarative guard spec for one guard
family, but it must consume this inventory instead of starting from an ad hoc
file list.

## Stop Lines

- no all-at-once deletion of hundreds of guard entrypoints
- no shell `eval`
- no `shell=True` subprocess dispatch
- no hidden non-manifest proof app routing for manifest-backed apps
- no allocator behavior changes
- no compiler acceptance changes
