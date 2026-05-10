---
Status: Completed
Date: 2026-05-10
Scope: M74 native system provider proof boundary.
Related:
  - docs/development/current/main/design/allocator-provider-native-system-proof-ssot.md
  - docs/development/current/main/design/allocator-provider-native-system-proof-v0.toml
  - tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh
---

# 293x-126 M74 Native System Provider Proof Boundary

## Summary

M74 adds a reserved proof boundary for `native_system_malloc`.

The fixture describes the native system allocator provider proof vocabulary for:

```text
alloc
realloc
free
```

## Boundary

This card does not add provider selection, provider environment toggles,
implicit manifest discovery, native metal activation, runtime hook activation,
`#[global_allocator]`, `GlobalAlloc`, or process allocator replacement.

The proof remains diagnostic/reserved only:

```text
provider_selection = "inactive"
would_select_provider = false
would_activate = false
```

## Verification

```bash
bash tools/checks/k2_wide_allocator_provider_native_system_proof_guard.sh
bash tools/checks/k2_wide_allocator_provider_debug_guarded_proof_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
