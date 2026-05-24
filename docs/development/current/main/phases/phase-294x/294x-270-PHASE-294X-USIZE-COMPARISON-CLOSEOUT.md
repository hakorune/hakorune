---
Status: Landed
Date: 2026-05-24
Scope: close phase-294x's exact `usize` comparison-quality slice.
Blocker: PHASE-294X-USIZE-COMPARISON-CLOSEOUT-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-269-HAKO-ALLOC-USIZE-NEXT-FIELD-GROUP-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-268-MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh
---

# 294x-270 Phase 294x Usize Comparison Closeout

## Decision

Close:

```text
PHASE-294X-USIZE-COMPARISON-CLOSEOUT-001
```

Phase-294x has enough exact `usize` foundation and hako_alloc field migration
for the current comparison-quality vertical slice:

- exact numeric syntax / metadata / MIR facts / VM reference execution;
- exact typed-object storage and backend consumption;
- production facade-local stats;
- comparison-required owner-local counters and page-map / queue / result-ledger
  counters;
- legacy page-heap non-id size/count/capacity/requested-byte fields;
- refreshed mimalloc comparison vertical-slice evidence after the page-heap
  closeout.

Do not extend phase-294x to drain remaining allocator fields.

## Parked Categories

Keep the following categories parked until a later row explicitly selects their
semantic contract:

- page/handle ids and signed identity/index seams;
- negative sentinel-bearing fields;
- status/reason vocabularies and bool-like flags;
- report mirror payloads and presentation-only mirrors;
- pointer-like payloads;
- provider package / DLL generation, provider activation, provider API calls,
  host allocator replacement, hooks, backend matchers, worker/TLS, atomics,
  remote-free stress, abandoned heap stress, and `#[global_allocator]`.

## Evidence

The closeout reuses:

```text
tools/checks/k2_wide_hako_alloc_mimalloc_comparison_page_heap_usize_refresh_guard.sh
```

That guard composes:

- page-heap exact non-id `usize` closeout;
- V2/V3/V4/V5 mimalloc comparison vertical-slice closeout;
- route preflight and stable V5 evidence.

## Next Row

Select the next lane/row from:

```text
PHASE-294X-POST-CLOSEOUT-ROW-SELECTION-001
```

The expected direction is to resume mimalloc `.hako` port work from a comparison
or execution seam, not to keep migrating unrelated `usize` fields.

## Verification

```bash
bash tools/checks/k2_wide_phase294x_usize_comparison_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
