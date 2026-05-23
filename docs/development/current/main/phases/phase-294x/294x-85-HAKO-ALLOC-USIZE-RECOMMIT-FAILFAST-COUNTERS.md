---
Status: Landed
Date: 2026-05-23
Scope: recommit fail-fast entry owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-84-HAKO-ALLOC-USIZE-RECOMMIT-FAILFAST-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_recommit_failfast_box.hako
  - apps/hako-alloc-recommit-failfast-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh
---

# 294x-85 Hako Alloc Usize Recommit Failfast Counters

## Decision

Migrate only the selected `HakoAllocRecommitFailFastEntry` owner-local
monotonic counters to exact `usize` storage:

- `attempt_count`
- `no_recommit_count`
- `blocked_count`
- `missing_count`

The M201 recommit fail-fast guard now asserts these four fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `HakoAllocRecommitFailFastReport` fields;
- `last_page_id`, because `-1` remains the no-page sentinel;
- `recommit_execution_count` or `source_execution_count`, because those fields
  remain closed-execution evidence and must stay signed until actual recommit /
  source execution opens in its own row;
- page-source attach report fields, alloc-miss report fields or count mirrors,
  huge-page-source / huge-failfast seams, OSVM byte/pointer payloads, provider /
  hook / global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_recommit_failfast_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
