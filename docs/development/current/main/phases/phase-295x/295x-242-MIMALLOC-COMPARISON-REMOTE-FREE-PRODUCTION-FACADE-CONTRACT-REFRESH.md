---
Status: Current
Date: 2026-05-25
Scope: refresh the smallest production-facing remote-free comparison contract.
Related:
  - docs/development/current/main/phases/phase-295x/295x-241-MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION.md
  - docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
---

# 295x-242 Remote-Free Production Facade Contract Refresh

## Blocker

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002
```

## Decision

Refresh the contract for the remote-free production facade after selecting it as
the next semantic seam.

This row keeps the comparison lane on the semantic side. It does not reopen
process-repeat evidence or add another benchmark-only median row.

The refreshed contract pairs the existing substrate evidence for:

```text
worker identity
TLS cache slots
atomic routes
remote-free policy
thread-safe hako_mem ABI
native multi-worker stress
```

The contract remains explicit about the comparison lane inputs and outputs:

```text
output_contract=mimalloc-comparison-remote-free-production-facade-contract-v0
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

## Stop Line

This row does not open provider activation, DLL/replacement/hook/global
allocator seams, does not broaden to source-level threading, and does not add
benchmark workloads, repeated medians, or timing parity claims.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-EVIDENCE-295X-002
```

The next row should run the refreshed remote-free comparison contract once,
without widening into provider activation or replacement seams.
