---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook dry-run test-only surface before CLI/env exposure.
Related:
  - docs/development/current/main/design/allocator-hook-dry-run-manifest-callsite-ssot.md
  - src/runtime/allocator_hook_dry_run.rs
---

# Allocator Hook Dry-Run Test Surface (SSOT)

## Goal

Expose one explicit test-only observation surface for allocator hook dry-run
without adding CLI flags, environment variables, or process allocator
replacement.

## Decision

The only current observation surface is:

```text
#[cfg(test)]
validate_allocator_hook_dry_run_reserved_fixtures_for_test()
```

This helper reads the reserved docs fixtures at compile time in test builds only
and returns the diagnostic dry-run report.

## Contract

- The helper is `#[cfg(test)]`.
- The helper returns `would_install = false`.
- Production code does not read fixture files.
- No CLI flag is added.
- No environment variable is added.

## Stop Line

M59 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- CLI hook flags;
- runtime file-system manifest discovery;
- `.inc` hook/facade/policy name matching.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh
bash tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
