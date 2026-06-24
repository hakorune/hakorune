---
Status: Landed
Date: 2026-05-27
Scope: define the 3-way hako/C/provider explicit comparison contract.
Blocker: MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-16-MIMALLOC-PROVIDER-EXPLICIT-MEASUREMENT-CLOSEOUT.md
  - docs/development/current/main/phases/phase-296x/296x-07-MIMALLOC-BENCHMARK-EXACT-EXE-REPEATED-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-15-MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-17 Provider Explicit Comparison Contract

## Decision

Close:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT-296X-001
```

Define the 3-way comparison contract before writing adapters:

```text
output_contract=mimalloc-provider-explicit-comparison-contract-v0
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
measurement_profile=phase296x-provider-explicit-comparison-v0
sample_count=3
warmup_count=1
operation_repeat=128
summary_statistic=min,median,max
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
```

## Subject Inputs

The contract consumes existing landed evidence families:

```text
hako_exact_exe:
  output_contract=mimalloc-comparison-repeated-measurement-v0
  source_row=296x-07

c_mimalloc_explicit_runner:
  output_contract=mimalloc-comparison-repeated-measurement-v0
  source_row=296x-07

provider_package_explicit_alloc_free:
  output_contract=hakorune-provider-explicit-repeated-measurement-v0
  source_row=296x-15
```

The first adapter row may bridge these into one normalized scalar report. It
must not run new benchmarks, activate providers, replace allocators, install
hooks, use global allocator integration, or compute winners.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001
```

## Stop Line

This row defines only the contract. It does not run new measurements, activate
providers, replace the process allocator, install hooks, use global allocator
integration, or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_contract_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
