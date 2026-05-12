---
Status: Complete
Date: 2026-05-12
Scope: M173 `.hako` mimalloc pre-realloc release invariant freeze.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_release_box.hako
  - lang/src/hako_alloc/memory/page_map_release_invariant_box.hako
---

# 293x-183 M173 Pre-Realloc Release Invariant Freeze

## Goal

Freeze the release contract before any realloc body exists.

M172 already owns the release execution sequence:

```text
HakoAllocPageMap.lookup(ptr)
  -> HakoAllocPageModel.releaseLocal(block_id)
  -> HakoAllocPageMap.unregister(ptr)
```

M173 keeps that execution owner unchanged and adds a narrow observer that proves:

- successful release expires the handle after the seam advances release and
  unregister counts;
- released-block, stale-page, and unknown-pointer rejects keep the existing
  ownership state observable and do not fake a release/unregister delta;
- page-local `used` / `local_free` mutation is visible only on the success path.

## Stop Line

M173 does not implement realloc body, same-class/no-move policy, alloc-copy
fallback, byte copy, aligned allocation, huge allocation, secure-list
hardening, OSVM release, provider activation, hook install, process allocator
replacement, `.inc` name matching, or production `usize` field migration.

The new observer must not take over pointer registration, page release, or
unregister execution. Those owners stay in M171/M172.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_pre_realloc_release_invariant_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
