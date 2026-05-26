---
Status: Landed
Date: 2026-05-27
Scope: call explicit provider alloc/free without process allocator replacement.
Blocker: MIMALLOC-PROVIDER-ALLOC-FREE-SMOKE-296X-001
Related:
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-13-MIMALLOC-PROVIDER-NOOP-CALL-SMOKE.md
  - tools/allocator/provider_package_alloc_free_smoke.py
---

# 296x-14 Provider Alloc/Free Smoke

## Decision

Close:

```text
MIMALLOC-PROVIDER-ALLOC-FREE-SMOKE-296X-001
```

Add explicit provider alloc/free smoke:

```text
tools/allocator/provider_package_alloc_free_smoke.py
```

The tool calls `alloc` and `free` through the provider API table. This is not
process allocator replacement and does not install hooks or global allocators.

## Contract

```text
output_contract=hakorune-provider-package-alloc-free-smoke-v0
dll_mode=provider-alloc-free
provider_api_bound=1
provider_call_executed=1
allocator_entrypoint_called=1
provider_alloc_executed=1
provider_free_executed=1
allocation_count=1
free_count=1
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
MIMALLOC-PROVIDER-EXPLICIT-REPEATED-MEASUREMENT-296X-001
```

The next row may run repeated measurement through explicit provider alloc/free.
It must keep process allocator replacement, hooks, global allocator
integration, and benchmark winner claims closed.

## Stop Line

This row does not activate providers, replace the process allocator, install
hooks, use global allocator integration, or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_alloc_free_smoke_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
