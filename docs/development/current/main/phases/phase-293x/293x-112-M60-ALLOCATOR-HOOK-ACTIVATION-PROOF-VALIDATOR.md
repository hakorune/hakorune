---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M60 allocator hook activation-proof validator.
Related:
  - docs/development/current/main/design/allocator-hook-activation-proof-validator-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
  - src/runtime/allocator_hook_dry_run.rs
  - tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh
---

# 293x-112 M60 Allocator Hook Activation-Proof Validator

## Goal

Add a diagnostic-only validator for allocator hook activation-proof TOML text.

## Result

`src/runtime/allocator_hook_dry_run.rs` now exposes
`validate_allocator_hook_activation_proof_text(activation_proof_toml, hook_id)`.

The validator:

- accepts TOML text and a requested hook id from the caller;
- checks the reserved schema/status/inactive/hook/activation facts;
- checks every reserved `required_proofs` entry;
- returns a ready diagnostic for valid proof text;
- returns the missing-proof diagnostic for malformed, missing, active, or
  wrong-hook proof text;
- always returns `would_activate = false`.

## Non-Goals

This card does not add:

- runtime hook install/uninstall behavior;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment variables;
- CLI hook flags;
- runtime file discovery;
- `.inc` hook/facade/policy name matching;
- allocator activation route widening.

## Verification

```bash
bash tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh
bash tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
