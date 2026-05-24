---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative production field group after page-heap requested bytes migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-005
Related:
  - lang/src/hako_alloc/memory/page_heap_box.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-256-HAKO-ALLOC-USIZE-PAGE-HEAP-REQUESTED-BYTES.md
---

# 294x-257 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-005
```

Select the legacy handle requested-size payload as the next field group:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-257-PAGE-HEAP-HANDLE-REQUESTED-SIZE-001
```

Field:

- `HakoAllocHandle.requested_size`

## Why This Group

`HakoAllocHandle.requested_size` is a non-negative size payload carried by
object-return and result-contract proof apps. It is narrower than page id,
block id, block-size, capacity, or stack-top migration because it does not
carry identity, index, or sentinel semantics.

The source request remains current-lane signed at method entry. This row only
migrates the stored handle payload and relies on the existing allocation /
realloc guards to reject non-positive requests before a live handle is
returned.

## Stop Line

The migration row must not change:

- `HakoAllocHandle.page_id` or `block_id`;
- `HakoAllocPage.page_id`, `block_size`, `capacity`, or `free_top`;
- `HakoAllocPage.requested_sizes` array payload semantics;
- method parameter types for `allocate`, `resizeInPlace`, `realloc`, or
  `reallocResult`;
- production `HakoAllocPageModel` fields;
- page-map entries, pointer-like payloads, provider/DLL seams, hooks,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-257-PAGE-HEAP-HANDLE-REQUESTED-SIZE-001
```

Expected code change:

- `HakoAllocHandle.requested_size: usize`

Expected verification should include the object-return and result-contract
proof apps because they print the handle requested-size field.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
