---
Status: Current
Date: 2026-05-27
Scope: pilot hakmem benchmark compatibility with the probe-only LD_PRELOAD shim.
Blocker: HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-73-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE.md
---

# 296x-74 Hako Mimalloc Hakmem LD_PRELOAD Bench Pilot

## Purpose

Pilot one hakmem-side benchmark compatibility check with the probe-only
LD_PRELOAD shim before any benchmark winner claim or default replacement.

## Required Input

```text
output_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0
ld_preload_compatible=1
malloc_family_symbols_exported=1
hakmem_script_compatible=probe-only
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-hakmem-ldpreload-bench-pilot-v0
hakmem_script_compatible=probe-only
ld_preload_env_applied=1
benchmark_sample_executed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Stop Line

Do not claim C mimalloc parity, do not make the shim default for Hakorune, and
do not open process allocator replacement beyond the explicit probe process.
