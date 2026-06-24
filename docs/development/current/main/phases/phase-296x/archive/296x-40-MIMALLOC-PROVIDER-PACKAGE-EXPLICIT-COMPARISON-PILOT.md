---
Status: Landed
Date: 2026-05-27
Scope: run the first .hako-derived provider package explicit comparison pilot.
Blocker: MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-39-MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION.md
  - tools/allocator/mimalloc_repeated_measurement_runner.py
  - tools/allocator/provider_package_explicit_repeated_measurement.py
  - tools/allocator/provider_explicit_comparison_adapter.py
---

# 296x-40 Provider Package Explicit Comparison Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001
```

Run the first provider-package comparison using a `.hako`-derived generated
provider package:

```text
selected .hako source
  -> provider package build with alloc-free-owns-literal-v0
  -> provider package explicit repeated measurement
  -> .hako exact-EXE + C mimalloc repeated measurement
  -> 3-way comparison adapter
```

The pilot keeps the comparison evidence-only:

```text
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
workload_id=representative-small-block-v0
sample_count=3
warmup_count=1
operation_repeat=128
winner_claim=0
```

## Evidence

The guard produces:

```text
output_contract=hakorune-provider-explicit-repeated-measurement-v0
dll_mode=provider-explicit-repeated-measurement
sample_count=3
warmup_count=1
operation_repeat=128
provider_call_executed=1
allocator_entrypoint_called=1
provider_active=0
replacement_active=0
winner_claim=0
summary=ok
```

and adapts it with `.hako`/C evidence into:

```text
output_contract=mimalloc-provider-explicit-comparison-adapter-v0
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
subject_count=3
subject_0_id=hako_exact_exe
subject_1_id=c_mimalloc_explicit_runner
subject_2_id=provider_package_explicit_alloc_free
subject_0_winner_claim=0
subject_1_winner_claim=0
subject_2_winner_claim=0
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
MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001
```

The closeout row should stabilize this generated provider-package comparison as
the benchmark-return surface, then decide whether to continue with more
workloads or move to a separate activation/replacement decision lane.

## Stop Line

This row does not compare winners, activate providers, replace process
allocators, install hooks, use global allocator integration, or claim the
generated package is a host allocator replacement.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_explicit_comparison_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
