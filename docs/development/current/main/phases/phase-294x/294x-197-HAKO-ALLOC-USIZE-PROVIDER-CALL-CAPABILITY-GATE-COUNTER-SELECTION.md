---
Status: Landed
Date: 2026-05-24
Scope: select the next exact `usize` production field group.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-195
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_capability_gate_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_capability_gate_inventory_guard.sh
---

# 294x-197 Hako Alloc Usize Provider Call Capability Gate Counter Selection

## Decision

Select the owner-local `HakoAllocProviderCallCapabilityGateInventory` counters
as `HAKO-ALLOC-USIZE-FIELD-GROUP-196`:

- `inventory_count`
- `accepted_count`
- `reject_count`
- `missing_model_reject_count`
- `inactive_model_reject_count`
- `missing_capability_reject_count`
- `invalid_capability_reject_count`
- `closed_execution_reject_count`

These fields are monotonic capability-gate inventory/reject counters initialized
to `0`. The selected group records the explicit provider-call capability gate
only and keeps provider-call execution closed.

## Stop Line

Do not migrate:

- `last_reason`;
- `HakoAllocProviderCallCapabilityGateInventoryReportFields`;
- `HakoAllocProviderCallCapabilityGateInventoryReport`;
- capability-present / capability-valid flags, modeled-open payloads, or
  bool-like inactive / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Next Row

`HAKO-ALLOC-USIZE-FIELD-GROUP-196` should migrate only the selected owner-local
counters and update the capability-gate guard to assert exact `usize` storage
while report mirrors remain signed.

## Verification

Docs-only selection row:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
