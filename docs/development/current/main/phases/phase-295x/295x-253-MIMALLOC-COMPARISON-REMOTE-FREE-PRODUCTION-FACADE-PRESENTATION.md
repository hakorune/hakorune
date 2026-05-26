---
Status: Current
Date: 2026-05-27
Scope: normalize the remote-free production-facade evidence into a stable presentation contract and hand off to the malloc-large closeout seam.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-243-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE.md
  - tools/allocator/mimalloc_remote_free_production_facade_evidence_runner.py
  - tools/allocator/mimalloc_remote_free_production_facade_presentation.py
  - tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_presentation_guard.sh
---

# 295x-253 Remote-Free Production Facade Presentation

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-PRESENTATION-295X-002
```

Normalize the remote-free production-facade evidence into a stable presentation
contract without widening provider or replacement seams, then hand off to the
malloc-large closeout seam.

## Presentation

The evidence is normalized into one stable presentation contract:

```text
output_contract=mimalloc-comparison-remote-free-production-facade-presentation-v0
input_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0
presentation_only=1
proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress
worker_id=0
tls_cache_slot=0
atomic_route=ptr_store_load_cas
remote_pending=0,6,4,3
abandoned_owner=3,1,1,1,1
page_ownership=0,2,1,2
thread_safe_abi=1
native_multi_worker_stress=1
worker_count=4
iterations_per_worker=64
expected_remote_free_count=256
observed_remote_free_count=256
drained_nodes=256
payload_sum_nonzero=1
provider_active=0
replacement_active=0
winner_claim=0
counts=6
summary=ok
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-MALLOC-LARGE-MEMORY-GAP-CLOSEOUT-295X-001
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_presentation_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
