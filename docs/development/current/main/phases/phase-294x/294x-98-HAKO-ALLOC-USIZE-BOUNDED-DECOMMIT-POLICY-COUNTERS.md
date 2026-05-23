---
Status: Landed
Date: 2026-05-23
Scope: bounded decommit policy owner-local counter exact `usize` migration.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-97-HAKO-ALLOC-USIZE-BOUNDED-DECOMMIT-POLICY-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/purge_bounded_decommit_box.hako
  - apps/hako-alloc-bounded-decommit-policy-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh
---

# 294x-98 Hako Alloc Usize Bounded Decommit Policy Counters

## Decision

Migrate only the selected `HakoAllocBoundedDecommitPolicy` owner-local
monotonic counters to exact `usize` storage:

- `attempt_count`
- `blocked_count`
- `decommit_attempt_count`
- `decommit_success_count`
- `source_reject_count`

The M195 bounded decommit execution policy guard now asserts these fields are
exact `usize` in the typed-object plan.

## Stop Line

This row does not migrate:

- `max_decommit_bytes`, because it is a byte-bound payload compared against
  call input bytes;
- `HakoAllocBoundedDecommitReport` fields, because they are status / flag /
  base / bytes report vocabulary;
- fake proof source counters, page-source adapter state, heap/page mutation,
  OSVM byte/pointer payloads, provider / hook / global-allocator rows, TLS,
  atomics, or `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_bounded_decommit_policy_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
