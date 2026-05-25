---
Status: Current
Date: 2026-05-25
Scope: run the smallest production-facing remote-free comparison evidence contract on the comparison lane.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-242-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH.md
  - tools/allocator/mimalloc_remote_free_production_facade_evidence_runner.py
  - tools/checks/k2_wide_phase295x_mimalloc_remote_free_production_facade_evidence_guard.sh
---

# 295x-243 Remote-Free Production Facade Evidence

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002
```

The refreshed contract is now run once as a bundled comparison evidence
contract. The evidence stays on the semantic side of the lane and does not
open provider activation, replacement, hook, or global allocator seams.

The evidence bundle reuses the existing proof surfaces for:

```text
worker identity / TLS cache slots
pointer atomic routes
remote-free policy
remote-abandoned-owner policy
remote-free page integration
thread-safe hako_mem ABI
native multi-worker stress
```

The output contract is fixed as:

```text
output_contract=mimalloc-comparison-remote-free-production-facade-evidence-v0
```

Stable report vocabulary:

```text
worker_id
tls_cache_slot
atomic_route
remote_pending
abandoned_owner
page_ownership
thread_safe_abi
provider_active
replacement_active
winner_claim
counts
```

## Evidence Contract

The evidence runner keeps the comparison lane narrow:

```text
proof_bundle=worker_tls_cache+remote_free_policy+ptr_remote_free_list+remote_abandoned_owner_policy+remote_free_page_integration+threadsafe_abi+native_stress
worker_id=0
tls_cache_slot=0
atomic_route=ptr_store_load_cas
remote_pending=0,6,4,3
abandoned_owner=3,1,1,1,1
page_ownership=0,2,1,2
thread_safe_abi=1
provider_active=0
replacement_active=0
winner_claim=0
counts=6
```

## Stop Line

This row does not open provider activation, DLL/replacement/hook/global
allocator seams, does not broaden to source-level threading, and does not add
benchmark workloads, repeated medians, or timing parity claims.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-MINIMUM-BENCHMARK-SELECTION-295X-002
```

The semantic closeout now folds directly into an implementation-first benchmark
selection row so the lane does not create a presentation-only mimalloc row that
touches no `.hako` implementation.
