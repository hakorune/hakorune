---
Status: Landed
Date: 2026-05-27
Scope: close the first .hako semantic allocator-entrypoint provider-codegen pilot.
Blocker: MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-CLOSEOUT-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-36-MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-PILOT.md
  - docs/development/current/main/design/provider-package-artifact-ssot.md
  - docs/reference/runtime/provider-package-v0.md
  - tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_alloc_free_closeout_guard.sh
---

# 296x-37 Provider Package .hako Semantic Alloc/Free Closeout

## Decision

Close:

```text
MIMALLOC-PROVIDER-PACKAGE-HAKO-SEMANTIC-ALLOC-FREE-CLOSEOUT-296X-001
```

The `alloc-free-owns-literal-v0` semantic provider-codegen mode is accepted
across the staged provider package ladder:

```text
.hako source + MIR JSON
  -> generated shared-library provider artifact
  -> metadata preflight
  -> descriptor-read smoke
  -> API-bind smoke
  -> no-op provider call smoke
  -> explicit alloc/free provider smoke
```

The semantic values remain visible at the runtime smoke boundaries:

```text
HakoProvider.ping/0 -> 7
HakoProvider.ownsAllocated/0 -> 1
provider_noop_call_result=7
provider_owns_result=1
```

## Evidence Contract

The closeout row requires all of these contracts over the same generated
package manifest:

```text
output_contract=hakorune-provider-package-hako-derived-build-v0
hako_semantic_provider_codegen=alloc-free-owns-literal-v0
hako_provider_ping_codegen=1
hako_provider_ping_value=7
hako_provider_owns_codegen=1
hako_provider_owns_value=1
shared_library_artifact_generated=1
provider_call_executed=0
summary=ok
```

```text
output_contract=hakorune-provider-package-metadata-preflight-v0
dll_mode=metadata-preflight
shared_library_load_executed=0
descriptor_ready=0
provider_active=0
replacement_active=0
summary=ok
```

```text
output_contract=hakorune-provider-package-descriptor-smoke-v0
dll_mode=descriptor-smoke
shared_library_load_executed=1
required_export_resolved=1
descriptor_read_executed=1
provider_call_executed=0
allocator_entrypoint_called=0
summary=ok
```

```text
output_contract=hakorune-provider-package-api-bind-smoke-v0
dll_mode=provider-api-bind
provider_api_bound=1
provider_call_executed=0
allocator_entrypoint_called=0
summary=ok
```

```text
output_contract=hakorune-provider-package-noop-call-smoke-v0
dll_mode=provider-noop-call
provider_call_executed=1
provider_noop_call_result=7
allocator_entrypoint_called=0
summary=ok
```

```text
output_contract=hakorune-provider-package-alloc-free-smoke-v0
dll_mode=provider-alloc-free
provider_call_executed=1
allocator_entrypoint_called=1
provider_alloc_executed=1
provider_free_executed=1
provider_owns_result=1
allocated_pointer_nonzero=1
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
MIMALLOC-PROVIDER-PACKAGE-HAKO-DERIVED-FUNCTIONAL-CLOSEOUT-296X-001
```

The next row should close the `.hako`-derived provider package lane as a
functional v0 package artifact: selected `.hako` source, MIR hash metadata,
generated shared library, descriptor/API surface, semantic ping, and explicit
allocator smoke are all proven. Native pointer allocation mechanics,
activation, replacement, hooks, globals, and winner claims remain separate
future lanes.

## Stop Line

This closeout proves explicit provider allocator entrypoint smoke plus `.hako`
ownership policy lowering. It does not make `.hako` responsible for native
pointer allocation/free mechanics, activate providers, replace allocators,
install hooks, use global allocator integration, or make benchmark winner
claims.

## Verification

```bash
bash tools/checks/k2_wide_phase296x_mimalloc_provider_package_hako_semantic_alloc_free_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
