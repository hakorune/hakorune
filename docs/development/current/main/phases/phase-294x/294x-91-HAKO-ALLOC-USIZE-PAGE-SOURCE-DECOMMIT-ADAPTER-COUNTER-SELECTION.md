---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the page-source recommit adapter counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-89-HAKO-ALLOC-USIZE-PAGE-SOURCE-RECOMMIT-ADAPTER-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_page_source_decommit_adapter_box.hako
  - tools/checks/k2_wide_hako_alloc_page_source_decommit_adapter_guard.sh
---

# 294x-91 Hako Alloc Usize Page Source Decommit Adapter Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocPageSourceDecommitAdapter` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-113`:

- `call_count`
- `success_count`
- `reject_count`

These fields count adapter invocations and outcomes only. They do not carry
OSVM base pointers, byte lengths, status codes, heap/page identity, or bounded
decommit policy state.

## Stop Line

This selection does not migrate:

- `last_base`, because it is pointer-like OSVM payload;
- `last_bytes`, because it is byte-length payload tied to the OSVM decommit
  call;
- `last_rc`, because it is status vocabulary;
- bounded decommit policy state, purge inventory state, heap/page mutation,
  facade page-source seams, OSVM-backed fast-path owners, provider / hook /
  global-allocator rows, TLS, atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
