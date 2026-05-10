---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: allocator hook dry-run manifest callsite before any hook activation.
Related:
  - docs/development/current/main/design/allocator-hook-plan-v0.toml
  - docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
  - docs/development/current/main/design/allocator-hook-runtime-dry-run-ssot.md
---

# Allocator Hook Dry-Run Manifest Callsite (SSOT)

## Goal

Connect the reserved HookPlan and activation proof fixtures to the runtime
dry-run validator without activating allocator replacement.

M58 is a narrow diagnostic callsite row.

## Decision

The runtime callsite accepts manifest text from an explicit caller and returns a
diagnostic report. It does not read files, environment variables, or install
hooks.

Current function:

```text
validate_allocator_hook_dry_run_from_manifest_texts(plan_toml, proof_toml)
```

Current fixture owner:

```text
docs/development/current/main/design/allocator-hook-plan-v0.toml
docs/development/current/main/design/allocator-hook-activation-proof-v0.toml
```

## Contract

- Valid reserved plan + valid reserved proof -> `dry-run-ready`
- Missing/invalid plan -> `dry-run-missing-plan`
- Valid plan + missing/invalid proof -> `activation-proof-missing`
- Every report has `would_install = false`

## Stop Line

M58 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- file-system manifest discovery;
- `.inc` hook/facade/policy name matching;
- pointer `fetch_add`;
- OSVM unreserve/release;
- native pointer attr widening.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_dry_run_manifest_callsite_guard.sh
bash tools/checks/k2_wide_allocator_hook_runtime_dry_run_code_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
