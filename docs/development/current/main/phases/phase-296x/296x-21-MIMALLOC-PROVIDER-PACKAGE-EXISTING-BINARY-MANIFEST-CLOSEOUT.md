---
Status: Landed
Date: 2026-05-27
Scope: close the existing-binary package helper and select the stable CLI package entry.
Blocker: MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-20-MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-PILOT.md
  - tools/allocator/provider_package_existing_binary_manifest.py
---

# 296x-21 Provider Package Existing-Binary Manifest Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-EXISTING-BINARY-MANIFEST-CLOSEOUT-296X-001
```

The existing-binary package helper is landed and emits a manifest package that
passes no-load metadata preflight:

```text
output_contract=hakorune-provider-package-existing-binary-manifest-v0
package_mode=existing-binary-manifest
schema_version=hakorune-provider-package-v1
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
summary=ok
```

To make provider packaging a Hakorune feature rather than only a repo-local
tool, the next row should expose the same Phase A package operation through
the `hakorune` CLI.

## Selected Next

Select:

```text
MIMALLOC-PROVIDER-PACKAGE-CLI-PACKAGE-PILOT-296X-001
```

The next row may add a stable CLI entry for existing-binary provider package
creation. It must preserve the no-load/no-activation boundary and keep the
Python helper as a compatibility tool rather than a second source of truth.

## Stop Line

This closeout does not compile `.hako` to a shared library, load provider
binaries, resolve exports, read descriptors, bind provider APIs, call
allocator entrypoints, activate providers, replace the process allocator,
install hooks, use global allocator integration, or compute winners.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_existing_binary_manifest_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
