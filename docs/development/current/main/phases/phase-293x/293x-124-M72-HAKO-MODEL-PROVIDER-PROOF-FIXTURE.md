---
Status: Completed
Date: 2026-05-10
Scope: M72 hako model provider proof fixture.
Related:
  - docs/development/current/main/design/allocator-provider-hako-model-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-hako-model-proof-v0.toml
  - tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh
---

# 293x-124 M72 Hako Model Provider Proof Fixture

## Summary

M72 adds a reserved proof fixture for `hako_model_allocator`.

The fixture describes the `.hako` policy/model provider proof vocabulary for:

```text
model_alloc
model_free
stats
stress_validate
```

## Boundary

This card does not add provider selection, provider environment toggles,
implicit manifest discovery, native metal activation, runtime hook activation,
or process allocator replacement.

The proof remains diagnostic/reserved only:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_registry_boundary_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
