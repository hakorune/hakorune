---
Status: Completed
Decision: accepted
Date: 2026-05-10
Scope: M65 allocator provider manifest vocabulary.
Related:
  - docs/development/current/main/design/allocator-provider-manifest-v0-ssot.md
  - docs/development/current/main/design/allocator-provider-manifest-v0.toml
  - tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh
---

# 293x-117 M65 Allocator Provider Manifest Vocabulary

## Goal

Define a reserved provider manifest fixture for the M64 provider boundary.

## Result

M65 adds `allocator-provider-manifest-v0.toml` as docs vocabulary only. It
reserves rows for:

- `native_system_malloc`
- `native_mimalloc`
- `hako_model_allocator`
- `debug_guarded_allocator`

The manifest stays `active = false` and `provider_selection = "inactive"`.

## Non-Goals

This card does not add:

- runtime provider manifest parser;
- runtime provider registry;
- provider selection CLI;
- provider environment variables;
- runtime hook install/uninstall behavior;
- process allocator replacement;
- `#[global_allocator]`;
- implicit manifest discovery;
- `.inc` provider/hook/facade/policy name matching;
- allocator activation route widening.

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_manifest_vocab_guard.sh
bash tools/checks/k2_wide_allocator_provider_boundary_vocab_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
