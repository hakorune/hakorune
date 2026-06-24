---
Status: Landed
Date: 2026-05-27
Scope: select the benchmark return row after .hako-derived provider package v0 functional closeout.
Blocker: MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-38-MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT.md
  - tools/allocator/provider_package_explicit_repeated_measurement.py
  - tools/allocator/provider_explicit_comparison_adapter.py
---

# 296x-39 Provider Package Benchmark Return Selection

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-BENCHMARK-RETURN-SELECTION-296X-001
```

Return from provider package build work to benchmark evidence by selecting a
provider-package explicit comparison pilot.

The next row should run the already accepted same workload through:

```text
.hako exact-EXE + C mimalloc repeated measurement
.hako-derived provider package explicit alloc/free repeated measurement
3-way comparison adapter
```

The comparison remains evidence-only:

```text
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
winner_claim=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
```

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-EXPLICIT-COMPARISON-PILOT-296X-001
```

The pilot should build the `.hako`-derived provider package with
`alloc-free-owns-literal-v0`, run explicit provider alloc/free repeated
measurement against that generated package, and adapt it with the landed
`.hako`/C repeated measurement report into the 3-way comparison contract.

## Stop Line

This selection does not activate a provider, replace a process allocator,
install hooks, use global allocator integration, or make benchmark winner
claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_benchmark_return_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
