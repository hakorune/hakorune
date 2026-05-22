---
Status: Landed
Date: 2026-05-23
Scope: select huge-page model next-page id exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
---

# 294x-68 Hako Alloc Usize Huge Model Next Page Id Selection

## Decision

Select `HakoAllocHugePageModel.next_page_id` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-091`.

`next_page_id` is the huge-page model's owner-local next page id source. It
starts at `1000`, is copied into page-map / metadata-store identity payloads,
and then increments after a successful huge allocation registration. Unlike the
OSVM-backed fast path, this field is not used as a `-1` failure sentinel source,
so the follow-on row can migrate the id source while keeping all published page
id payloads signed.

## Stop Line

The follow-on row must not migrate:

- `HakoAllocHugePageModel.next_ptr`;
- `HakoAllocHugePageModel.last_result_ptr`, `last_page_id`,
  `last_requested_size`, `last_committed_size`, or `last_failure_kind`;
- `HakoAllocHugePageMetaStore` page id / pointer / size payloads;
- page-map entry pointer/id fields or live flags;
- OSVM-backed `next_page_id`;
- provider activation, host allocator replacement, hooks, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
