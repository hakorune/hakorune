---
Status: Completed
Date: 2026-05-10
Scope: M78 allocator provider selection decision diagnostic shape.
Related:
  - docs/development/current/main/design/allocator-provider-selection-decision-ssot.md
  - docs/development/current/main/design/allocator-provider-selection-decision-v0.toml
  - tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh
---

# 293x-130 M78 Allocator Provider Selection Decision

## Summary

M78 adds the reserved selection request/decision diagnostic shape.

The fixture fixes the data contract between a registry snapshot and a future
selection implementation:

```text
requested_provider_id
required_operations
candidate_provider_ids
selection_policy
selection_status
selected_provider_id
```

## Boundary

This card does not add runtime provider registry code, provider selection
implementation, provider selection CLI/env toggles, implicit manifest
discovery, hook activation, provider proof bundle consumption,
`#[global_allocator]`, `GlobalAlloc`, or process allocator replacement.

The selection decision remains reserved:

```text
provider_selection = "inactive"
selection_status = "reserved_no_selection"
selected_provider_id = "none_reserved"
would_build_registry = false
would_select_provider = false
would_activate = false
activation = "future_row_required"
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_selection_decision_guard.sh
bash tools/checks/k2_wide_allocator_provider_registry_snapshot_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
