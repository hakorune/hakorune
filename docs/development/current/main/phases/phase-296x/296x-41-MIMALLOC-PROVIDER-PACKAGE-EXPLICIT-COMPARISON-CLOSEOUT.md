---
Status: Landed
Date: 2026-05-27
Scope: close the .hako-derived provider package explicit comparison evidence and return to parity lane selection.
Blocker: MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-40-MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT.md
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# 296x-41 Provider Package Explicit Comparison Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-CLOSEOUT-296X-001
```

The provider-package explicit comparison surface is now closed as benchmark
return evidence:

```text
output_contract=mimalloc-provider-explicit-comparison-adapter-v0
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
subject_count=3
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

The next lane is the `.hako` mimalloc performance-parity roadmap. Keep
`hakozuna_reference` as reference-only, and keep allocator product selection
parked until a separate decision row opens it.

## Selected Next

Select:

```text
HAKO-MIMALLOC-PERF-PARITY-ROADMAP-SELECTION-296X-001
```

The next row should select the parity lane without opening provider
activation, process allocator replacement, hooks, or global allocator
integration.

## Stop Line

This row does not run benchmarks, activate providers, replace the process
allocator, install hooks, or make winner claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_explicit_comparison_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
