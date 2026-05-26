---
Status: SSOT
Decision: accepted
Date: 2026-05-27
Scope: Hakorune provider runtime load ladder and fail-fast boundaries.
Related:
  - docs/development/current/main/design/provider-abi-v1-ssot.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
---

# Provider Runtime Load

## Decision

Runtime loading is a staged ladder. Each row opens exactly one boundary.

```text
metadata-preflight
shared-library-load-only-smoke
descriptor-read-smoke
provider-api-bind-smoke
explicit-provider-call-smoke
activation decision
replacement / hook / global allocator lane
```

## Stage 1: Metadata Preflight

```text
dll_mode=metadata-preflight
shared_library_load_executed=0
required_export_resolved=0
descriptor_read_executed=0
provider_call_executed=0
provider_active=0
replacement_active=0
```

## Stage 2: Shared-Library Load-Only Smoke

The host loads the manifest-selected binary and stops. It must not resolve
exports, read descriptors, bind APIs, call providers, or touch allocator
entrypoints.

```text
dll_mode=load-only
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
```

On Unix this corresponds to `dlopen`. On Windows it corresponds to
`LoadLibrary` / `LoadLibraryEx` by an explicit manifest-selected path. Ambient
search-path discovery is forbidden.

## Stage 3: Descriptor-Read Smoke

Descriptor-read smoke may resolve and call the descriptor export only:

```text
dll_mode=descriptor-smoke
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_call_executed=0
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
```

The descriptor export must be side-effect-free for provider activation:

```text
no provider activation
no allocator replacement
no hook installation
no background thread
no allocator entrypoint calls
```

## Later Stages

## Stage 4: Provider API Bind Smoke

API bind smoke may resolve and call `hakorune_provider_get_api_v1` to obtain
the API table shape. It must not call `ping`, allocator entrypoints, or any
other function pointer in the returned table.

```text
dll_mode=provider-api-bind
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_api_bound=1
provider_call_executed=0
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
```

## Later Stages

Explicit provider calls are separate from API bind. Provider activation is
separate from explicit provider calls. Process allocator replacement, hooks,
and global allocator integration are a future lane.

## Fail-Fast Rules

```text
ABI major mismatch -> fail-fast
descriptor size too small -> fail-fast
contract hash mismatch -> fail-fast
required export missing -> fail-fast in descriptor-read row
capability requested but not declared -> fail-fast
manifest activation policy false -> do not activate
```

Provider package presence alone never activates a provider.
