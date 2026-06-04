---
Status: SSOT
Decision: accepted
Date: 2026-05-27
Scope: common Hakorune provider ABI v1 vocabulary shared by package artifacts and runtime loading.
Related:
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/provider-abi-shim-boundary-ssot.md
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
The field order below is the ABI layout SSOT for generated C providers,
Python `ctypes` bind smokes, and LD_PRELOAD shim probes. Optional tail fields
are detected by `api_table_size`; new fields may only append at the end.

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

  /* optional tail: provider-owned lifecycle claims */
  int (*free_claim)(void* ptr);
  int (*usable_size_claim)(void* ptr, size_t* out_size);
  int (*realloc_claim)(void* ptr, size_t new_size, void** out_ptr);
};
```

ABI field order:

```text
0 magic
1 abi_major
2 abi_minor
3 api_table_size
4 ping
5 alloc
6 free
7 owns
8 free_claim
9 usable_size_claim
10 realloc_claim
```

Tail bind semantics:

```text
bound=1:
  the function pointer exists in this provider API table and is non-null

enabled=1:
  this provider route owns the truth needed to use the operation as mainline

bound=1 enabled=0:
  compatibility tail exists, but the route must return not_owned / disabled
  semantics until the required truth source exists
```

Current allocator-provider route examples:

```text
host_backed_adapter:
  free_claim bound=1 enabled=1 for provider-allocated wrapper pointers
  usable_size_claim bound=1 enabled=0 until HostAllocatorV0
  realloc_claim bound=1 enabled=0 until HostAllocatorV0

pure_allocator / native-slot:
  free_claim bound=1 enabled=1
  usable_size_claim bound=1 enabled=1
  realloc_claim bound=1 enabled=1
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

`owns(ptr)` is compatibility / diagnostic ownership observation. The
replacement-shim mainline must prefer claim operations once the relevant route
reports them as enabled. Claim operation semantics are defined in:

```text
docs/development/current/main/design/provider-abi-shim-boundary-ssot.md
```

## Always Closed In v0

The following are closed until a later decision row opens them:

```text
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
winner_claim=0
```
