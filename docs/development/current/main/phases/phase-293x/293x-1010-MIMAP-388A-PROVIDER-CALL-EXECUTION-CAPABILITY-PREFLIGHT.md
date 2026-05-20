# 293x-1010 MIMAP-388A Provider Call Execution Capability Preflight

Status: selected current
Date: 2026-05-21

## Purpose

Inventory the explicit capability preflight required before provider API call
execution can open. This row consumes the provider-call modeled-open report and
records whether the execution capability seam is ready, while keeping actual
provider API calls closed.

## Scope

- Add a narrow provider-call execution capability preflight owner.
- Consume `HakoAllocProviderCallModeledOpenPilotReport`.
- Accept only explicit, accepted provider-call modeled-open reports.
- Record that provider-call execution capability is present in model space.
- Keep actual provider API calls, host replacement, hooks, backend matcher
  additions, worker/thread execution, and global allocator install closed.

## Stop Lines

- No provider API calls.
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

L3/L4 evidence is deferred until the first provider-call execution seam or a
provider-call execution capability closeout.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_execution_capability_preflight_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
