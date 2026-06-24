---
Status: Landed
Date: 2026-05-27
Scope: bind the provider API table without calling provider functions.
Blocker: MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001
Related:
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-11-MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE.md
  - tools/allocator/provider_package_api_bind_smoke.py
---

# 296x-12 Provider API Bind Smoke

## Decision

Close:

```text
MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001
```

Add API-bind smoke:

```text
tools/allocator/provider_package_api_bind_smoke.py
```

The tool validates metadata, descriptor, and binary hash, then resolves and
calls `hakorune_provider_get_api_v1` to obtain the API table shape. It does not
call `ping`, allocator entrypoints, or activation hooks.

## Contract

```text
output_contract=hakorune-provider-package-api-bind-smoke-v0
dll_mode=provider-api-bind
manifest_ready=1
descriptor_ready=1
binary_hash_ready=1
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_api_bound=1
provider_call_executed=0
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
MIMALLOC-PROVIDER-NOOP-CALL-SMOKE-296X-001
```

The next row may call only a safe no-op provider function such as `ping`. It
must keep allocator entrypoints, activation, replacement, hooks, global
allocator integration, and benchmark winner claims closed.

## Stop Line

This row does not call provider function pointers, allocator entrypoints,
activate providers, replace the process allocator, install hooks, or compute
benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_api_bind_smoke_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
