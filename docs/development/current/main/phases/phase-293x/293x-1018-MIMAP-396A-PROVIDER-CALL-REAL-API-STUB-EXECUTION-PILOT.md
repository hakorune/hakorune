# 293x-1018 MIMAP-396A Provider Call Real API Stub Execution Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open the first stubbed provider API call execution seam after the real API
execution preflight and first-pattern plan. This row records a model-space
provider API call result without host allocator replacement, hooks, backend
matcher additions, worker/thread execution, or global allocator install.

## Scope

- Add a narrow provider-call real API stub execution owner.
- Consume `HakoAllocProviderCallRealApiExecutionPreflightReport`.
- Accept only explicit, accepted real API execution preflight reports.
- Record stub provider API call execution evidence.
- Keep host replacement, hooks, backend matcher additions, worker/thread
  execution, and global allocator install closed.

## Stop Lines

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

Because this is the first provider API stub execution seam, L3 evidence may be
added by a follow-up closeout row instead of this daily row.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_provider_call_real_api_stub_execution_pilot_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added `provider_call_real_api_stub_execution_pilot_box.hako`.
- Added the manifest-backed proof app and L2 guard.
- Recorded stub/model-space provider API call execution evidence while actual
  provider API calls, host replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install remain closed.
