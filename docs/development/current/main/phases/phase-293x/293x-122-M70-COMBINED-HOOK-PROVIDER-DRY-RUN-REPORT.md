---
Status: Completed
Date: 2026-05-10
Scope: M70 combined hook/provider dry-run report.
Related:
  - docs/development/current/main/design/allocator-provider-combined-dry-run-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - src/runtime/allocator_provider_manifest.rs
  - src/cli/allocator_provider_manifest.rs
  - tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh
---

# 293x-122 M70 Combined Hook/Provider Dry-Run Report

## Summary

M70 adds one combined diagnostic report for explicit hook plan, activation
proof, and provider manifest inputs.

The report composes:

- hook dry-run diagnostics;
- activation proof diagnostics;
- activation preflight diagnostics;
- provider manifest diagnostics;
- provider readiness preflight diagnostics.

All action booleans remain false:

- `would_install = false`;
- `would_select_provider = false`;
- `would_activate = false`.

## Boundary

This card does not add provider selection, a provider registry, implicit
manifest discovery, hook activation, or process allocator replacement.

## Verification

```bash
cargo test -q allocator_provider
bash tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh
bash tools/checks/k2_wide_allocator_provider_readiness_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
