---
Status: Complete
Date: 2026-05-12
Scope: M176 `.hako` mimalloc realloc negative matrix / failure contract.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - lang/src/hako_alloc/memory/page_map_realloc_same_class_box.hako
  - lang/src/hako_alloc/memory/page_map_realloc_alloc_copy_release_box.hako
  - lang/src/hako_alloc/memory/page_map_realloc_failure_contract_box.hako
---

# 293x-186 M176 Realloc Negative Matrix / Failure Contract

## Goal

Freeze the public realloc failure matrix without widening the underlying realloc
owners.

M176 adds a diagnostics wrapper over M174 and M175:

```text
requested_size <= 0
  -> zero reject
requested_size > max page block size
  -> oversized reject
otherwise
  -> try M174 same-class/no-move
  -> if grow-only reject, try M175 alloc-copy-release
  -> classify alloc-fail / released / stale / unknown explicitly
```

This row keeps the failure kinds explicit. It does not change release ordering,
copy modeling, aligned allocation, or huge-page routing.

## Stop Line

M176 does not implement byte copy, alignment, huge allocation, secure-list
hardening, OSVM release, provider activation, hook install, process allocator
replacement, `.inc` name matching, or production `usize` field migration.

The new owner may read page block sizes and delegate to M174/M175, but it must
not take over raw `register(...)`, `releasePtr(...)`, `unregister(...)`, or
`releaseLocal(...)` execution.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_realloc_failure_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
