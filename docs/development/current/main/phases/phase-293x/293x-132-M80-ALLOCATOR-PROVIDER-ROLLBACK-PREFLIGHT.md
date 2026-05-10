---
Status: Completed
Date: 2026-05-10
Scope: M80 allocator provider rollback preflight diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-rollback-preflight-ssot.md
  - docs/development/current/main/design/allocator-provider-rollback-preflight-v0.toml
  - tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh
---

# 293x-132 M80 Allocator Provider Rollback Preflight

## Summary

M80 adds the reserved allocator provider rollback preflight diagnostic shape.

The fixture fixes the data contract between proof bundle consumption data,
hook plan data, and a future rollback preparation implementation:

```text
preflight_input
activation_entry_input
registry_snapshot_input
selection_decision_input
proof_bundle_input
hook_plan_input
hook_activation_preflight_input
activation_proof_input
rollback_target_source
rollback_target_provider_id
rollback_status
rollback_preflight
```

## Boundary

This card does not add runtime provider registry code, provider selection
implementation, proof consumption implementation, rollback preparation
implementation, hook activation, hook activation CLI/env toggles, provider
rollback CLI/env toggles, implicit manifest/proof/hook-plan discovery,
`#[global_allocator]`, `GlobalAlloc`, process allocator replacement, or `.inc`
name matching.

The rollback preflight row remains reserved:

```text
active = false
rollback_preflight = "inactive"
rollback_status = "reserved_no_rollback"
provider_selection = "inactive"
proof_bundle_consumption = "inactive"
hook_activation = "inactive"
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_prepare_rollback = false
would_activate_hook = false
would_activate = false
activation = "future_row_required"
```

## Stable Diagnostics

```text
[allocator-provider/rollback-preflight-missing]
[allocator-provider/rollback-input-missing]
[allocator-provider/rollback-snapshot-missing]
[allocator-provider/rollback-selection-missing]
[allocator-provider/rollback-proof-bundle-missing]
[allocator-provider/rollback-hook-plan-missing]
[allocator-provider/rollback-activation-preflight-missing]
[allocator-provider/rollback-activation-proof-missing]
[allocator-provider/rollback-target-missing]
[allocator-provider/rollback-activation-blocked]
```

## Verification

```bash
bash -n tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh
bash tools/checks/k2_wide_allocator_provider_rollback_preflight_guard.sh
git diff --check
```
