---
Status: Landed
Date: 2026-05-27
Scope: run repeated measurement through explicit provider alloc/free.
Blocker: MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT-296X-001
Related:
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-14-MIMALLOC-PROVIDER-ALLOC-FREE-SMOKE.md
  - tools/allocator/provider_package_explicit_repeated_measurement.py
---

# 296x-15 Provider Explicit Repeated Measurement

## Decision

Close:

```text
MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT-296X-001
```

Add repeated measurement through explicit provider alloc/free:

```text
tools/allocator/provider_package_explicit_repeated_measurement.py
```

The measurement uses `warmup_count=1`, `sample_count=3`, and
`operation_repeat=128`. It calls provider `alloc` / `free` explicitly through
the API table. It does not replace the process allocator and does not claim a
winner.

## Contract

```text
output_contract=hakorune-provider-explicit-repeated-measurement-v0
measurement_profile=phase296x-provider-explicit-repeated-v0
dll_mode=provider-explicit-repeated-measurement
warmup_count=1
sample_count=3
operation_repeat=128
provider_api_bound=1
provider_call_executed=1
allocator_entrypoint_called=1
provider_alloc_executed=1
provider_free_executed=1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT-296X-001
```

The next row may close out provider explicit measurement evidence and decide
whether to park provider work or open a separate activation decision lane.

## Stop Line

This row does not activate providers, replace the process allocator, install
hooks, use global allocator integration, or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_repeated_measurement_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
