---
Status: Landed
Date: 2026-05-24
Scope: select the next object-lifecycle facade page-source attach report mirror exact `usize` row.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-171
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-81-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-PAGE-SOURCE-ATTACH-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
  - tools/checks/k2_wide_mimalloc_facade_page_source_fresh_page_exe_guard.sh
---

# 294x-172 Hako Alloc Usize Page-Source Attach Report Mirror Counter Selection

## Decision

Select the page-source attach report mirror counters in
`HakoAllocObjectLifecycleFacadePageSourceAttachReport` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-172`.

Chosen fields:

- `source_reserved`
- `source_committed`
- `facade_page_count`
- `source_reject`

These fields mirror non-negative page-source/facade count facts. They do not
carry sentinel values.

## Stop Line

The follow-on row must not migrate:

- attach report `status`;
- attach report `added_page_id`;
- attach report `base`;
- attach report `bytes`, `block_size`, `capacity`, or `reserved`;
- alloc-miss report source/final mirrors;
- page/block identity payloads;
- pointer-like fields;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
