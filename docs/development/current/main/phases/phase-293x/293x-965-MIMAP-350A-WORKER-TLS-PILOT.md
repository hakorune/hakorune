# 293x-965 MIMAP-350A Worker/TLS Pilot

Status: landed
Date: 2026-05-21

## Decision

Open worker/TLS behavior as the next narrow seam after the OSVM/page-source
pilot.

## Context

MIMAP-349A proved a bounded OSVM/page-source fact after atomic bitmap. The next
seam may connect that scalar execution chain to worker/TLS facts needed for the
allocator, but it must not activate providers or replace the host allocator.

## Scope

- Add a worker/TLS pilot owner/proof/guard.
- Consume the MIMAP-349A OSVM/page-source report.
- Publish bounded scalar worker/TLS facts for the allocator seam.
- Keep provider activation, host allocator replacement, hooks, and backend
  matcher execution closed.

## Landed Shape

- Owner: `lang/src/hako_alloc/memory/worker_tls_pilot_box.hako`
- Proof: `apps/hako-alloc-worker-tls-pilot-proof`
- Design SSOT:
  `docs/development/current/main/design/hako-alloc-worker-tls-pilot-ssot.md`
- Guard: `tools/checks/k2_wide_hako_alloc_worker_tls_pilot_guard.sh`

The row records a single accepted OSVM/page-source report into a bounded
worker/TLS roundtrip through the existing `HakoAllocWorkerTlsCache` internal
substrate seam. It marks worker/TLS as present while keeping worker scheduling,
provider activation, host allocator replacement, hooks, and backend matchers
closed.

## Stop Lines

- No provider activation, host allocator replacement, hooks, or
  `#[global_allocator]`.
- No source-level concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.
- No backend `.inc` matcher by app, box, owner, or row name.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_worker_tls_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
