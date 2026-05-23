---
Status: Landed
Date: 2026-05-23
Scope: page-source decommit adapter owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-91-HAKO-ALLOC-USIZE-PAGE-SOURCE-DECOMMIT-ADAPTER-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako
  - apps/hako-alloc-page-source-decommit-adapter-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh
---

# 294x-92 Hako Alloc Usize Page Source Decommit Adapter Counters

## Decision

Migrate only the selected `HakoAllocPageSourceDecommitAdapter` owner-local
monotonic counters to exact `usize` storage:

- `call_count`
- `success_count`
- `reject_count`

The M196 page-source decommit adapter guard now asserts these fields are exact
`usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `last_base`, because it is pointer-like OSVM payload;
- `last_bytes`, because it is byte-length payload tied to the OSVM decommit
  call;
- `last_rc`, because it is status vocabulary;
- bounded decommit policy state, purge inventory state, heap/page mutation,
  facade page-source seams, OSVM-backed fast-path owners, provider / hook /
  global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
