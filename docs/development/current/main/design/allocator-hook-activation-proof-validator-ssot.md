---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook activation-proof validation before runtime activation.
Related:
  - docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
  - docs/development/current/main/design/allocator-hook-dry-run-manifest-callsite-ssot.md
  - src/runtime/allocator_hook_dry_run.rs
---

# Allocator Hook Activation-Proof Validator (SSOT)

## Goal

Validate allocator hook activation-proof TOML text as a diagnostic-only runtime
fact before any allocator hook activation path exists.

## Decision

The current validator surface is:

```text
validate_allocator_hook_activation_proof_text(activation_proof_toml, hook_id)
```

It accepts caller-provided TOML text and a caller-provided hook id. It does not
discover files, read environment variables, install hooks, or replace the
process allocator.

## Accepted Shape

A proof is diagnostic-ready only when all of these are true:

- `schema_version = "allocator_hook_activation_proof_v0"`
- `status = "reserved"`
- `active = false`
- `hook_id` matches the requested hook id
- `activation = "future_row_required"`
- every reserved `required_proofs` entry is present

Any missing, malformed, active, or wrong-hook proof returns the missing-proof
diagnostic.

## Contract

- Ready proof returns `[allocator-hook/activation-proof-ready]`.
- Missing or invalid proof returns `[allocator-hook/activation-proof-missing]`.
- `would_activate` is always `false`.
- The runtime file remains diagnostic-only.
- `.inc` does not infer allocator hook activation from symbol names.

## Stop Line

M60 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- CLI hook flags;
- runtime file-system manifest discovery;
- `.inc` hook/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_activation_proof_validator_guard.sh
bash tools/checks/k2_wide_allocator_hook_dry_run_test_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
