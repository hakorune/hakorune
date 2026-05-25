---
Status: Current
Date: 2026-05-25
Scope: select the remote-free production facade as the next allocator-facing semantic seam.
Related:
  - docs/development/current/main/phases/phase-295x/295x-240-MIMALLOC-COMPARISON-NEXT-SEMANTIC-SEAM-SELECTION.md
  - docs/development/current/main/design/mimalloc-concurrency-substrate-boundary-ssot.md
---

# 295x-241 Remote-Free Production Facade Selection

## Blocker

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-SELECTION-295X-002
```

## Decision

Pick the remote-free production facade as the next allocator-facing semantic
seam after the benchmark-only process-repeat pack was closed.

This row stays on the semantic side of the lane. It does not reopen
process-repeat evidence or add another median-only workload row.

The selected seam is:

```text
MIMAP-REMOTE-001 production-facade remote-free policy integration over existing atomic/TLS proofs
```

The reason this is the right seam is that the substrate evidence already exists
for:

```text
worker identity
TLS cache slots
atomic routes
remote-free policy
thread-safe hako_mem ABI
native multi-worker stress
```

## Stop Line

This row does not open provider activation, DLL/replacement/hook/global
allocator seams, does not broaden to source-level threading, and does not add
benchmark workloads or timing parity claims.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-PRODUCTION-FACADE-CONTRACT-REFRESH-295X-002
```

The next row should turn this selection into the smallest production-facing
remote-free comparison contract that still keeps provider and replacement
seams closed.
