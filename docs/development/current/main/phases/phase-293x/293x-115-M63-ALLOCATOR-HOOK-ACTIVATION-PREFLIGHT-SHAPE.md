---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M63 allocator hook activation preflight data shape.
Related:
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - src/runtime/allocator_hook_dry_run.rs
  - tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh
---

# 293x-115 M63 Allocator Hook Activation Preflight Shape

## Goal

Implement the M62 activation preflight boundary as diagnostic-only runtime data.

## Result

Runtime now exposes:

- `AllocatorHookActivationPreflightFacts`
- `AllocatorHookActivationPreflightReport`
- `validate_allocator_hook_activation_preflight(...)`
- `validate_allocator_hook_activation_preflight_from_manifest_texts(...)`

The validator reports stable missing fact names and always returns
`would_activate = false`.

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
bash tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
