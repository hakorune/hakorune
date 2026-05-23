---
Status: Landed
Date: 2026-05-24
Scope: provider call capability gate inventory owner-local counter exact `usize` migration.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-196
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-197-HAKO-ALLOC-USIZE-PROVIDER-CALL-CAPABILITY-GATE-COUNTER-SELECTION.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/provider_call_capability_gate_inventory_box.hako
  - tools/checks/k2_wide_hako_alloc_provider_call_capability_gate_inventory_guard.sh
---

# 294x-198 Hako Alloc Usize Provider Call Capability Gate Counters

## Decision

Migrate only the selected owner-local
`HakoAllocProviderCallCapabilityGateInventory` counters to exact `usize`
storage:

- `inventory_count`
- `accepted_count`
- `reject_count`
- `missing_model_reject_count`
- `inactive_model_reject_count`
- `missing_capability_reject_count`
- `invalid_capability_reject_count`
- `closed_execution_reject_count`

## Stop Line

This row does not migrate:

- `last_reason`;
- `HakoAllocProviderCallCapabilityGateInventoryReportFields`;
- `HakoAllocProviderCallCapabilityGateInventoryReport`;
- capability-present / capability-valid flags, modeled-open payloads, or
  bool-like inactive / would-execute flags;
- provider calls, host replacement, hooks, global allocator install, backend
  matchers, worker/TLS, atomics, provider package / DLL generation, or
  `#[global_allocator]`.

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_capability_gate_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
