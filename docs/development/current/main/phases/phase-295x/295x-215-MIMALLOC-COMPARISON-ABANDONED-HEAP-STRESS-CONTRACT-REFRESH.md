---
Status: Current
Date: 2026-05-25
Scope: phase-295x abandoned-heap stress contract refresh on the comparison lane
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-214-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-SELECTION.md
  - apps/mimalloc-remote-abandoned-owner-policy-proof/main.hako
  - apps/hako-alloc-abandoned-reclaim-inventory-proof/main.hako
  - tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh
  - tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_contract_refresh_guard.sh
---

# 295x-215 Abandoned Heap Stress Contract Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH-295X-002
```

The abandoned-heap seam is refreshed by pairing the existing
`mimalloc-remote-abandoned-owner-policy-proof` and
`hako-alloc-abandoned-reclaim-inventory-proof` surfaces under one comparison
contract. The two proofs already expose the stable abandoned-owner /
abandoned-reclaim vocabulary needed for the next evidence run.

## Contract

The comparison contract keeps the existing proof shapes stable:

```text
output_contract=mimalloc-comparison-abandoned-heap-stress-contract-v0
```

Mimalloc-side stable fields:

```text
same
remote
abandoned
pending
counts
```

Hako-side stable fields:

```text
missing
active_owner
remote_pending
decommitted
live
retired
counts
```

These fields are enough to track abandoned-owner / reclaim-adjacent policy
shape without opening provider/DLL, host replacement, or broader reclaim
execution seams.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_contract_refresh_guard.sh
bash tools/checks/k2_wide_mimalloc_remote_abandoned_owner_policy_guard.sh
bash tools/checks/k2_wide_hako_alloc_abandoned_reclaim_inventory_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
