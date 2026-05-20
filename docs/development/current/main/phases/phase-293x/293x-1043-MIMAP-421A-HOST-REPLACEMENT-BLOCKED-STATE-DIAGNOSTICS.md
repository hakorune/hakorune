# 293x-1043 MIMAP-421A Host Replacement Blocked-State Diagnostics

Status: selected current
Date: 2026-05-21

## Purpose

Add a narrow diagnostic row for host replacement preflight blocked states after
MIMAP-420A. This row should make missing or rejected explicit preflight inputs
observable without installing hooks, adding backend matchers, replacing the
process allocator, or installing a global allocator.

## Scope

- Consume the MIMAP-420A explicit preflight inventory report.
- Summarize blocked states and reject reasons for missing explicit request,
  hook plan, rollback plan, backend no-growth proof, and closed-seam leakage.
- Keep host replacement itself closed.

## Stop Lines

- No hook installation.
- No backend matcher additions.
- No process allocator replacement.
- No `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Daily validation is L2:

```text
VM proof
MIR JSON emit
route preflight
```
