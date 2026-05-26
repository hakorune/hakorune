---
Status: Landed
Date: 2026-05-27
Scope: load a manifest-selected shared library and stop before export resolution.
Blocker: MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001
Related:
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-09-MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/provider_package_load_only_smoke.py
---

# 296x-10 Provider Shared-Library Load-Only Smoke

## Decision

Close:

```text
MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001
```

The row name is historical. The accepted behavior is
shared-library-load-only smoke:

```text
MIMALLOC-PROVIDER-SHARED-LIBRARY-LOAD-ONLY-SMOKE-296X-001
```

Add a load-only provider-package smoke:

```text
tools/allocator/provider_package_load_only_smoke.py
```

The tool validates manifest metadata, verifies the binary hash, loads the
manifest-selected shared library, and stops before export resolution.

## Contract

```text
output_contract=hakorune-provider-package-load-only-smoke-v0
dll_mode=load-only
manifest_ready=1
descriptor_ready=0
binary_hash_ready=1
shared_library_load_executed=1
required_export_resolved=0
descriptor_read_executed=0
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
MIMALLOC-PROVIDER-DESCRIPTOR-READ-SMOKE-296X-001
```

The next row may resolve and call only `hakorune_provider_descriptor_v1`. It
must not bind provider APIs, call allocator entrypoints, activate providers,
replace the process allocator, install hooks, or compute benchmark winners.

## Stop Line

This row does not resolve exports, call descriptors, bind provider APIs, call
allocator entrypoints, activate providers, replace the process allocator,
install hooks, or compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_shared_library_load_only_smoke_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
