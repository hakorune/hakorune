---
Status: Current
Date: 2026-05-25
Scope: phase-295x thread-safe hako_mem ABI seam selection on the comparison lane
Blocker: MIMALLOC-COMPARISON-THREADSAFE-ABI-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-208-MIMALLOC-COMPARISON-REMOTE-FREE-POLICY-SELECTION.md
  - tools/checks/k2_wide_hako_mem_threadsafe_abi_guard.sh
---

# 295x-209 Threadsafe ABI Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-THREADSAFE-ABI-SELECTION-295X-002
```

Closed the production-facade remote-free policy selection row and selected the thread-safe `hako_mem` ABI boundary as the next narrow concurrency seam.

## Selected Row

Select:

```text
MIMAP-THREADSAFE-ABI-001 thread-safe `hako_mem` ABI contract
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_hako_mem_threadsafe_abi_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
