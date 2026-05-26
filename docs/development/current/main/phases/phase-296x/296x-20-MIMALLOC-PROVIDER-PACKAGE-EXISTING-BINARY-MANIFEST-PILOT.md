---
Status: Landed
Date: 2026-05-27
Scope: package an existing provider binary with a Hakorune provider manifest.
Blocker: MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001
Related:
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - tools/allocator/provider_package_existing_binary_manifest.py
---

# 296x-20 Provider Package Existing-Binary Manifest Pilot

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT-296X-001
```

Add Phase A package artifact support for an already-built provider binary:

```text
tools/allocator/provider_package_existing_binary_manifest.py
```

The helper creates this package layout:

```text
hakorune_provider.json
hakorune_provider.sha256
libhakorune_provider.so | hakorune_provider.dll | libhakorune_provider.dylib
```

It copies the selected binary, computes artifact `sha256` / `size_bytes`,
emits manifest v1, and leaves runtime loading closed.

## Contract

```text
output_contract=hakorune-provider-package-existing-binary-manifest-v0
package_mode=existing-binary-manifest
schema_version=hakorune-provider-package-v1
abi_version=hakorune-provider-abi-v1
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
winner_claim=0
summary=ok
```

The generated manifest must also pass metadata preflight without loading the
shared library.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001
```

The next row should close the Phase A package helper and decide whether to
wire a stable CLI command name or move to selected-provider binary build.

## Stop Line

This row does not compile `.hako` to a shared library, load provider binaries,
resolve exports, read descriptors, bind provider APIs, call allocator
entrypoints, activate providers, replace the process allocator, install hooks,
use global allocator integration, or compute winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_existing_binary_manifest_pilot_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
