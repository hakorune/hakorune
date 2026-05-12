---
Status: Complete
Date: 2026-05-12
Scope: M183 secure-list diagnostics-only.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/secure_free_list_diagnostics_box.hako
  - apps/mimalloc-secure-list-diagnostics-proof/main.hako
---

# 293x-194 M183 Secure-List Diagnostics

## Goal

Add diagnostics-only observers for page-local free-list shape before
encode/decode policy lands.

M183 owns observation only:

```text
free / local_free / block_used / free_top / local_free_top
  -> out-of-range
  -> duplicate
  -> live block in free-list
  -> count mismatch
```

## Stop Line

M183 does not implement encoded next pointers, cookies, randomness,
cryptographic hardening, allocator replacement, provider activation, hooks,
OS release, or `.inc` allocator-name matching.

M184 owns the first encode/decode small path.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_secure_list_diagnostics_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
