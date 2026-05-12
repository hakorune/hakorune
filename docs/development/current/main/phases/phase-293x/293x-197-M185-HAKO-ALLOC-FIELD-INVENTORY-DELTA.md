---
Status: Complete
Date: 2026-05-12
Scope: M185 hako_alloc field inventory delta.
Related:
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 293x-197 M185 hako_alloc Field Inventory Delta

## Goal

Reconcile `lang/src/hako_alloc/memory/NUMERIC_FIELDS.md` with the post-M184
allocator owner set before any broader exact `usize` migration resumes.

M185 records:

- current production stored numeric field count: `215`
- production `usize` field group remains facade-local stats only
- observer/result sentinels that must stay signed
- secure-list policy has no stored numeric fields
- next migration target is owner-local, not global replacement

## Stop Line

M185 does not migrate stored fields, change `.hako` runtime semantics, add
backend lowering, or reinterpret `i64` page-map/handle/pointer ids as `usize`.

M187 owns the next actual exact numeric migration candidate.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_numeric_field_inventory_delta_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
