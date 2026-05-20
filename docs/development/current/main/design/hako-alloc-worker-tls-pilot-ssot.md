---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Row: MIMAP-350A
Scope: bounded worker/TLS pilot after the OSVM/page-source pilot.
Related:
  - docs/development/current/main/phases/phase-293x/293x-965-MIMAP-350A-WORKER-TLS-PILOT.md
  - lang/src/hako_alloc/memory/worker_tls_pilot_box.hako
  - apps/hako-alloc-worker-tls-pilot-proof/main.hako
  - tools/checks/k2_wide_hako_alloc_worker_tls_pilot_guard.sh
---

# Hako Alloc Worker/TLS Pilot

## Decision

MIMAP-350A opens a bounded allocator-facing worker/TLS fact after MIMAP-349A
OSVM/page-source. It consumes an accepted `HakoAllocOSVMPageSourcePilotReport`
and records one scalar worker/TLS roundtrip through the existing internal
`HakoAllocWorkerTlsCache` seam.

This row is a substrate proof row. It is not source-level concurrency.

## Owner

`HakoAllocWorkerTlsPilot` owns this row.

It may:

- require an accepted OSVM/page-source report
- call `HakoAllocWorkerTlsCache.storeSlot/loadSlot/clearSlot`
- publish scalar report fields for worker id, TLS slot, roundtrip values, and
  inherited allocator execution tokens
- mark `would_use_worker_tls = 1`
- keep `would_run_thread = 0`

It must not:

- expose `worker_local` syntax
- spawn or schedule workers
- introduce `Channel`, `co`, `nowait`, `await`, or `sync box` behavior
- execute release/recycle behavior
- activate providers
- replace the host allocator
- expose hooks or `#[global_allocator]`
- add backend `.inc` matchers by app, box, owner, or row name

## Reasons

```text
0 = accepted
1 = missing page-source fact
2 = rejected page-source fact
3 = invalid worker identity
4 = invalid TLS slot
5 = TLS roundtrip mismatch
6 = closed execution blocker inherited from the OSVM/page-source report
```

## Validation

Daily validation is L2:

```bash
bash tools/checks/k2_wide_hako_alloc_worker_tls_pilot_guard.sh --level L2
```

L3/L4 evidence is deferred to a closeout or provider-facing row because this
row only composes existing worker/TLS substrate with the allocator scalar chain.
