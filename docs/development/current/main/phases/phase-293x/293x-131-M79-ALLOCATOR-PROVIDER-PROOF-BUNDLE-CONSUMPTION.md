---
Status: Completed
Date: 2026-05-10
Scope: M79 allocator provider proof bundle consumption diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-ssot.md
  - docs/development/current/main/design/allocator-provider-proof-bundle-consumption-v0.toml
  - tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh
---

# 293x-131 M79 Allocator Provider Proof Bundle Consumption

## Summary

M79 adds the reserved provider proof bundle consumption diagnostic shape.

The fixture fixes the data contract between a selection decision and a future
proof-consuming implementation:

```text
registry_snapshot_input
selection_decision_input
proof_bundle_source
requested_provider_id
selected_provider_id
provider_proof_inputs
consumption_status
proof_bundle_consumption
```

## Boundary

This card does not add runtime provider registry code, provider selection
implementation, proof bundle consumption implementation, provider selection or
proof bundle CLI/env toggles, implicit manifest/proof discovery, hook
activation, `#[global_allocator]`, `GlobalAlloc`, or process allocator
replacement.

The proof bundle consumption row remains reserved:

```text
proof_bundle_consumption = "inactive"
provider_selection = "inactive"
selection_status = "reserved_no_selection"
selected_provider_id = "none_reserved"
proof_bundle_consumed = false
would_build_registry = false
would_select_provider = false
would_consume_proof_bundle = false
would_activate = false
activation = "future_row_required"
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_proof_bundle_consumption_guard.sh
bash tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
