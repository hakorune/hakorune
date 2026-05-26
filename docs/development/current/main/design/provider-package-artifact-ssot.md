---
Status: SSOT
Decision: accepted
Date: 2026-05-27
Scope: Hakorune provider package artifact contract and manifest layout.
Related:
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
---

# Provider Package Artifact

## Decision

The formal product is a provider package artifact, not a bare DLL. v0 may
package an existing shared library plus manifest metadata. Full Hakorune
generation of `.so` / `.dll` / `.dylib` is a later build row.

## Artifact Layout

```text
dist/hakorune-provider/
  hakorune_provider.json
  libhakorune_provider.so | hakorune_provider.dll | libhakorune_provider.dylib
  hakorune_provider.sha256
  hakorune_provider.h        # later
  hakorune_provider.lib      # optional Windows import library later
  hakorune_provider.pdb      # optional Windows debug artifact later
```

## Manifest v1

```json
{
  "schema_version": "hakorune-provider-package-v1",
  "package_id": "org.hakorune.provider.mimalloc.v0",
  "provider_kind": "allocator",
  "provider_name": "mimalloc-explicit",
  "provider_version": "0.1.0",
  "abi_version": "hakorune-provider-abi-v1",
  "target_triple": "x86_64-unknown-linux-gnu",
  "platform": "linux",
  "artifact": {
    "path": "libhakorune_provider.so",
    "sha256": "...",
    "size_bytes": 12345
  },
  "contract_hash": "...",
  "required_exports": [
    "hakorune_provider_descriptor_v1"
  ],
  "capabilities": [
    "descriptor",
    "explicit_allocator_api"
  ],
  "activation": {
    "provider_call_allowed": false,
    "allocator_replacement_allowed": false,
    "hook_allowed": false,
    "global_allocator_allowed": false
  }
}
```

## Build Command Boundary

Final CLI shape may become:

```bash
hakorune provider build --profile speed
hakorune provider preflight hakorune_provider.json
hakorune provider load-smoke hakorune_provider.json
hakorune provider descriptor-smoke hakorune_provider.json
```

v0 should not promise full cross-platform shared-library generation. The safe
sequence is:

```text
Phase A: package existing binary + manifest
Phase B: build selected provider binary
Phase C: build full .hako-derived provider package
```

## Preflight Requirements

Metadata preflight reads only manifest and filesystem metadata:

```text
dll_mode=metadata-preflight
manifest_ready=1
binary_hash_ready=1
descriptor_ready=0
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

No hidden discovery is allowed. The host must use the manifest artifact path,
resolved relative to the manifest directory unless an explicit absolute path is
provided.
