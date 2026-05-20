# 293x-1012 MIMAP-390A Provider Call No-Op Execution Seam Pilot

Status: selected current
Date: 2026-05-21

## Purpose

Open the provider-call execution boundary as a no-op model seam after the
execution capability preflight. This row proves that the allocator can cross an
explicit execution seam without calling a provider API yet.

## Scope

- Add a narrow provider-call no-op execution seam owner.
- Consume `HakoAllocProviderCallExecutionCapabilityPreflightReport`.
- Accept only explicit, accepted execution capability preflight reports.
- Record no-op execution seam evidence.
- Keep actual provider API calls, host replacement, hooks, backend matcher
  additions, worker/thread execution, and global allocator install closed.

## Stop Lines

- No actual provider API calls.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

Daily validation is L2:

```text
VM proof
MIR JSON emit
route preflight
```

L3/L4 evidence is deferred until the first real provider API call execution seam
or a provider-call execution closeout.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_noop_execution_seam_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
