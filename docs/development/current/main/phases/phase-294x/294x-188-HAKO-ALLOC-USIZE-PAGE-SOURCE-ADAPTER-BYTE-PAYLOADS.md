---
Status: Landed
Date: 2026-05-24
Scope: page-source adapter byte-length observer exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-186
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-187-HAKO-ALLOC-USIZE-PAGE-SOURCE-ADAPTER-BYTE-PAYLOAD-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako
  - lang/src/hako_alloc/memory/purge_page_source_recommit_adapter_box.hako
  - lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako
---

# 294x-188 Hako Alloc Usize Page-Source Adapter Byte Payloads

## Decision

Migrate only the selected page-source executor adapter byte-length observers to
exact `usize` storage:

- `HakoAllocPageSourceUnreserveAdapter.last_bytes`
- `HakoAllocPageSourceRecommitAdapter.last_bytes`
- `HakoAllocPageSourceDecommitAdapter.last_bytes`

The corresponding page-source policy methods already accept `bytes: usize`.

## Stop Line

This row does not migrate:

- `last_base` pointer-like payloads;
- `last_rc` status payloads;
- adapter counters already migrated in earlier rows;
- policy behavior, OSVM substrate behavior, provider activation, host
  replacement, hooks, global allocator install, worker/TLS, atomics, provider
  package / DLL generation, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh
bash tools/checks/k2_wide_hako_alloc_page_source_recommit_adapter_guard.sh
bash tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
