---
Status: Landed
Date: 2026-05-27
Scope: close explicit provider repeated measurement and select 3-way comparison contract.
Blocker: MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-15-MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
---

# 296x-16 Provider Explicit Measurement Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT-296X-001
```

The explicit provider ladder is open only through in-process explicit provider
API calls:

```text
metadata-preflight=landed
shared-library-load-only=landed
descriptor-read=landed
provider-api-bind=landed
provider-noop-call=landed
provider-alloc-free=landed
provider-explicit-repeated-measurement=landed
```

Activation stays parked:

```text
provider_activation_lane=parked
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001
```

The next row should define a 3-way comparison contract for:

```text
hako_exact_exe
c_mimalloc_explicit_runner
provider_package_explicit_alloc_free
```

The comparison contract must keep winner claims and process replacement closed.

## Stop Line

This row does not run another benchmark, activate providers, replace the
process allocator, install hooks, use global allocator integration, or compute
benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_measurement_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
