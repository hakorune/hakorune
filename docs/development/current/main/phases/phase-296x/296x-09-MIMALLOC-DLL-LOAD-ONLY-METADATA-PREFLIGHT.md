---
Status: Landed
Date: 2026-05-27
Scope: validate provider-package manifest metadata before shared-library loading.
Blocker: MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-08-MIMALLOC-DLL-LOAD-ONLY-SELECTION.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - tools/allocator/provider_package_metadata_preflight.py
---

# 296x-09 DLL Load-Only Metadata Preflight

## Decision

Close:

```text
MIMALLOC-DLL-LOAD-ONLY-METADATA-PREFLIGHT-296X-001
```

Add a no-load provider-package metadata preflight:

```text
tools/allocator/provider_package_metadata_preflight.py
```

The preflight validates a manifest fixture before any shared library is loaded.
It checks the package ABI name, target/profile fields, binary artifact name,
64-hex `binary_sha256`, 64-hex `contract_hash`, and the required single export
name:

```text
hakorune_provider_get_api_v1
```

## Contract

```text
output_contract=hakorune-provider-package-metadata-preflight-v0
dll_mode=metadata-preflight
manifest_ready=1
descriptor_ready=0
binary_hash_ready=1
shared_library_load_executed=0
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
MIMALLOC-DLL-LOAD-ONLY-SHARED-LIBRARY-SMOKE-296X-001
```

The next row may add a descriptor-only shared-library load smoke. It must not
call allocator entrypoints, activate a provider, replace the process allocator,
install hooks, or compute benchmark winners.

## Stop Line

This row does not generate a provider package, load a shared library, call
exports, activate a provider, replace the process allocator, install hooks, or
compute benchmark winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_dll_metadata_preflight_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
