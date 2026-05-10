---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M64 allocator provider boundary vocabulary.
Related:
  - docs/development/current/main/design/allocator-provider-boundary-v0-ssot.md
  - docs/development/current/main/design/allocator-hook-activation-preflight-shape-ssot.md
  - tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh
---

# 293x-116 M64 Allocator Provider Boundary Vocabulary

## Goal

Name the allocator provider boundary below `hako_alloc` policy/state before any
process allocator replacement work.

## Result

M64 reserves these provider ids:

- `native_system_malloc`
- `native_mimalloc`
- `hako_model_allocator`
- `debug_guarded_allocator`

They are docs vocabulary only in this row. No runtime provider registry,
provider CLI, environment toggle, `.inc` matcher, or process allocator
replacement is added.

## Non-Goals

This card does not add:

- runtime provider registry;
- provider selection CLI;
- provider environment variables;
- runtime hook install/uninstall behavior;
- process allocator replacement;
- `#[global_allocator]`;
- implicit manifest discovery;
- `.inc` hook/provider/facade/policy name matching;
- allocator activation route widening.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh
bash tools/checks/k2_wide_allocator_hook_activation_preflight_shape_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
