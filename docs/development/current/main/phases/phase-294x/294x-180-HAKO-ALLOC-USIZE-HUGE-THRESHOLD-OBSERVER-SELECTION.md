---
Status: Landed
Date: 2026-05-24
Scope: select the next huge-threshold router size observer exact `usize` row.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-179
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-179-HAKO-ALLOC-USIZE-PAGE-SOURCE-ALLOC-MISS-SOURCE-BYTE.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/huge_threshold_router_box.hako
  - tools/checks/k2_wide_mimalloc_huge_threshold_routing_guard.sh
---

# 294x-180 Hako Alloc Usize Huge-Threshold Observer Selection

## Decision

Select the huge-threshold router non-negative size observer fields as
`HAKO-ALLOC-USIZE-FIELD-GROUP-180`.

Chosen fields in `HakoAllocHugeThresholdRouter`:

- `last_padded_size`
- `last_good_size`
- `last_huge_threshold`

These fields are reset to `0` or assigned from validated size-class /
threshold calculations. They do not carry route-kind, pointer, status, reason,
or identity semantics.

## Stop Line

The follow-on row must not migrate:

- `last_route_kind`;
- `last_result_ptr`;
- aligned-small path observer fields;
- huge page model, release, unreserve, unregister, decommit, or page-source
  report fields;
- provider activation, host replacement, hooks, global allocator install,
  worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
