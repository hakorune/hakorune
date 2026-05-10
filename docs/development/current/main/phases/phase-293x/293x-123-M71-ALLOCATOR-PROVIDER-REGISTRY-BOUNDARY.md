---
Status: Completed
Date: 2026-05-10
Scope: M71 allocator provider registry boundary docs.
Related:
  - docs/development/current/main/design/allocator-provider-registry-boundary-ssot.md
  - docs/development/current/main/design/allocator-provider-current-task-breakdown-ssot.md
  - tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh
---

# 293x-123 M71 Allocator Provider Registry Boundary

## Summary

M71 names the future allocator provider registry owner and API shape without
adding registry code.

The future owner path is documented as:

```text
src/runtime/allocator_provider_registry.rs
```

That file remains absent in this card.

## Boundary

This card does not add provider selection, provider environment toggles,
implicit manifest discovery, runtime hook activation, or process allocator
replacement.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh
bash tools/checks/k2_wide_allocator_provider_combined_dry_run_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
