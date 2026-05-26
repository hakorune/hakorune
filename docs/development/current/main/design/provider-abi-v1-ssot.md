---
Status: SSOT
Decision: accepted
Date: 2026-05-27
Scope: common Hakorune provider ABI v1 vocabulary shared by package artifacts and runtime loading.
Related:
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md
---

# Provider ABI v1

## Decision

Hakorune provider packaging is a formal lane. The v0 formal range is package
metadata, manifest preflight, and shared-library load smoke. Provider API calls,
provider activation, process allocator replacement, hooks, and
`#[global_allocator]` remain separate later decisions.

The provider ABI is not "emit a DLL". A DLL / `.so` / `.dylib` is one artifact
inside a provider package.

## Layers

Keep these contracts separate:

```text
provider package artifact
  manifest
  binary artifact
  binary hash
  contract hash
  preflight report

runtime load
  metadata preflight
  shared-library-load-only smoke
  descriptor-read smoke
  API bind smoke
  explicit provider call
  activation decision

allocator replacement
  separate future lane
```

## Manifest Fields

The package manifest must carry enough information for no-load preflight:

```text
schema_version
package_id
provider_kind
provider_name
provider_version
abi_version
target_triple
platform
artifact.path
artifact.sha256
artifact.size_bytes
contract_hash
required_exports
capabilities
activation.provider_call_allowed
activation.allocator_replacement_allowed
activation.hook_allowed
activation.global_allocator_allowed
```

`provider_kind` is generic. v1 may define `allocator`, `runtime`,
`diagnostic`, and `tooling`, but the current phase only exercises
`allocator`.

## Required Exports

The first descriptor-read row uses one descriptor export:

```text
hakorune_provider_descriptor_v1
```

The provider API bind row may later use:

```text
hakorune_provider_get_api_v1
```

The load-only smoke must not resolve either symbol.

## Provider API Table Shape

API bind smoke may resolve and call `hakorune_provider_get_api_v1` to obtain
the table shape. It must not call any function pointer in the returned table.

```c
struct HakoProviderApiV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t api_table_size;

  int (*ping)(void);
  void* (*alloc)(size_t size, size_t align);
  void (*free)(void* ptr);
  int (*owns)(void* ptr);
};
```

API constants:

```text
api_magic=0x484B5241
api_abi_major=1
```

## Binary Descriptor Shape

Descriptor-read smoke reads identity and capability metadata only. It does not
return the allocator function table.

```c
struct HakoProviderDescriptorV1 {
  uint32_t magic;
  uint16_t abi_major;
  uint16_t abi_minor;
  uint32_t descriptor_size;

  const char* provider_id;
  const char* provider_kind;
  const char* provider_version;

  uint64_t capability_bits;
  uint64_t safety_flags;

  const char* contract_hash;
  const char* function_table_hash;

  uint32_t api_table_size;
  uint32_t reserved;
};
```

Descriptor constants:

```text
magic=0x484B5250
abi_major=1
```

## Contract Hash

`contract_hash` identifies normalized ABI/contract data, not the binary bytes.

It includes:

```text
abi_version
provider_kind
capabilities
required_exports
descriptor schema version
API table schema version
activation policy
memory ownership policy
```

It excludes:

```text
binary path
build timestamp
absolute host path
loader search path
```

`binary_sha256` / `artifact.sha256` identifies artifact bytes.

## Ownership Boundary

Allocator providers must not mix host and provider heap ownership:

```text
provider-owned pointer -> provider free only
host-owned pointer -> host free only
provider free miss -> fail-fast or explicit error
```

An allocator provider API must eventually expose `owns(ptr)` before replacement
can be considered.

## Always Closed In v0

The following are closed until a later decision row opens them:

```text
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```
