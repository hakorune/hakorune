---
Status: Completed
Date: 2026-05-10
Scope: M77 allocator provider registry snapshot diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-registry-snapshot-ssot.md
  - docs/development/current/main/design/allocator-provider-registry-snapshot-v0.toml
  - tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh
---

# 293x-129 M77 Allocator Provider Registry Snapshot

## Summary

M77 adds the reserved registry snapshot diagnostic shape.

The fixture fixes the provider entry shape copied from the provider manifest:

```text
provider_id
provider_kind
role
operations
state
activation
```

## Boundary

This card does not add runtime provider registry code, provider selection,
provider environment toggles, implicit manifest discovery, runtime hook
activation, `#[global_allocator]`, `GlobalAlloc`, or process allocator
replacement.

The registry snapshot remains reserved:

```text
provider_selection = "inactive"
would_build_registry = false
would_select_provider = false
would_activate = false
activation = "future_row_required"
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh
bash tools/checks/k2_wide_allocator_provider_activation_entry_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
