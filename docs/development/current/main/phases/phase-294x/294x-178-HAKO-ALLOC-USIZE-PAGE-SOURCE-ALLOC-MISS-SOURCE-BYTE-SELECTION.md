---
Status: Landed
Date: 2026-05-24
Scope: select the next object-lifecycle facade page-source alloc-miss source byte mirror exact `usize` row.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-177
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-177-HAKO-ALLOC-USIZE-PAGE-SOURCE-ALLOC-MISS-SOURCE-MIRRORS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_page_source_alloc_miss_box.hako
  - tools/checks/k2_wide_mimalloc_facade_page_source_alloc_miss_exe_guard.sh
---

# 294x-178 Hako Alloc Usize Page-Source Alloc-Miss Source Byte Selection

## Decision

Select the alloc-miss report byte-length mirror from the page-source attach
report as `HAKO-ALLOC-USIZE-FIELD-GROUP-178`.

Chosen field in
`HakoAllocObjectLifecycleFacadePageSourceAllocMissReport`:

- `source_bytes`

This field mirrors `HakoAllocObjectLifecycleFacadePageSourceAttachReport.bytes`,
which is already exact `usize`. It is a byte-length payload and does not carry
status, reason, boolean, identity, or pointer semantics.

## Stop Line

The follow-on row must not migrate:

- alloc-miss report `status`;
- alloc-miss report `initial_*`, `retry_*`, or `final_*`;
- alloc-miss report `fallback_attempted`;
- alloc-miss report `source_status`;
- alloc-miss report `source_added_page_id`;
- alloc-miss report `source_base`;
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
