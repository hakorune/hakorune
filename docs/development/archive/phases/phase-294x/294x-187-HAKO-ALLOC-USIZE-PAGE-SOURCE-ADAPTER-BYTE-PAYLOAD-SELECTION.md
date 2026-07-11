---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-185
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako
  - lang/src/hako_alloc/memory/purge_page_source_recommit_adapter_box.hako
  - lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako
---

# 294x-187 Hako Alloc Usize Page-Source Adapter Byte Payload Selection

## Decision

Select the page-source executor adapter byte-length observers as
`HAKO-ALLOC-USIZE-FIELD-GROUP-186`:

- `HakoAllocPageSourceUnreserveAdapter.last_bytes`
- `HakoAllocPageSourceRecommitAdapter.last_bytes`
- `HakoAllocPageSourceDecommitAdapter.last_bytes`

Each owner observes the `bytes` payload passed to a `HakoAllocPageSourcePolicy`
method whose byte parameter is already exact `usize`.

## Stop Line

Do not migrate:

- `last_base` pointer-like payloads;
- `last_rc` status payloads;
- adapter owner-local counters already migrated in earlier rows;
- policy behavior, OSVM substrate behavior, provider activation, host
  replacement, hooks, global allocator install, worker/TLS, atomics, provider
  package / DLL generation, or `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-186` should migrate only the selected three
`last_bytes` fields and update the three page-source adapter guards to assert
exact `usize` storage for byte-length observers while keeping `last_base` and
`last_rc` signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
