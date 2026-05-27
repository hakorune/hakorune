---
Status: Landed
Date: 2026-05-27
Scope: build and smoke-test a hakmem-compatible LD_PRELOAD shim without enabling normal replacement.
Blocker: HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-SMOKE-296X-001
Related:
  - docs/development/current/main/design/hako-mimalloc-performance-parity-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-72-HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION.md
---

# 296x-73 Hako Mimalloc Hakmem LD_PRELOAD Shim Smoke

## Purpose

Build a probe-only malloc/free symbol shim for hakmem compatibility and smoke
it separately from normal Hakorune execution.

## Required Input

```text
output_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0
ld_preload_shim_decision=accepted
decision_scope=hakmem_compat_probe_only
ld_preload_shim_build_allowed=1
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Required Output

```text
output_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0
ld_preload_compatible=1
shared_library_load_executed=1
malloc_family_symbols_exported=1
hakmem_script_compatible=probe-only
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

## Evidence

```text
output_contract=hako-mimalloc-hakmem-ldpreload-shim-smoke-v0
input_contract=hako-mimalloc-hakmem-ldpreload-shim-decision-v0
ld_preload_compatible=1
shim_kind=malloc_family_probe_only
shared_library_load_executed=1
malloc_family_symbols_exported=1
malloc_family_symbols=malloc,free,calloc,realloc
hakmem_script_compatible=probe-only
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-BENCH-PILOT-296X-001
summary=ok
```

## Guard

```text
tools/checks/k2_wide_phase296x_hako_mimalloc_hakmem_ldpreload_shim_smoke_guard.sh
```

## Stop Line

Do not make Hakorune's own runtime use the shim by default. Do not claim
benchmark parity or enable process allocator replacement in this row.
