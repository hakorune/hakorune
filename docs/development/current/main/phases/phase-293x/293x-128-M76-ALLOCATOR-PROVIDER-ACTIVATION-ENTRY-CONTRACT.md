---
Status: Completed
Date: 2026-05-10
Scope: M76 allocator provider activation entry contract.
Related:
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-ssot.md
  - docs/development/current/main/design/allocator-provider-activation-entry-contract-v0.toml
  - tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh
---

# 293x-128 M76 Allocator Provider Activation Entry Contract

## Summary

M76 adds the post-M75 activation entry contract.

The contract names the future owner paths and required facts for:

```text
registry/selection ownership
fail-fast selection diagnostics
activation proof consumption
native provider proof consumption
rollback behavior
```

## Boundary

This card does not add provider registry code, provider selection, provider
environment toggles, implicit manifest discovery, runtime hook activation,
`#[global_allocator]`, `GlobalAlloc`, or process allocator replacement.

The fixture remains reserved:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
activation = "future_row_required"
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh
bash tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
