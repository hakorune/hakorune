---
Status: Landed
Date: 2026-05-27
Scope: resolve and call only the provider descriptor export.
Blocker: MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001
Related:
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-10-MIMALLOC-PROVIDER-SHARED-LIBRARY-LOAD-ONLY-SMOKE.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/provider_package_descriptor_smoke.py
---

# 296x-11 Provider Descriptor-Read Smoke

## Decision

Close:

```text
MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001
```

Add descriptor-read smoke:

```text
tools/allocator/provider_package_descriptor_smoke.py
```

The tool validates manifest metadata, verifies the binary hash, loads the
manifest-selected shared library, resolves `hakorune_provider_descriptor_v1`,
calls only that descriptor export, validates descriptor magic / ABI major /
size / contract hash, and stops before provider API bind.

## Contract

```text
output_contract=hakorune-provider-package-descriptor-smoke-v0
dll_mode=descriptor-smoke
manifest_ready=1
descriptor_ready=1
binary_hash_ready=1
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
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
MIMALLOC-PROVIDER-API-BIND-SMOKE-296X-001
```

The next row may bind the provider API table. It must keep explicit allocator
calls, provider activation, process allocator replacement, hooks, global
allocator integration, and benchmark winner claims closed.

## Stop Line

This row does not bind provider APIs, call allocator entrypoints, activate
providers, replace the process allocator, install hooks, or compute benchmark
winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_descriptor_read_smoke_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
