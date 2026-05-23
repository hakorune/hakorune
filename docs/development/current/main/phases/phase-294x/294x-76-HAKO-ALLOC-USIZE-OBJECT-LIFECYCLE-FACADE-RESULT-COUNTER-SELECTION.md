---
Status: Landed
Date: 2026-05-23
Scope: select the next object-lifecycle facade source-owner exact `usize` row.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/object_lifecycle_facade_result_box.hako
  - lang/src/hako_alloc/memory/object_lifecycle_facade_stats_box.hako
---

# 294x-76 Hako Alloc Usize Object Lifecycle Facade Result Counter Selection

## Decision

Select the facade source-owner monotonic result counters in
`object_lifecycle_facade_result_box.hako` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-099`.

Chosen fields:

- `HakoAllocObjectLifecycleAllocResult.attempt_count`
- `HakoAllocObjectLifecycleAllocResult.success_count`
- `HakoAllocObjectLifecycleAllocResult.failure_count`
- `HakoAllocObjectLifecycleAllocResult.reusable_success_count`
- `HakoAllocObjectLifecycleAllocResult.active_success_count`
- `HakoAllocObjectLifecycleReleaseResult.success_count`
- `HakoAllocObjectLifecycleReleaseResult.failure_count`

These counters are owner-local, monotonic, and non-negative across the existing
small-alloc/release proofs. They publish the source facts consumed by the
facade getters and the later stats snapshot mirror, so the source owner lands
first and the downstream stats mirror stays its own later row.

## Stop Line

The follow-on row must not migrate:

- `last_page_id`, `last_block_id`, `last_reason`, or `last_ok`;
- any `HakoAllocObjectLifecycleAlignmentResult` field;
- any `HakoAllocObjectLifecycleReallocResult` field;
- `object_lifecycle_facade_stats_box.hako` snapshot counters or totals helpers;
- alignment requested/normalized observers, realloc ids/requested-size
  observers, page/block identity payloads, pointer-like fields, or unrelated
  OSVM/bin seams;
- provider activation, hooks, TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
