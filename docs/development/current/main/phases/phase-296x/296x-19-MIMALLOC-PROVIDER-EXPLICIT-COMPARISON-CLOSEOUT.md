---
Status: Landed
Date: 2026-05-27
Scope: close the 3-way explicit comparison adapter and select the next provider package artifact lane.
Blocker: MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-18-MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-ADAPTER-PILOT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
---

# 296x-19 Provider Explicit Comparison Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-EXPLICIT-COMPARISON-CLOSEOUT-296X-001
```

The 3-way explicit comparison surface is now available as a no-rerun adapter:

```text
output_contract=mimalloc-provider-explicit-comparison-adapter-v0
comparison_subjects=hako_exact_exe,c_mimalloc_explicit_runner,provider_package_explicit_alloc_free
subject_count=3
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

Do not open provider activation from this row. The next safe step is the
provider package artifact lane, Phase A from the artifact SSOT: package an
existing binary with a manifest.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001
```

The next row may add a package artifact helper for an already-built provider
binary plus `hakorune_provider.json`. It must not generate a shared library
from `.hako`, activate providers, replace the process allocator, install
hooks, or claim benchmark winners.

## Stop Line

This closeout does not run benchmarks, build shared libraries, load provider
binaries, resolve exports, bind provider APIs, activate providers, replace the
process allocator, install hooks, use global allocator integration, or compute
winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_explicit_comparison_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
