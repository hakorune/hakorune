---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M58 allocator hook dry-run manifest callsite
---

# 293x-110 M58 Allocator Hook Dry-Run Manifest Callsite

## Decision

`M58 allocator hook dry-run manifest callsite` is live-narrow.

M58 connects the reserved HookPlan and activation proof TOML fixtures to the
runtime dry-run validator as explicit manifest text. It does not read files or
environment variables and never installs or replaces the process allocator.

## Owned

- `validate_allocator_hook_dry_run_from_manifest_texts(...)`
- unit tests for valid fixture, missing plan, and missing proof cases.
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh`

## Not Owned

- Runtime hook install/uninstall implementation.
- Process allocator replacement.
- `#[global_allocator]`.
- Hook environment variables.
- File-system manifest discovery.
- `.inc` hook/facade/policy name matching.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh
cargo test -q allocator_hook_dry_run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- runtime callsite is text-input only;
- docs fixtures remain reserved/inactive;
- valid fixtures produce a ready diagnostic with `would_install = false`;
- missing plan/proof cases fail fast to diagnostic status;
- no env/file-system discovery, allocator install code, or `.inc` name matching
  is introduced.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh` passes.
