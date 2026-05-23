---
Status: Landed
Date: 2026-05-23
Scope: select the next owner-local production exact `usize` field group after the recommit-side purge marker counter migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-96-HAKO-ALLOC-USIZE-PURGE-RECOMMIT-MARKER-COUNTERS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_bounded_decommit_box.hako
  - tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh
---

# 294x-97 Hako Alloc Usize Bounded Decommit Policy Counter Selection

## Decision

Select the owner-local monotonic counters in
`HakoAllocBoundedDecommitPolicy` as
`HAKO-ALLOC-USIZE-FIELD-GROUP-119`:

- `attempt_count`
- `blocked_count`
- `decommit_attempt_count`
- `decommit_success_count`
- `source_reject_count`

These fields count M195 bounded decommit policy attempts and outcomes. They do
not carry byte bounds, page-source payload, status vocabulary, or report flags.

## Stop Line

This selection does not migrate:

- `max_decommit_bytes`, because it is a byte-bound payload compared against
  call input bytes;
- `HakoAllocBoundedDecommitReport` fields, because they are status / flag /
  base / bytes report vocabulary;
- fake proof source counters, page-source adapter state, heap/page mutation,
  OSVM byte/pointer payloads, provider / hook / global-allocator rows, TLS,
  atomics, or `#[global_allocator]`.

## Verification

Selection-only row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
