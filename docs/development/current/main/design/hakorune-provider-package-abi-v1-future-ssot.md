---
Status: SSOT
Decision: accepted
Date: 2026-05-21
Scope: future Hakorune provider package / DLL shared-library ABI plan.
Related:
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/design/provider-runtime-load-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-purpose-ssot.md
  - docs/development/current/main/design/hako-alloc-optional-process-allocator-replacement-proposal-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-1073-MIMAP-451A-ALLOCATOR-COMPARISON-C-MIMALLOC-EXPLICIT-RUNNER-EXECUTION-PILOT.md
---

# Hakorune Provider Package ABI v1 (Future SSOT)

## Current Split

This historical future SSOT is retained as a compatibility pointer. The active
provider-package design is now split by responsibility:

```text
docs/development/current/main/design/provider-abi-v1-ssot.md
docs/development/current/main/design/provider-package-artifact-ssot.md
docs/development/current/main/design/provider-runtime-load-ssot.md
```

Use those three SSOTs for new Phase-296x rows.

## Decision

Hakorune shared-library generation is a future provider-package backend, not a
requirement for the current C mimalloc comparison lane.

Current MIMAP-451A remains an explicit external-runner row:

```text
C mimalloc executable runner
stable output contract
memory-use evidence
representative hako_alloc workload comparison
```

It must not grow DLL/shared-library loading, plugin packaging, process allocator
replacement, hook installation, or `#[global_allocator]` behavior.

The future shared-library feature is defined as:

```text
Hakorune provider package =
  shared library binary
  stable C ABI descriptor
  function table
  manifest
  binary hash
  host-side preflight
  speed/diagnostic contract
```

Do not frame this as "just emit a DLL". The exported binary is one artifact in a
provider package.

## Package Shape

Windows package example:

```text
dist/hakorune-provider/
  hakorune_provider.dll
  hakorune_provider.pdb        # optional
  hakorune_provider.lib        # optional import library
  hakorune_provider.h
  hakorune_provider.json
  hakorune_provider.sha256
```

Unix variants use the same ABI contract with `.so` / `.dylib` as later backend
artifacts.

The package manifest records the binary and contract before the host loads it:

```json
{
  "provider_name": "hakorune-mimalloc-exp",
  "abi": "hakorune-provider-v1",
  "target": "x86_64-pc-windows-msvc",
  "profile": "speed",
  "binary": "hakorune_provider.dll",
  "binary_sha256": "...",
  "contract_hash": "...",
  "features": {
    "diagnostic_stats": false,
    "timing_stats": false,
    "speed_lane": true
  },
  "exports": [
    "hakorune_provider_get_api_v1"
  ]
}
```

## Single Export

Provider packages should avoid growing many exported symbols. The default ABI
exports one descriptor entry:

```c
__declspec(dllexport)
const HkrProviderApiV1* hakorune_provider_get_api_v1(void);
```

The returned table owns both the descriptor and the allocator entrypoints:

```c
#define HKR_PROVIDER_MAGIC 0x484B5250u  /* "HKRP" */
#define HKR_PROVIDER_ABI_V1 1u

typedef struct HkrProviderDescriptorV1 {
    uint32_t magic;
    uint16_t abi_version;
    uint16_t struct_size;

    uint32_t provider_kind;
    uint32_t feature_flags;

    char provider_name[64];
    char build_id[64];
    char source_commit[48];
    char contract_hash[64];

    uint64_t speed_forbidden_mask;
    uint64_t active_feature_mask;
} HkrProviderDescriptorV1;

typedef struct HkrProviderApiV1 {
    HkrProviderDescriptorV1 desc;

    void* (*create)(const char* config_json, size_t config_len);
    void  (*destroy)(void* ctx);

    void* (*alloc)(void* ctx, size_t size, size_t align);
    void  (*free)(void* ctx, void* ptr);
    int   (*owns)(void* ctx, void* ptr);

    int   (*stats_snapshot)(void* ctx, void* out, size_t out_len);
    int   (*reset_stats)(void* ctx);
} HkrProviderApiV1;
```

Minimum v1 providers may implement only `create`, `destroy`, `alloc`, `free`,
and `owns` first. Diagnostic and stats rows must keep speed-lane contamination
out of speed packages.

## ABI Layers

Keep two ABI layers separate:

```text
provider ABI
  shared-library entrypoint
  descriptor
  manifest / hash / contract preflight
  function table discovery

allocator ABI
  create / destroy
  alloc / free
  owns
  stats / reset
```

The provider ABI answers "what package was loaded and what contract does it
claim?" The allocator ABI answers "how does this provider allocate and release
memory?"

## Loader Contract

Windows loader rows must be conservative:

```text
LoadLibraryExW(full_path, NULL, LOAD_LIBRARY_SEARCH_DLL_LOAD_DIR)
GetProcAddress("hakorune_provider_get_api_v1")
descriptor.magic/version/struct_size check
manifest binary_sha256 check
descriptor contract_hash check
speed/diagnostic profile check
```

Do not rely on ambient DLL search path behavior. The host must load by full path
or package-directory-relative path.

## Ownership / Free Contract

Provider and host heap ownership must not be mixed:

```text
provider pointer
  provider.owns(ctx, ptr) == true
  release only with provider.free(ctx, ptr)

host pointer
  provider.owns(ctx, ptr) == false
  provider must not free it
```

Forbidden:

```text
free miss -> lazy-load provider
host CRT free(provider pointer)
provider free(host pointer)
provider pointer crossing into implicit host replacement
```

This mirrors the allocator lane's no-hidden-crossing rule: ownership miss is a
diagnostic boundary, not a dynamic activation trigger.

## Speed / Diagnostic Profiles

Provider binaries must declare whether they are speed or diagnostic packages.

```text
speed profile
  diagnostic counters off
  timing samples off
  diagnostic storage off
  descriptor lane_kind = speed

diagnostic profile
  counters allowed
  timing allowed only as diagnostic evidence
  benchmark ranking forbidden
```

A speed benchmark must fail fast if the manifest or descriptor says the package
is diagnostic, or if `speed_forbidden_mask` has active diagnostic bits.

## Future Row Schedule

These rows are parked. They do not gate MIMAP-451A.

| Future token | Purpose | Opens |
| --- | --- | --- |
| PROVIDER-PKG-000 | This SSOT: define package/ABI/loader stop lines. | no code |
| PROVIDER-PKG-001 | Descriptor/manifest schema fixture and host preflight contract. | no allocator execution |
| PROVIDER-PKG-002 | Descriptor-only toy DLL/shared-library loader smoke. | loader only |
| PROVIDER-PKG-003 | Toy provider function table with `create/destroy/alloc/free/owns`. | explicit provider allocation only |
| PROVIDER-PKG-004 | Hakorune-generated provider package backend pilot. | generated provider artifact |
| PROVIDER-PKG-005 | Full package artifacts: header, manifest, hash, optional import library/debug symbols. | packaging |
| PROVIDER-PKG-006 | Optional host replacement / global allocator integration proposal. | still requires separate approval |

Provider host replacement is not part of v1 package bringup. It remains parked
behind the optional process allocator replacement SSOT.

## Stop Lines

Until a future provider-package row explicitly opens them, the following remain
closed:

- process allocator replacement;
- hook installation;
- `#[global_allocator]`;
- implicit provider discovery;
- hidden env activation;
- backend owner-name or app-name matcher additions;
- broad exported-symbol surface;
- `DllMain` allocator initialization;
- `DllMain` `LoadLibrary` / heavy heap work;
- cross-CRT free;
- free-miss lazy loading;
- mixing speed and diagnostic profiles in one benchmark lane.

## MIMAP-451A Boundary

MIMAP-451A should continue to build the C mimalloc explicit runner execution
pilot. It may reference this SSOT only to say that DLL/provider-package
generation is deliberately parked.
