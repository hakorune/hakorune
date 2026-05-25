---
Status: Landed
Date: 2026-05-25
Scope: phase-295x abandoned-heap stress evidence on the comparison lane
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-215-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CONTRACT-REFRESH.md
  - tools/allocator/mimalloc_abandoned_heap_stress_evidence_runner.py
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_evidence_guard.sh
---

# 295x-216 Abandoned Heap Stress Evidence

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-EVIDENCE-295X-002
```

The evidence runner pairs the existing abandoned-owner policy proof and
abandoned-reclaim inventory proof under one stable comparison evidence
contract.

## Evidence

The runner preserves the stable proof-pair shape:

```text
output_contract=mimalloc-comparison-abandoned-heap-stress-evidence-v0
evidence_pair=abandoned-owner-policy+abandoned-reclaim-inventory
proof_pair_summary=ok
```

Remote-proof stable fields:

```text
same=1,1,0
remote=2,1,1,0
abandoned=3,1,1,1,1
pending=0,6,4,3
counts=4,1,1,1,1
mailbox=0,0,0
shape=9
```

Reclaim-proof stable fields:

```text
missing=0,1,10,0
active_owner=0,2,0,1
same_owner=0,2,2,2
remote_pending=0,3,3
decommitted=0,4,1
live=1,0,1,1,1,0
retired=1,0,1,1,1
would=0,0,0,0,0,0
counts=7,2,5,1,2,1,1,1,1,1,16,0
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-PRESENTATION-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_evidence_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
