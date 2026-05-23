---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the page-source alloc-miss fallback counter closeout.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-83-HAKO-ALLOC-USIZE-OBJECT-LIFECYCLE-FACADE-PAGE-SOURCE-ALLOC-MISS-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_recommit_failfast_box.hako
  - tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh
---

# 294x-84 Hako Alloc Usize Recommit Failfast Counter Selection

## Decision

Select the owner-local monotonic counters in `HakoAllocRecommitFailFastEntry`
as `HAKO-ALLOC-USIZE-FIELD-GROUP-107`:

- `attempt_count`
- `no_recommit_count`
- `blocked_count`
- `missing_count`

These fields count local classification/report attempts in the recommit
fail-fast entry. They do not carry negative sentinels, pointer payloads, page
identity, or status vocabulary.

## Stop Line

This selection does not migrate:

- `HakoAllocRecommitFailFastReport` fields;
- `HakoAllocRecommitFailFastEntry.last_page_id`, because `-1` is the no-page
  sentinel;
- `recommit_execution_count` or `source_execution_count`, because they are
  closed-execution evidence for the recommit/source stop line;
- page-source attach report fields, alloc-miss report fields or count mirrors,
  huge-page-source / huge-failfast seams, OSVM byte/pointer payloads, provider /
  hook / global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
