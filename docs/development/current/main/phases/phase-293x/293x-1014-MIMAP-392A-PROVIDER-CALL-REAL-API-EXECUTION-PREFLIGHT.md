# 293x-1014 MIMAP-392A Provider Call Real API Execution Preflight

Status: selected current
Date: 2026-05-21

## Purpose

Inventory the explicit preflight required before the provider-call seam can
perform a real provider API call. This row consumes the no-op execution seam
report and records readiness for a future real provider API call without
executing it.

## Scope

- Add a narrow provider-call real API execution preflight owner.
- Consume `HakoAllocProviderCallNoopExecutionSeamPilotReport`.
- Accept only explicit, accepted no-op execution seam reports.
- Record real provider API execution preflight readiness.
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

L3/L4 evidence is deferred until the first real provider API call execution
pilot or a provider-call execution closeout.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_real_api_execution_preflight_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
