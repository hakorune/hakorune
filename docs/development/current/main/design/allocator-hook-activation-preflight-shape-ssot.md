---
Status: SSOT
Decision: accepted
Date: 2026-05-10
Scope: diagnostic-only allocator hook activation preflight data shape.
Related:
  - docs/development/current/main/design/allocator-hook-activation-preflight-ssot.md
  - src/runtime/allocator_hook_dry_run.rs
---

# Allocator Hook Activation Preflight Shape (SSOT)

## Goal

Make the activation preflight boundary executable as diagnostic data without
activating allocator replacement.

## Decision

Runtime owns this diagnostic shape:

```text
AllocatorHookActivationPreflightFacts
AllocatorHookActivationPreflightReport
validate_allocator_hook_activation_preflight(...)
validate_allocator_hook_activation_preflight_from_manifest_texts(...)
```

The report always returns `would_activate = false`.

## Fact Vocabulary

The preflight report tracks missing fact names individually:

- `dry_run_ready`
- `activation_proof_ready`
- `reentrancy_guard_named`
- `bootstrap_allocation_path_named`
- `no_alloc_no_safepoint_contract_named`
- `rollback_condition_named`
- `fail_fast_diagnostic_named`

## Diagnostic Contract

- All facts present returns `[allocator-hook/activation-preflight-ready]`.
- Missing facts return `[allocator-hook/activation-preflight-missing]`.
- `missing_facts` lists stable fact names.
- A ready preflight is still diagnostic-only.
- No runtime hook install/uninstall behavior is introduced.

## Manifest Text Contract

`validate_allocator_hook_activation_preflight_from_manifest_texts(plan, proof)`
derives:

- `dry_run_ready` from the dry-run validator;
- `activation_proof_ready` from the activation-proof validator;
- named proof facts from the proof TOML `required_proofs` array.

The function accepts only caller-provided text. It does not read files, inspect
environment variables, or search default locations.

## Stop Line

M63 keeps these inactive:

- runtime hook install/uninstall body;
- process allocator replacement;
- `#[global_allocator]`;
- hook environment toggles;
- implicit runtime file-system manifest discovery;
- `.inc` hook/facade/policy name matching;
- route widening for allocator activation.

## Gate

```bash
bash tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_preflight_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
