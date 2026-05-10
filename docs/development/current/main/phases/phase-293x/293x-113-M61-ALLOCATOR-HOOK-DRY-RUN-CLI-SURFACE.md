---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M61 allocator hook dry-run CLI surface.
Related:
  - docs/development/current/main/design/allocator-hook-dry-run-cli-surface-ssot.md
  - src/cli/allocator_hook_dry_run.rs
  - tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh
---

# 293x-113 M61 Allocator Hook Dry-Run CLI Surface

## Goal

Expose the allocator hook dry-run validator through an explicit CLI diagnostic
surface.

## Result

The accepted CLI shape is:

```text
hakorune --allocator-hook-dry-run \
  --allocator-hook-plan <PLAN_TOML> \
  --allocator-hook-proof <PROOF_TOML>
```

The CLI reads only the explicit files supplied by the user, feeds their text to
the runtime dry-run validators, prints stable diagnostics, and exits before
normal program execution.

## Non-Goals

This card does not add:

- runtime hook install/uninstall behavior;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment variables;
- implicit manifest discovery;
- runner ownership;
- `.inc` hook/facade/policy name matching;
- allocator activation route widening.

## Verification

```bash
bash tools/checks/k2_wide_allocator_hook_dry_run_cli_surface_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
