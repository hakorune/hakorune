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

## Phase B Selection

Phase B is a selected-provider-binary build/package lane. It is not arbitrary
shell execution and not full `.hako` to shared-library generation.

The accepted Phase B shape is:

```text
repo-selected provider source or build fixture
  -> explicit Hakorune-owned build step
  -> shared-library artifact
  -> manifest v1 package
  -> metadata preflight
```

Phase B build rows must reuse manifest v1 and keep package build no-load:

```text
package_mode=selected-binary-build-package
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

## Phase C Selection

Phase C starts as a staged `.hako`-derived provider package lane. The first
accepted shape is deliberately narrower than arbitrary `.hako` allocator
semantic lowering:

```text
selected .hako provider fixture
  -> source existence + .hako extension preflight
  -> MIR JSON emission preflight
  -> source_hash + mir_json_hash included in package contract/build metadata
  -> Hakorune-owned provider ABI wrapper artifact
  -> manifest v1 package
  -> metadata preflight
```

The accepted Phase C0/C1 vocabulary is:

```text
package_mode=hako-derived-provider-package
build_mode=hako-derived-selected-fixture
hako_source_checked=1
hako_mir_json_emitted=1
hako_semantic_provider_codegen=0
shared_library_load_executed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```

`hako_semantic_provider_codegen=0` is the stop line for the first `.hako`
package row. It means the package is tied to a real `.hako` source and emitted
MIR artifact, while native provider entrypoints may still be provided by a
Hakorune-owned ABI wrapper until a later row opens semantic provider codegen.

## Phase C Semantic Codegen Step 1

The first accepted semantic provider-codegen mode is `ping-literal-v0`.

```text
.hako static box HakoProvider {
  ping() { return <i64 literal> }
}
  -> MIR JSON function HakoProvider.ping/0
  -> const i64 return literal extraction
  -> generated provider hako_ping() returns that literal
  -> provider noop-call smoke observes the same value
```

This mode may report:

```text
hako_semantic_provider_codegen=ping-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=<i64>
provider_noop_call_result=<same i64>
```

It does not open allocator entrypoint semantic codegen. `alloc`, `free`,
`owns`, `realloc`, aligned allocation, process replacement, hooks, global
allocator integration, and winner claims remain separate future rows.

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
