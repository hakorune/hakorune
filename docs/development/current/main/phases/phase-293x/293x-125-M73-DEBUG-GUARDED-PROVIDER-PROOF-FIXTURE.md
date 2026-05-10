---
Status: Completed
Date: 2026-05-10
Scope: M73 debug guarded provider proof fixture.
Related:
  - docs/development/current/main/design/allocator-provider-debug-guarded-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-debug-guarded-proof-v0.toml
  - tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh
---

# 293x-125 M73 Debug Guarded Provider Proof Fixture

## Summary

M73 adds a reserved proof fixture for `debug_guarded_allocator`.

The fixture describes the guarded diagnostic provider proof vocabulary for:

```text
alloc
realloc
free
guard_check
leak_check
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
bash tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_hako_model_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
