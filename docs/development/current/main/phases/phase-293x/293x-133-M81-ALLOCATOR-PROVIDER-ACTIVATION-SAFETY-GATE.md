---
Status: Completed
Date: 2026-05-11
Scope: M81 allocator provider activation safety gate diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-safety-gate-v0.toml
  - tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh
---

# 293x-133 M81 Allocator Provider Activation Safety Gate

## Summary

M81 adds the reserved allocator provider activation safety gate diagnostic
shape.

The fixture fixes the data contract between all activation evidence reports and
a future runtime diagnostic implementation:

```text
activation_entry_input
provider_readiness_input
combined_dry_run_input
registry_snapshot_input
selection_decision_input
proof_bundle_input
rollback_preflight_input
hook_plan_input
hook_activation_preflight_input
activation_proof_input
activation_target_source
activation_target_provider_id
safety_status
activation_safety_gate
```

## Boundary

This card does not add runtime provider registry code, provider selection
implementation, proof consumption implementation, rollback preparation
implementation, activation safety gate implementation, hook activation, hook
activation CLI/env toggles, activation safety gate CLI/env toggles, implicit
manifest/proof/hook-plan discovery, `#[global_allocator]`, `GlobalAlloc`,
process allocator replacement, route widening, or `.inc` name matching.

The activation safety gate row remains reserved:

```text
active = false
activation_safety_gate = "inactive"
safety_status = "reserved_gate_closed"
provider_selection = "inactive"
proof_bundle_consumption = "inactive"
rollback_preflight = "inactive"
hook_activation = "inactive"
activation_gate_open = false
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_prepare_rollback = false
would_open_activation_gate = false
would_activate_hook = false
would_activate = false
activation = "future_row_required"
```

## Stable Diagnostics

```text
[allocator-provider/activation-safety-gate-missing]
[allocator-provider/activation-safety-entry-missing]
[allocator-provider/activation-safety-readiness-missing]
[allocator-provider/activation-safety-combined-dry-run-missing]
[allocator-provider/activation-safety-registry-missing]
[allocator-provider/activation-safety-selection-missing]
[allocator-provider/activation-safety-proof-bundle-missing]
[allocator-provider/activation-safety-rollback-missing]
[allocator-provider/activation-safety-hook-plan-missing]
[allocator-provider/activation-safety-preflight-missing]
[allocator-provider/activation-safety-proof-missing]
[allocator-provider/activation-safety-target-missing]
[allocator-provider/activation-safety-blocked]
```

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_safety_gate_guard.sh
git diff --check
```
