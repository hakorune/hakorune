---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M62 allocator hook activation preflight boundary.
Related:
  - docs/development/current/main/design/allocator-hook-activation-preflight-ssot.md
  - docs/development/current/main/design/allocator-hook-dry-run-cli-surface-ssot.md
  - tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh
---

# 293x-114 M62 Allocator Hook Activation Preflight

## Goal

Lock the proof handoff required before any allocator hook activation or process
allocator replacement can be implemented.

## Result

M62 names the required activation preflight facts:

- reentrancy guard;
- bootstrap allocation path;
- no-allocation / no-safepoint contract;
- rollback condition;
- fail-fast diagnostic;
- active hook plan row;
- active activation proof row.

The row is docs/guard only and keeps activation inactive.

## Non-Goals

This card does not add:

- runtime hook install/uninstall behavior;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment variables;
- implicit manifest discovery;
- `.inc` hook/facade/policy name matching;
- allocator activation route widening.

## Verification

```bash
bash tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh
bash tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
