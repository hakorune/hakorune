---
Status: Landed
Date: 2026-05-27
Scope: adapt landed hako/C/provider explicit measurement evidence into the 3-way comparison contract.
Blocker: MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-17-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CONTRACT.md
  - tools/allocator/provider_explicit_comparison_adapter.py
---

# 296x-18 Provider Explicit Comparison Adapter Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT-296X-001
```

Add a no-rerun adapter:

```text
tools/allocator/provider_explicit_comparison_adapter.py
```

The adapter reads landed hako/C repeated evidence and provider explicit
repeated evidence, then emits the 3-way scalar comparison surface. It does not
run new measurements and does not claim winners.

## Contract

```text
output_contract=mimalloc-provider-explicit-comparison-adapter-v0
input_contract=mimalloc-provider-explicit-comparison-contract-v0
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
subject_count=3
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001
```

The next row should decide whether to park provider packaging work or start a
new package-build artifact lane. Activation remains parked.

## Stop Line

This row does not run benchmarks, activate providers, replace the process
allocator, install hooks, use global allocator integration, or compute winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_adapter_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
