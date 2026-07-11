---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-183
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_page_model_box.hako
---

# 294x-185 Hako Alloc Usize Huge Model Size Observer Selection

## Decision

Select these `HakoAllocHugePageModel` size observers as
`HAKO-ALLOC-USIZE-FIELD-GROUP-184`:

- `last_requested_size`
- `last_committed_size`

These fields are reset to `0`, remain `0` on reject paths, and are assigned
from accepted positive huge allocation request/commit sizes on success. They
are consumed by the huge/OSVM comparison slice, so migrating them improves the
comparison-quality vertical slice without widening pointer, id, status, or
provider seams.

## Stop Line

Do not migrate:

- `HakoAllocHugePageModel.next_ptr`;
- `HakoAllocHugePageModel.last_result_ptr`;
- `HakoAllocHugePageModel.last_page_id`;
- `HakoAllocHugePageModel.last_failure_kind`;
- `HakoAllocHugeReleaseSeam.last_requested_size`;
- `HakoAllocHugeReleaseSeam.last_committed_size`;
- facade route/report mirrors;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-184` should migrate only the selected two fields
and update the huge-page model / huge-OSVM comparison guards to assert exact
`usize` storage.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
