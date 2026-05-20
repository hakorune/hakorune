# 293x-1024 MIMAP-402A Provider Call External API Adapter Preflight

Status: landed
Date: 2026-05-21

## Purpose

Record a provider-call external API adapter preflight after the adapter
inventory. This row proves that the adapter boundary is present and valid before
any external provider API execution is opened.

## Scope

- Add a narrow provider-call external API adapter preflight owner.
- Consume `HakoAllocProviderCallExternalApiAdapterInventoryReport`.
- Accept only explicit, accepted adapter inventory reports.
- Record external provider API call preflight readiness.
- Keep external provider API execution, host allocator replacement, hooks,
  backend matcher additions, worker/thread execution, and global allocator
  install closed.

## Stop Lines

- No external provider API execution.
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
bash tools/checks/k2_wide_hako_alloc_provider_call_external_api_adapter_preflight_guard.sh --level L2
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added `provider_call_external_api_adapter_preflight_box.hako`.
- Added the manifest-backed proof app and L2 guard.
- Recorded external provider API adapter preflight readiness while external
  provider API execution, host replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install remain closed.
