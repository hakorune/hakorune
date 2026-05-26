---
Status: Landed
Date: 2026-05-27
Scope: call only the provider no-op function and keep allocator entrypoints closed.
Blocker: MIMALLOC-PROVIDER-NOOP-CALL-SMOKE-296X-001
Related:
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-12-MIMALLOC-PROVIDER-API-BIND-SMOKE.md
  - tools/allocator/provider_package_noop_call_smoke.py
---

# 296x-13 Provider No-op Call Smoke

## Decision

Close:

```text
MIMALLOC-PROVIDER-NOOP-CALL-SMOKE-296X-001
```

Add no-op provider call smoke:

```text
tools/allocator/provider_package_noop_call_smoke.py
```

The tool binds the API table and calls only `ping`. It does not call allocator
entrypoints or activate the provider.

## Contract

```text
output_contract=hakorune-provider-package-noop-call-smoke-v0
dll_mode=provider-noop-call
provider_api_bound=1
provider_call_executed=1
provider_noop_call_executed=1
allocator_entrypoint_called=0
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
MIMALLOC-PROVIDER-ALLOC-FREE-SMOKE-296X-001
```

The next row may call explicit provider `alloc` / `free` through the API table.
It must keep process allocator replacement, hooks, global allocator
integration, and benchmark winner claims closed.

## Stop Line

This row does not call allocator entrypoints, activate providers, replace the
process allocator, install hooks, or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_noop_call_smoke_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
