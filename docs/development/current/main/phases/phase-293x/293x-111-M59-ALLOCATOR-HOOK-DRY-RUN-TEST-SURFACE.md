---
Status: Done
Decision: accepted
Date: 2026-05-10
Scope: M59 allocator hook dry-run test surface
---

# 293x-111 M59 Allocator Hook Dry-Run Test Surface

## Decision

`M59 allocator hook dry-run test surface` is live-narrow.

M59 adds a `#[cfg(test)]` reserved-fixture observation helper. It does not add a
CLI flag, environment variable, file-system discovery, or process allocator
replacement.

## Owned

- `validate_allocator_hook_dry_run_reserved_fixtures_for_test()`
- Coverage guard:
  `tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh`

## Not Owned

- CLI hook command.
- Runtime hook install/uninstall implementation.
- Process allocator replacement.
- `#[global_allocator]`.
- Hook environment variables.
- `.inc` hook/facade/policy name matching.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh
bash tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh
cargo test -q allocator_hook_dry_run
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

The guard verifies:

- the reserved-fixture helper is test-only;
- no CLI flag, env toggle, runtime file discovery, or allocator install appears;
- dry-run tests still return `would_install = false`.

## Result

Result on 2026-05-10:
`k2_wide_allocator_hook_dry_run_test_surface_guard.sh` passes.
