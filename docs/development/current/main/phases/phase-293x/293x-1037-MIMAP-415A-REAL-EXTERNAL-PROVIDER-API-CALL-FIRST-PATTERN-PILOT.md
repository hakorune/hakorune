# 293x-1037 MIMAP-415A Real External Provider API Call First-Pattern Pilot

Status: landed
Date: 2026-05-21

## Purpose

Open the first explicitly-scoped real external provider API call pilot after the
MIMAP-414A plan. The pilot must be narrow: it may exercise the real-call seam
only through the planned adapter/preflight report boundary, while host allocator
replacement, hooks, backend matcher additions, worker/thread execution, and
global allocator install remain closed.

## Scope

- Add the first-pattern real external provider API call pilot owner.
- Consume `HakoAllocRealExternalProviderApiAdapterExecutionPreflightReport`.
- Accept only explicit, accepted MIMAP-410A preflight reports.
- Record real external provider API call execution/result evidence.
- Keep host allocator replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install closed.

## Stop Lines

- No host allocator replacement, hooks, backend matcher additions, or
  `#[global_allocator]`.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Validation

First-pattern validation is L3 unless implementation deliberately keeps the
pilot model-only and the active guard documents an L2-only reason.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_real_external_provider_api_call_first_pattern_pilot_guard.sh --level L3
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed Evidence

- Added the first-pattern real external provider API call pilot owner.
- Added the proof app and L3 guard.
- Added the MIMAP-415A design SSOT and proof-app manifest row.
- Recorded real external provider API call pilot evidence through the
  MIMAP-410A preflight report boundary.
- Kept host allocator replacement, hooks, backend matcher additions,
  worker/thread execution, and global allocator install closed.
