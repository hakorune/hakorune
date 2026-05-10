---
Status: Completed
Date: 2026-05-10
Scope: M75 native mimalloc provider proof boundary.
Related:
  - docs/development/current/main/design/allocator-provider-native-mimalloc-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-mimalloc-proof-v0.toml
  - tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh
---

# 293x-127 M75 Native Mimalloc Provider Proof Boundary

## Summary

M75 adds a reserved proof boundary for `native_mimalloc`.

The fixture describes the native mimalloc provider proof vocabulary for:

```text
alloc
realloc
free
page_reserve
page_commit
page_decommit
```

## Boundary

This card does not add provider selection, provider environment toggles,
implicit manifest discovery, production activation, runtime hook activation,
`#[global_allocator]`, `GlobalAlloc`, or process allocator replacement.

The proof remains diagnostic/reserved only:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_native_mimalloc_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
