---
Status: Landed
Date: 2026-05-23
Scope: page-source unreserve adapter owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-86-HAKO-ALLOC-USIZE-PAGE-SOURCE-UNRESERVE-ADAPTER-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako
  - apps/hako-alloc-page-source-unreserve-adapter-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh
---

# 294x-87 Hako Alloc Usize Page Source Unreserve Adapter Counters

## Decision

Migrate only the selected `HakoAllocPageSourceUnreserveAdapter` owner-local
monotonic counters to exact `usize` storage:

- `call_count`
- `success_count`
- `reject_count`

The MIMAP-033A page-source unreserve adapter guard now asserts these fields are
exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `last_base`, because it is pointer-like OSVM payload;
- `last_bytes`, because it is byte-length payload tied to the OSVM unreserve
  call;
- `last_rc`, because it is status vocabulary;
- page-source policy state, facade huge unreserve owners, OSVM-backed fast-path
  owners, provider / hook / global-allocator rows, TLS, atomics, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
