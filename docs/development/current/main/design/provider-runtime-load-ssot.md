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

## Stage 5: Provider No-op Call Smoke

No-op call smoke may call a safe provider function such as `ping`. It must not
call allocator entrypoints or activate the provider.

```text
dll_mode=provider-noop-call
provider_api_bound=1
provider_call_executed=1
provider_noop_call_executed=1
allocator_entrypoint_called=0
provider_active=0
replacement_active=0
```

## Later Stages

## Stage 6: Explicit Alloc/Free Smoke

Explicit alloc/free smoke may call provider allocator entrypoints through the
API table. It is not process allocator replacement.

```text
dll_mode=provider-alloc-free
provider_api_bound=1
provider_call_executed=1
allocator_entrypoint_called=1
provider_alloc_executed=1
provider_free_executed=1
provider_active=0
replacement_active=0
```

## Later Stages

Repeated measurement through explicit provider calls is separate from the first
alloc/free smoke. Provider activation is separate from explicit provider calls.
Process allocator replacement, hooks, and global allocator integration are a
future lane.

## Stage 7A: Provider-Backed LD_PRELOAD Pilot

Decision: accepted as a narrow pilot, not as product allocator replacement.

This stage may build a local `LD_PRELOAD` malloc-family shim that binds the
manifest-selected provider API and routes a controlled smoke process through
provider `alloc` / `free`.

Allowed:

```text
manifest preflight
provider API bind
LD_PRELOAD env for a generated smoke process
malloc / calloc / realloc / free shim exports
provider alloc/free calls through the API table
shim-local pointer-size table for pilot realloc correctness
```

Still closed:

```text
winner claim
Rust #[global_allocator]
system allocator default
production hook install
ambient provider discovery
unbounded external workload replacement claim
```

The report must distinguish the pilot from closed product integration:

```text
dll_mode=provider-backed-ldpreload-pilot
ld_preload_env_applied=1
provider_api_bound=1
provider_call_executed=1
allocator_entrypoint_called=1
replacement_active=1
replacement_scope=generated-smoke-process-only
hook_installed=0
global_allocator=0
winner_claim=0
```

This pilot exists to prove that a `.hako`-derived provider package can be
reached from a malloc-family replacement seam. It does not claim C parity,
process-wide production replacement, or global allocator readiness.

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
