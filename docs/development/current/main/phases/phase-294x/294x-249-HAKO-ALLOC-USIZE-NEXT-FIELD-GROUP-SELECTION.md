---
Status: Landed
Date: 2026-05-24
Scope: select the next explicit non-negative `hako_alloc` production field group after Stage-B parser-front alignment.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-001
Related:
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/phases/phase-294x/294x-248-STAGEB-PARSER-FIELD-TYPE-ANNOTATION-ALIGNMENT.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-249 Hako Alloc Usize Next Field Group Selection

## Decision

Close `HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-001`.

Select `HakoAllocObjectLifecycleReallocResult.last_requested_size` as the next
explicit non-negative production field group.

Selected next blocker:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-249-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER-001
```

## Why This Group

`last_requested_size` is a requested-size observer owned by
`HakoAllocObjectLifecycleReallocResult`.

It is a narrow exact `usize` candidate because:

- it is a size payload, not an id/index/pointer;
- it is initialized to `0`, not `-1`;
- it is written from realloc requested-size inputs;
- it is read through a scalar observer method;
- it can migrate without changing page/block identity, reason vocabulary, or
  success flags.

## Stop Line

The migration row must not change:

- `last_page_id`, `last_block_id`, `last_new_page_id`, or `last_new_block_id`;
- `last_reason` or `last_ok`;
- `HakoAllocObjectLifecycleAlignmentResult`;
- huge model / huge release requested-size fields;
- mimalloc comparison schemas, provider activation, hooks, DLL packaging,
  worker/TLS, atomics, or `#[global_allocator]`.

## Next Row

Implement:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-249-REALLOC-REQUESTED-SIZE-RESULT-OBSERVER-001
```

Expected code change:

- `HakoAllocObjectLifecycleReallocResult.last_requested_size: usize = 0`

Expected metadata/doc updates:

- update `NUMERIC_FIELDS.md`;
- add a narrow guard that verifies the field is exact `usize` and nearby
  sentinel/status fields remain signed;
- update current pointers only after the migration row lands.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
