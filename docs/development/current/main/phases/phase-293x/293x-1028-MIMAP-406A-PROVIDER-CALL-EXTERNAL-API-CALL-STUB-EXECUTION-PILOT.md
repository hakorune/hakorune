# 293x-1028 MIMAP-406A Provider Call External API Call Stub Execution Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open the first model-space external provider API call stub execution seam after
the external API adapter closeout. This row records a stub external call result
without executing an external provider API call or replacing the host allocator.

## Scope

- Add a narrow provider-call external API call stub execution owner.
- Consume `HakoAllocProviderCallExternalApiAdapterPreflightReport`.
- Accept only explicit, accepted adapter preflight reports.
- Record stub external provider API call execution evidence.
- Keep actual external provider API execution, host allocator replacement,
  hooks, backend matcher additions, worker/thread execution, and global
  allocator install closed.

## Stop Lines

- No actual external provider API execution.
- No host allocator replacement, hooks, backend matcher additions, or
  `#[global_allocator]`.
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

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_external_api_call_stub_execution_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added `provider_call_external_api_call_stub_execution_pilot_box.hako`.
- Added the manifest-backed proof app and L2 guard.
- Recorded model-space external provider API call stub execution evidence while
  actual external provider API calls, host replacement, hooks, backend matcher
  additions, worker/thread execution, and global allocator install remain
  closed.
