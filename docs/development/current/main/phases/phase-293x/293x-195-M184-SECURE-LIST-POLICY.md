---
Status: Complete
Date: 2026-05-12
Scope: M184 secure-list encode/decode small path.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/secure_free_list_policy_box.hako
  - apps/mimalloc-secure-list-encode-decode-proof/main.hako
---

# 293x-195 M184 Secure-List Policy

## Goal

Add the first secure-list encoded-next policy as a standalone small owner.

M184 owns:

```text
next index + caller-provided cookie
  -> encoded next integer
  -> decoded next index
  -> capacity validation
```

`-1` remains the valid end-of-list sentinel. `-2` is the policy-local invalid
decode/encode result.

## Stop Line

M184 does not mutate page state, source entropy, claim cryptographic strength,
change diagnostics ownership, install hooks, replace allocators, call OS
release, or add `.inc` allocator-name matching.

M185 owns the post-secure-list numeric field inventory.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_secure_list_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
